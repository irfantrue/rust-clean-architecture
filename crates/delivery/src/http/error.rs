use application::error::{CheckEmailAvailabilityError, TokenServiceError};
use axum::{
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use domain::error::EmailError;

use crate::http::problem::{FieldError, Problem};

#[derive(Debug)]
pub enum ApiError {
    BadRequest,
    UnprocessableEntity(Vec<FieldError>),
    TokenExpired,
    Unauthenticated,
    Forbidden,
    EmailTaken,
    Internal(anyhow::Error),
}

impl From<TokenServiceError> for ApiError {
    fn from(e: TokenServiceError) -> Self {
        match e {
            TokenServiceError::Expired => ApiError::TokenExpired, // 401, type token-expired
            TokenServiceError::Invalid => ApiError::Unauthenticated, // 401
            TokenServiceError::Internal(err) => ApiError::Internal(err), // 500
        }
    }
}

pub fn email_message(e: EmailError) -> &'static str {
    match e {
        EmailError::Empty => "Email wajib diisi",
        EmailError::InvalidFormat => "Format email tidak valid",
    }
}

impl From<CheckEmailAvailabilityError> for ApiError {
    fn from(e: CheckEmailAvailabilityError) -> Self {
        match e {
            CheckEmailAvailabilityError::Storage(err) => ApiError::Internal(err),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::BadRequest => Problem::new(StatusCode::BAD_REQUEST, "Malformed request")
                .type_("https://api.example.com/errors/malformed-request")
                .detail("Body bukan JSON yang valid.")
                .into_response(),
            ApiError::UnprocessableEntity(errors) => {
                Problem::new(StatusCode::UNPROCESSABLE_ENTITY, "Validation failed")
                    .type_("https://api.example.com/errors/validation-failed")
                    .detail("Beberapa field tidak valid.")
                    .errors(errors)
                    .into_response()
            }
            ApiError::Unauthenticated => unauthorized(
                Problem::new(StatusCode::UNAUTHORIZED, "Authentication required")
                    .type_("https://api.example.com/errors/unauthenticated")
                    .detail("Autentikasi diperlukan."),
            ),
            ApiError::TokenExpired => unauthorized(
                Problem::new(StatusCode::UNAUTHORIZED, "Token expired")
                    .type_("https://api.example.com/errors/token-expired")
                    .detail("Token sudah kedaluwarsa."),
            ),
            ApiError::Forbidden => Problem::new(StatusCode::FORBIDDEN, "Forbidden")
                .detail("Kamu tidak punya izin untuk melakukan aksi ini.")
                .into_response(),
            ApiError::EmailTaken => Problem::new(StatusCode::CONFLICT, "Email already registered")
                .type_("https://api.example.com/errors/email-taken")
                .detail("Email sudah terdaftar.")
                .into_response(),
            ApiError::Internal(err) => {
                let trace_id = uuid::Uuid::new_v4().to_string();
                tracing::error!(%trace_id, error = ?err, "internal server error");
                Problem::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error")
                    .detail("Terjadi kesalahan pada server.")
                    .trace_id(trace_id)
                    .into_response()
            }
        }
    }
}

fn unauthorized(problem: Problem) -> Response {
    let mut res = problem.into_response();
    res.headers_mut()
        .insert(header::WWW_AUTHENTICATE, HeaderValue::from_static("Bearer"));
    res
}
