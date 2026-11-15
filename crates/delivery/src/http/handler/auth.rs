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
    let available = state.check_email_availability.execute(req.into()).await?;
    Ok(ApiResponse::ok(CheckEmailAvailabilityResponse::from(
        available,
    )))
}
