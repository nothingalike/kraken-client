//! Kraken API client implementation

use reqwest::{Client as HttpClient, ClientBuilder};
use std::time::Duration;

use crate::api::{public::PublicApi, private::PrivateApi, websocket::WebSocketApi, rate_limiter::RateLimiter};
use crate::config::Config;
use crate::error::Result;

/// Kraken API client
#[derive(Debug, Clone)]
pub struct KrakenClient {
    /// Client configuration
    pub config: Config,
    
    /// HTTP client
    http_client: HttpClient,
    
    /// Rate limiter
    rate_limiter: RateLimiter,
}

impl KrakenClient {
    /// Create a new Kraken API client with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let http_client = ClientBuilder::new()
            .timeout(Duration::from_secs(config.timeout))
            .user_agent(&config.user_agent)
            .build()?;
        
        Ok(Self {
            config,
            http_client,
            rate_limiter: RateLimiter::new(),
        })
    }
    
    /// Create a new Kraken API client with default configuration
    pub fn default() -> Result<Self> {
        Self::new(Config::default())
    }
    
    /// Get the HTTP client
    pub fn http_client(&self) -> &HttpClient {
        &self.http_client
    }
    
    /// Get the rate limiter
    pub fn rate_limiter(&self) -> &RateLimiter {
        &self.rate_limiter
    }
    
    /// Get the public API
    pub fn public(&self) -> PublicApi {
        PublicApi::new(self)
    }
    
    /// Get the private API
    pub fn private(&self) -> PrivateApi {
        PrivateApi::new(self)
    }
    
    /// Get the WebSocket API
    pub fn websocket(&self) -> WebSocketApi {
        WebSocketApi::new(self)
    }
}
