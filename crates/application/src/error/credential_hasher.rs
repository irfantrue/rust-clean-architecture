#[derive(Debug, thiserror::Error)]
pub enum CredentialHasherError {
    #[error("credential hasher failure")]
    Internal(#[source] anyhow::Error),
}
