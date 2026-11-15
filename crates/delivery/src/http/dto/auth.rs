use application::usecase::check_email_availability::CheckEmailAvailabilityCommand;
use domain::error::Email;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::http::{
    error::{ApiError, email_message},
    problem::FieldError,
};

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CheckEmailAvailabilityRequest {
    #[validate(
        email(message = "Format email tidak valid"),
        length(max = 254, message = "Email terlalu panjang")
    )]
    email: String,
}

impl TryFrom<CheckEmailAvailabilityRequest> for CheckEmailAvailabilityCommand {
    type Error = ApiError;

    fn try_from(req: CheckEmailAvailabilityRequest) -> Result<Self, Self::Error> {
        let email = Email::parse(&req.email).map_err(|e| {
            ApiError::UnprocessableEntity(vec![FieldError {
                field: "email".into(),
                message: email_message(e).into(),
            }])
        })?;

        Ok(Self { email })
    }
}

#[derive(Debug, Serialize)]
pub struct CheckEmailAvailabilityResponse {
    available: bool,
}

impl From<bool> for CheckEmailAvailabilityResponse {
    fn from(available: bool) -> Self {
        Self { available }
    }
}
