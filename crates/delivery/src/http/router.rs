use crate::{
    http::{handler, problem::Problem},
    state::ApplicationState,
};
use axum::{
    Router,
    http::{Method, Request, StatusCode, header},
    response::IntoResponse,
    routing::post,
};
use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    sensitive_headers::SetSensitiveRequestHeadersLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};

pub fn init(state: ApplicationState) -> Router {
    let middleware = ServiceBuilder::new()
        .layer(SetSensitiveRequestHeadersLayer::new([header::AUTHORIZATION, header::COOKIE]))
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(TraceLayer::new_for_http().make_span_with(|req: &Request<_>| {
            let id = req
                .headers()
                .get("x-request-id")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("-");
            tracing::info_span!("http", method = %req.method(), path = %req.uri().path(), request_id = %id)
        }))
        .layer(PropagateRequestIdLayer::x_request_id())
        .layer(CatchPanicLayer::custom(|_| {
            Problem::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response()
        }))
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)))
        .layer(RequestBodyLimitLayer::new(1024 * 1024)) // 1 MB
        .layer(CompressionLayer::new())
        .layer(CorsLayer::new().allow_methods([Method::GET, Method::POST]).allow_origin(Any));

    Router::new()
        .nest("/api/v1/auth", router_api().layer(middleware))
        .fallback(handler::fallback::not_found)
        .with_state(state)
}

fn router_api() -> Router<ApplicationState> {
    Router::new().route(
        "/check-email-availability",
        post(handler::auth::check_email_availability),
    )
    // .route("/register", post(handler::auth::register))
    // .route("/login", post(handler::auth::login))
    // .route("/refresh", post(handler::auth::refresh))
    // .route("/logout", post(handler::auth::logout))
}
