//! Error handling for the Kraken API client

use thiserror::Error;

/// Result type for the Kraken API client
pub type Result<T> = std::result::Result<T, Error>;

/// Error type for the Kraken API client
#[derive(Error, Debug)]
pub enum Error {
    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL error
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// API error
    #[error("API error: {0}")]
    Api(String),

    /// Authentication error
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Rate limit error
    #[error("Rate limit error: {0}")]
    RateLimit(String),

    /// WebSocket error
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Other error
    #[error("Other error: {0}")]
    Other(String),
}
