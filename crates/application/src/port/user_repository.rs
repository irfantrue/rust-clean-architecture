use crate::error::RepositoryError;
use async_trait::async_trait;
use domain::{entities::user::User, error::Email};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_email(&self, email: &Email) -> Result<User, RepositoryError>;
    async fn exists_by_email(&self, email: &Email) -> Result<bool, RepositoryError>;
}
