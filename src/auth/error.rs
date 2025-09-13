use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Token is expired")]
    TokenExpired,

    #[error("Failed to read service account file: {0}")]
    ServiceAccountReadError(#[from] std::io::Error),

    #[error("Failed to parse service account file: {0}")]
    ServiceAccountParseError(#[from] serde_json::Error),

    #[error("Failed to create JWT: {0}")]
    JwtCreationError(#[from] jsonwebtoken::errors::Error),

    #[error("HTTP request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Request error: {0}")]
    RequestError(String),

    #[error("Other error: {0}")]
    Other(String),
}
