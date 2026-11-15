use application::usecase::check_email_availability::CheckEmailAvailabilityUseCase;
use axum::extract::FromRef;
use std::sync::Arc;

#[derive(Clone)]
pub struct ApplicationState {
    pub check_email_availability: Arc<CheckEmailAvailabilityUseCase>,
}

impl FromRef<ApplicationState> for Arc<CheckEmailAvailabilityUseCase> {
    fn from_ref(app_state: &ApplicationState) -> Self {
        app_state.check_email_availability.clone()
    }
}
