use application::usecase::check_email_availability::CheckEmailAvailabilityCommand;
use axum::{extract::State, response::IntoResponse};
use tracing::instrument;

use crate::{
    http::{
        dto::auth::{CheckEmailAvailabilityRequest, CheckEmailAvailabilityResponse},
        error::ApiError,
        response::ApiResponse,
        validation::ValidatedJson,
    },
    state::ApplicationState,
};

#[instrument(skip(state, req))]
pub async fn check_email_availability(
    State(state): State<ApplicationState>,
    ValidatedJson(req): ValidatedJson<CheckEmailAvailabilityRequest>,
) -> Result<impl IntoResponse, ApiError> {
    let cmd = CheckEmailAvailabilityCommand::try_from(req)?;

    let available = state.check_email_availability.execute(cmd).await?;

    Ok(ApiResponse::ok(CheckEmailAvailabilityResponse::from(
        available,
    )))
}
