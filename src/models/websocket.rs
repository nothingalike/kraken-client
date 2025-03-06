//! WebSocket models for the Kraken API

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// WebSocket message types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WebSocketMessageType {
    /// Subscribe to a channel
    Subscribe,
    
    /// Unsubscribe from a channel
    Unsubscribe,
    
    /// Ping
    Ping,
    
    /// Pong
    Pong,
    
    /// Heartbeat
    Heartbeat,
    
    /// System status
    #[serde(rename = "systemStatus")]
    SystemStatus,
    
    /// Subscription status
    #[serde(rename = "subscriptionStatus")]
    SubscriptionStatus,
}

/// WebSocket subscription types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WebSocketSubscriptionType {
    /// Ticker
    Ticker,
    
    /// OHLC
    OHLC,
    
    /// Trade
    Trade,
    
    /// Spread
    Spread,
    
    /// Book
    Book,
    
    /// All tickers
    #[serde(rename = "*")]
    All,
}

/// WebSocket subscription
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebSocketSubscription {
    /// Subscription name
    pub name: WebSocketSubscriptionType,
    
    /// Interval for OHLC
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
    
    /// Depth for book
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth: Option<u32>,
}

/// WebSocket subscription request
#[derive(Debug, Clone, Serialize)]
pub struct WebSocketSubscriptionRequest {
    /// Event type
    pub event: WebSocketMessageType,
    
    /// Subscription
    pub subscription: WebSocketSubscription,
    
    /// Pairs to subscribe to
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<Vec<String>>,
}

impl WebSocketSubscriptionRequest {
    /// Create a new subscription request with default ticker subscription
    pub fn new() -> Self {
        Self {
            event: WebSocketMessageType::Subscribe,
            subscription: WebSocketSubscription {
                name: WebSocketSubscriptionType::Ticker,
                interval: None,
                depth: None,
            },
            pair: None,
        }
    }
    
    /// Create a new subscription request with specific subscription type
    pub fn new_with_type(subscription_type: WebSocketSubscriptionType) -> Self {
        Self {
            event: WebSocketMessageType::Subscribe,
            subscription: WebSocketSubscription {
                name: subscription_type,
                interval: None,
                depth: None,
            },
            pair: None,
        }
    }
    
    /// Set the pairs to subscribe to
    pub fn with_pairs(mut self, pairs: Vec<String>) -> Self {
        self.pair = Some(pairs);
        self
    }
    
    /// Add a pair to subscribe to
    pub fn add_pair<S: Into<String>>(mut self, pair: S) -> Self {
        if let Some(pairs) = self.pair.as_mut() {
            pairs.push(pair.into());
        } else {
            self.pair = Some(vec![pair.into()]);
        }
        self
    }
    
    /// Set the interval for OHLC
    pub fn with_interval(mut self, interval: u32) -> Self {
        self.subscription.interval = Some(interval);
        self
    }
    
    /// Set the depth for book
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.subscription.depth = Some(depth);
        self
    }
    
    /// Set the subscription type
    pub fn add_subscription<S: AsRef<str>>(mut self, name: S) -> Self {
        let name_str = name.as_ref();
        self.subscription.name = match name_str {
            "ticker" => WebSocketSubscriptionType::Ticker,
            "ohlc" => WebSocketSubscriptionType::OHLC,
            "trade" => WebSocketSubscriptionType::Trade,
            "spread" => WebSocketSubscriptionType::Spread,
            "book" => WebSocketSubscriptionType::Book,
            _ => WebSocketSubscriptionType::Ticker, // Default to ticker
        };
        self
    }
}

/// WebSocket unsubscription request
#[derive(Debug, Clone, Serialize)]
pub struct WebSocketUnsubscriptionRequest {
    /// Event type
    pub event: WebSocketMessageType,
    
    /// Subscription
    pub subscription: WebSocketSubscription,
    
    /// Pairs to unsubscribe from
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pair: Option<Vec<String>>,
}

impl WebSocketUnsubscriptionRequest {
    /// Create a new unsubscription request
    pub fn new(subscription_type: WebSocketSubscriptionType) -> Self {
        Self {
            event: WebSocketMessageType::Unsubscribe,
            subscription: WebSocketSubscription {
                name: subscription_type,
                interval: None,
                depth: None,
            },
            pair: None,
        }
    }
    
    /// Set the pairs to unsubscribe from
    pub fn with_pairs(mut self, pairs: Vec<String>) -> Self {
        self.pair = Some(pairs);
        self
    }
    
    /// Set the interval for OHLC
    pub fn with_interval(mut self, interval: u32) -> Self {
        self.subscription.interval = Some(interval);
        self
    }
    
    /// Set the depth for book
    pub fn with_depth(mut self, depth: u32) -> Self {
        self.subscription.depth = Some(depth);
        self
    }
}

/// WebSocket message
#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum WebSocketMessage {
    /// System status
    SystemStatus {
        /// Event type
        event: String,
        
        /// Connection ID
        #[serde(rename = "connectionID")]
        connection_id: u64,
        
        /// Status
        status: String,
        
        /// Version
        version: String,
    },
    
    /// Subscription status
    SubscriptionStatus {
        /// Channel ID
        #[serde(rename = "channelID")]
        channel_id: Option<u64>,
        
        /// Channel name
        #[serde(rename = "channelName")]
        channel_name: Option<String>,
        
        /// Event type
        event: String,
        
        /// Pair
        pair: Option<String>,
        
        /// Status
        status: String,
        
        /// Subscription
        subscription: WebSocketSubscription,
    },
    
    /// Heartbeat
    Heartbeat {
        /// Event type
        #[serde(rename = "event")]
        event_type: WebSocketMessageType,
    },
    
    /// Ping
    Ping {
        /// Event type
        #[serde(rename = "event")]
        event_type: WebSocketMessageType,
        
        /// Request ID
        #[serde(rename = "reqid")]
        req_id: Option<u64>,
    },
    
    /// Pong
    Pong {
        /// Event type
        #[serde(rename = "event")]
        event_type: WebSocketMessageType,
        
        /// Request ID
        #[serde(rename = "reqid")]
        req_id: Option<u64>,
    },
    
    /// Error
    Error {
        /// Event type
        event: String,
        
        /// Error message
        #[serde(rename = "errorMessage")]
        error_message: String,
        
        /// Status
        status: String,
        
        /// Subscription
        subscription: Option<WebSocketSubscription>,
        
        /// Pair
        pair: Option<String>,
    },
    
    /// Data array
    DataArray(Vec<Value>),
    
    /// Generic message
    Generic(Value),
}
