use thiserror::Error;

#[derive(Error, Debug)]
pub enum GSheetError {
    #[error("HTTP request error: {0}")]
    HttpRequestError(#[from] reqwest::Error),

    #[error("Response parse error: {0}")]
    ResponseParseError(String),

    #[error("Authentication error: {0}")]
    AuthError(#[from] crate::auth::error::AuthError),

    #[error("Utility function error: {0}")]
    UtilsError(String),

    #[error("Other error: {0}")]
    Other(String),
}
