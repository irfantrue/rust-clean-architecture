use crate::error::CredentialHasherError;
use async_trait::async_trait;
use secrecy::SecretString;

pub type CredentialHasherResult<T> = std::result::Result<T, CredentialHasherError>;

#[async_trait]
pub trait CredentialHasher: Send + Sync {
    async fn hash(&self, password: &SecretString) -> CredentialHasherResult<SecretString>;
    async fn verify(
        &self,
        password: &SecretString,
        hash: &SecretString,
    ) -> CredentialHasherResult<bool>;
}
