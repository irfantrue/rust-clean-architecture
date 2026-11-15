use application::{
    port::{CredentialHasher, TokenService, UserRepository},
    usecase::check_email_availability::CheckEmailAvailabilityUseCase,
};
use bootstrap::{config::Config, telemetry};
use delivery::state::ApplicationState;
use infrastructure::{
    postgres::PostgresRepository,
    security::{hasher::Argon2PasswordHasher, jwt::JwtTokenService},
};
use secrecy::ExposeSecret;
use sqlx::postgres::PgPoolOptions;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let conf = Config::from_env()?;
    let _guard = telemetry::init(&conf);

    tracing::info!("starting myapp");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(conf.database_url.expose_secret())
        .await?;
    tracing::info!("connected to database!");

    let repository = Arc::new(PostgresRepository::new(pool));
    let user_repository: Arc<dyn UserRepository> = repository.clone();

    let argon2_hasher = Arc::new(Argon2PasswordHasher::default());
    let _hasher: Arc<dyn CredentialHasher> = argon2_hasher.clone();
    let _token_service: Arc<dyn TokenService> = Arc::new(JwtTokenService::new(
        &conf.jwt_secret,
        conf.jwt_issuer,
        conf.jwt_ttl_secs,
    ));

    let state = ApplicationState {
        check_email_availability: Arc::new(CheckEmailAvailabilityUseCase::new(
            user_repository.clone(),
        )),
    };

    let router = delivery::http::router::init(state);

    let listener = tokio::net::TcpListener::bind(conf.bind_addr).await?;
    tracing::info!(addr = %listener.local_addr()?, "listening");
    axum::serve(listener, router).await?;

    Ok(())
}
