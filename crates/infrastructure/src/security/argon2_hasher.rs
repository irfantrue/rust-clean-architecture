use application::{
    error::CredentialHasherError,
    port::{CredentialHasher, CredentialHasherResult},
};
use argon2::{
    Argon2, PasswordHash,
    password_hash::{PasswordHasher, PasswordVerifier},
};
use async_trait::async_trait;
use secrecy::{ExposeSecret, SecretString};
use std::sync::Arc;
use zeroize::Zeroizing;

#[derive(Default)]
pub struct Argon2PasswordHasher {
    hasher: Arc<Argon2<'static>>,
}

#[async_trait]
impl CredentialHasher for Argon2PasswordHasher {
    async fn hash(&self, password: &SecretString) -> CredentialHasherResult<SecretString> {
        let hasher = self.hasher.clone();
        let plaintext = Zeroizing::new(password.expose_secret().to_owned());

        let hash = tokio::task::spawn_blocking(move || {
            hasher
                .hash_password(plaintext.as_bytes())
                .map(|h| h.to_string())
        })
        .await
        .map_err(|e| CredentialHasherError::Internal(e.into()))? // JoinError
        .map_err(|e| CredentialHasherError::Internal(anyhow::anyhow!("argon2 hash: {e}")))?;

        Ok(SecretString::from(hash))
    }

    async fn verify(
        &self,
        password: &SecretString,
        password_hash: &SecretString,
    ) -> CredentialHasherResult<bool> {
        let hasher = self.hasher.clone();
        let plaintext = Zeroizing::new(password.expose_secret().to_owned());
        let stored_hash = password_hash.expose_secret().to_owned();

        tokio::task::spawn_blocking(move || -> CredentialHasherResult<bool> {
            let parsed = PasswordHash::new(&stored_hash).map_err(|e| {
                CredentialHasherError::Internal(anyhow::anyhow!("invalid stored hash: {e}"))
            })?;

            match hasher.verify_password(plaintext.as_bytes(), &parsed) {
                Ok(()) => Ok(true),
                Err(argon2::password_hash::Error::PasswordInvalid) => Ok(false),
                Err(e) => Err(CredentialHasherError::Internal(anyhow::anyhow!(
                    "argon2 verify: {e}"
                ))),
            }
        })
        .await
        .map_err(|e| CredentialHasherError::Internal(e.into()))?
    }
}
