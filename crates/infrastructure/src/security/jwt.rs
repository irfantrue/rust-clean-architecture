use std::time::{SystemTime, UNIX_EPOCH};

use application::{
    error::TokenServiceError,
    port::{AccessToken, AuthClaims, TokenService, TokenServiceResult},
};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    iss: String,
    iat: u64,
    exp: u64,
}

pub struct JwtTokenService {
    encoding: EncodingKey,
    decoding: DecodingKey,
    validation: Validation,
    issuer: String,
    ttl_secs: u64,
}

impl JwtTokenService {
    pub fn new(secret: &SecretString, issuer: String, ttl_secs: u64) -> Self {
        let bytes = secret.expose_secret().as_bytes();

        let mut validation = Validation::new(jsonwebtoken::Algorithm::HS256);
        validation.set_issuer(&[&issuer]);
        validation.set_required_spec_claims(&["exp", "sub", "iss"]);

        Self {
            encoding: EncodingKey::from_secret(bytes),
            decoding: DecodingKey::from_secret(bytes),
            validation,
            issuer,
            ttl_secs,
        }
    }
}

impl TokenService for JwtTokenService {
    fn issue(&self, user_id: Uuid) -> TokenServiceResult<AccessToken> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| TokenServiceError::Internal(e.into()))?
            .as_secs();

        let claims = Claims {
            sub: user_id.to_string(),
            iss: self.issuer.clone(),
            iat: now,
            exp: now + self.ttl_secs,
        };

        let token = encode(&Header::new(Algorithm::HS256), &claims, &self.encoding)
            .map_err(|e| TokenServiceError::Internal(e.into()))?;

        Ok(AccessToken {
            token: SecretString::from(token),
            expires_in: self.ttl_secs,
        })
    }

    fn verify(&self, token: &str) -> TokenServiceResult<AuthClaims> {
        let data =
            decode::<Claims>(token, &self.decoding, &self.validation).map_err(|e| {
                match e.kind() {
                    ErrorKind::ExpiredSignature => TokenServiceError::Expired,
                    _ => TokenServiceError::Invalid,
                }
            })?;
        let id = Uuid::parse_str(&data.claims.sub).map_err(|_| TokenServiceError::Invalid)?;
        Ok(AuthClaims { user_id: id })
    }
}
