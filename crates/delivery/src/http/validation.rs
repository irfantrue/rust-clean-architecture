use super::{error::ApiError, problem::FieldError};
use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::{Validate, ValidationErrors};

impl From<ValidationErrors> for ApiError {
    fn from(errs: ValidationErrors) -> Self {
        let mut list: Vec<FieldError> = errs
            .field_errors()
            .iter()
            .flat_map(|(field, es)| {
                es.iter().map(move |e| FieldError {
                    field: field.to_string(),
                    message: e
                        .message
                        .as_ref()
                        .map(|m| m.to_string())
                        .unwrap_or_else(|| e.code.to_string()),
                })
            })
            .collect();

        list.sort_by(|a, b| a.field.cmp(&b.field)); // HashMap tidak berurutan
        ApiError::UnprocessableEntity(list)
    }
}

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|_| ApiError::BadRequest)?;

        value.validate()?; // ValidationErrors -> AppError lewat From
        Ok(Self(value))
    }
}
