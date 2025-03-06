//! Configuration for the Kraken API client

/// Configuration for the Kraken API client
#[derive(Debug, Clone)]
pub struct Config {
    /// API key for authenticated requests
    pub api_key: Option<String>,
    
    /// API secret for authenticated requests
    pub api_secret: Option<String>,
    
    /// Base URL for the Kraken API
    pub api_url: String,
    
    /// WebSocket URL for the Kraken API
    pub ws_url: String,
    
    /// Timeout for HTTP requests in seconds
    pub timeout: u64,
    
    /// User agent string
    pub user_agent: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            api_url: "https://api.kraken.com".to_string(),
            ws_url: "wss://ws.kraken.com".to_string(),
            timeout: 30,
            user_agent: format!("kraken_client/{}", env!("CARGO_PKG_VERSION")),
        }
    }
}

impl Config {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Set the API key
    pub fn with_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
        self
    }
    
    /// Set the API secret
    pub fn with_api_secret(mut self, api_secret: impl Into<String>) -> Self {
        self.api_secret = Some(api_secret.into());
        self
    }
    
    /// Set the API URL
    pub fn with_api_url(mut self, api_url: impl Into<String>) -> Self {
        self.api_url = api_url.into();
        self
    }
    
    /// Set the WebSocket URL
    pub fn with_ws_url(mut self, ws_url: impl Into<String>) -> Self {
        self.ws_url = ws_url.into();
        self
    }
    
    /// Set the timeout
    pub fn with_timeout(mut self, timeout: u64) -> Self {
        self.timeout = timeout;
        self
    }
    
    /// Set the user agent
    pub fn with_user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
}
