use crate::http::problem::Problem;
use axum::http::StatusCode;

pub async fn not_found() -> Problem {
    Problem::new(StatusCode::NOT_FOUND, "Not Found").detail(format!("endpoint tidak ditemukan."))
}
