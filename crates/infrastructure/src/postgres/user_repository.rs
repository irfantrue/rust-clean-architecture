use application::{error::RepositoryError, port::UserRepository};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use domain::{entities::user::User, error::Email};
use secrecy::SecretString;
use uuid::Uuid;

use crate::postgres::PostgresRepository;

#[derive(sqlx::FromRow)]
struct UserRow {
    id: Uuid,
    email: String,
    password_hash: String,
    created_at: DateTime<Utc>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            email: Email::from_trusted(row.email),
            password_hash: SecretString::from(row.password_hash),
            created_at: row.created_at,
        }
    }
}

const USER_BY_EMAIL: &str = "
SELECT id, email, password_hash, created_at \
FROM users WHERE email = $1";

const EMAIL_EXISTS: &str = "SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)";

#[async_trait]
impl UserRepository for PostgresRepository {
    async fn find_by_email(&self, email: &Email) -> Result<User, RepositoryError> {
        let row = sqlx::query_as::<_, UserRow>(USER_BY_EMAIL)
            .bind(email.as_str())
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(User::from(row))
    }

    async fn exists_by_email(&self, email: &Email) -> Result<bool, RepositoryError> {
        let exists: bool = sqlx::query_scalar(EMAIL_EXISTS)
            .bind(email.as_str())
            .fetch_one(&self.pool)
            .await
            .map_err(map_sqlx_error)?;

        Ok(exists)
    }
}

fn map_sqlx_error(e: sqlx::Error) -> RepositoryError {
    match &e {
        sqlx::Error::Database(db) if db.is_unique_violation() => {
            RepositoryError::Conflict(db.constraint().unwrap_or("unknown_constraint").to_string())
        }
        _ => RepositoryError::Storage(e.into()),
    }
}
