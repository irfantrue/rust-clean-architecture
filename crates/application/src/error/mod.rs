mod check_email_availablity;
mod credential_hasher;
mod token_service;

pub use check_email_availablity::CheckEmailAvailabilityError;
pub use credential_hasher::*;
pub use token_service::TokenServiceError;

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("entity not found")]
    NotFound,

    #[error("conflict on constraint `{0}`")]
    Conflict(String),

    #[error("storage failure")]
    Storage(#[source] anyhow::Error),
}
