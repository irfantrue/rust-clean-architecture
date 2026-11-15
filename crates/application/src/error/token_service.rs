#[derive(Debug, thiserror::Error)]
pub enum TokenServiceError {
    #[error("token has expired")]
    Expired,

    #[error("token is invalid")]
    Invalid,

    #[error("token service failure")]
    Internal(#[source] anyhow::Error),
}
