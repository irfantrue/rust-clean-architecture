use application::usecase::check_email_availability::CheckEmailAvailabilityCommand;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct CheckEmailAvailabilityRequest {
    #[validate(
        email(message = "Format email tidak valid"),
        length(max = 254, message = "Email terlalu panjang")
    )]
    email: String,
}

impl From<CheckEmailAvailabilityRequest> for CheckEmailAvailabilityCommand {
    fn from(req: CheckEmailAvailabilityRequest) -> Self {
        Self { email: req.email }
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
