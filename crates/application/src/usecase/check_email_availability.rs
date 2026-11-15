use domain::error::Email;
use std::sync::Arc;
use tracing::instrument;

use crate::{error::CheckEmailAvailabilityError, port::UserRepository};

pub struct CheckEmailAvailabilityCommand {
    pub email: Email,
}

pub struct CheckEmailAvailabilityUseCase {
    user_repo: Arc<dyn UserRepository>,
}

type Result<T> = std::result::Result<T, CheckEmailAvailabilityError>;

impl CheckEmailAvailabilityUseCase {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }

    #[instrument(name = "check email availability", skip_all, fields(email = tracing::field::Empty))]
    pub async fn execute(&self, cmd: CheckEmailAvailabilityCommand) -> Result<bool> {
        let exists = self.user_repo.exists_by_email(&cmd.email).await?;

        Ok(exists)
    }
}
