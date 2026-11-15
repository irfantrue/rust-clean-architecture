use domain::error::EmailError;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,

    #[error("conflict on constraint `{0}`")]
    Conflict(String),

    #[error("storage failure")]
    Storage(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum TokenServiceError {
    #[error("token has expired")]
    Expired,

    #[error("token is invalid")]
    Invalid,

    #[error("token service failure")]
    Internal(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CredentialHasherError {
    #[error("credential hasher failure")]
    Internal(#[source] anyhow::Error),
}

#[derive(Debug, thiserror::Error)]
pub enum CheckEmailAvailabilityError {
    #[error(transparent)]
    InvalidEmail(#[from] EmailError),

    #[error("storage failure")]
    Storage(#[source] anyhow::Error),
}

impl From<RepositoryError> for CheckEmailAvailabilityError {
    fn from(e: RepositoryError) -> Self {
        match e {
            RepositoryError::Storage(err) => Self::Storage(err),
            other => Self::Storage(anyhow::anyhow!("unexpected repository error: {other}")),
        }
    }
}
