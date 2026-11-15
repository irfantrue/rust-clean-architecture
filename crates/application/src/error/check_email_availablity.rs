use super::RepositoryError;

#[derive(Debug, thiserror::Error)]
pub enum CheckEmailAvailabilityError {
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
