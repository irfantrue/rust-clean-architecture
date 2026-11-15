use chrono::{DateTime, Utc};
use secrecy::SecretString;
use uuid::Uuid;

use crate::error::Email;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: Email,
    pub password_hash: SecretString,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(email: Email, password_hash: SecretString) -> Self {
        Self {
            id: Uuid::now_v7(),
            email,
            password_hash,
            created_at: Utc::now(),
        }
    }
}
