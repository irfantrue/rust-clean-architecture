use secrecy::SecretString;
use uuid::Uuid;

use crate::error::TokenServiceError;

#[derive(Debug)]
pub struct AccessToken {
    pub token: SecretString,
    pub expires_in: u64,
}

#[derive(Debug)]
pub struct AuthClaims {
    pub user_id: Uuid,
}

pub type TokenServiceResult<T> = std::result::Result<T, TokenServiceError>;

pub trait TokenService: Send + Sync {
    fn issue(&self, user_id: Uuid) -> TokenServiceResult<AccessToken>;
    fn verify(&self, token: &str) -> TokenServiceResult<AuthClaims>;
}
