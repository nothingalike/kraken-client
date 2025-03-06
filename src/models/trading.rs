//! Trading models for the Kraken API

use serde::{Deserialize, Serialize};
use crate::models::account::OrderDescription;

/// Order types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderType {
    /// Market order
    Market,
    
    /// Limit order
    Limit,
    
    /// Stop-loss order
    StopLoss,
    
    /// Take-profit order
    TakeProfit,
    
    /// Stop-loss-limit order
    StopLossLimit,
    
    /// Take-profit-limit order
    TakeProfitLimit,
    
    /// Settle-position order
    SettlePosition,
}

impl ToString for OrderType {
    fn to_string(&self) -> String {
        match self {
            OrderType::Market => "market".to_string(),
            OrderType::Limit => "limit".to_string(),
            OrderType::StopLoss => "stop-loss".to_string(),
            OrderType::TakeProfit => "take-profit".to_string(),
            OrderType::StopLossLimit => "stop-loss-limit".to_string(),
            OrderType::TakeProfitLimit => "take-profit-limit".to_string(),
            OrderType::SettlePosition => "settle-position".to_string(),
        }
    }
}

/// Order sides
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderSide {
    /// Buy order
    Buy,
    
    /// Sell order
    Sell,
}

impl ToString for OrderSide {
    fn to_string(&self) -> String {
        match self {
            OrderSide::Buy => "buy".to_string(),
            OrderSide::Sell => "sell".to_string(),
        }
    }
}

/// Order statuses
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    /// Pending order
    Pending,
    
    /// Open order
    Open,
    
    /// Closed order
    Closed,
    
    /// Canceled order
    Canceled,
    
    /// Expired order
    Expired,
}

/// Order flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderFlag {
    /// Post-only order (available when ordertype = limit)
    Post,
    
    /// Prefer fee in base currency (default if selling)
    Fcib,
    
    /// Prefer fee in quote currency (default if buying)
    Fciq,
    
    /// No market price protection
    Nompp,
    
    /// Order volume in quote currency
    Viqc,
}

impl ToString for OrderFlag {
    fn to_string(&self) -> String {
        match self {
            OrderFlag::Post => "post".to_string(),
            OrderFlag::Fcib => "fcib".to_string(),
            OrderFlag::Fciq => "fciq".to_string(),
            OrderFlag::Nompp => "nompp".to_string(),
            OrderFlag::Viqc => "viqc".to_string(),
        }
    }
}

/// Order request
#[derive(Debug, Clone, Serialize)]
pub struct Order {
    /// Asset pair
    pub pair: String,
    
    /// Type of order (buy/sell)
    pub type_: OrderSide,
    
    /// Order type
    pub ordertype: OrderType,
    
    /// Order volume in base currency
    pub volume: String,
    
    /// Price (optional, dependent on ordertype)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    
    /// Secondary price (optional, dependent on ordertype)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price2: Option<String>,
    
    /// Amount of leverage desired (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leverage: Option<String>,
    
    /// Comma delimited list of order flags (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oflags: Option<String>,
    
    /// Scheduled start time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starttm: Option<String>,
    
    /// Expiration time (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiretm: Option<String>,
    
    /// User reference ID (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub userref: Option<String>,
    
    /// Validate inputs only, do not submit order (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
    
    /// Close order type (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_ordertype: Option<OrderType>,
    
    /// Close order price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price: Option<String>,
    
    /// Close order secondary price (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_price2: Option<String>,
}

impl Order {
    /// Create a new order
    pub fn new(pair: impl Into<String>, side: OrderSide, order_type: OrderType, volume: impl Into<String>) -> Self {
        Self {
            pair: pair.into(),
            type_: side,
            ordertype: order_type,
            volume: volume.into(),
            price: None,
            price2: None,
            leverage: None,
            oflags: None,
            starttm: None,
            expiretm: None,
            userref: None,
            validate: None,
            close_ordertype: None,
            close_price: None,
            close_price2: None,
        }
    }
    
    /// Set the price
    pub fn with_price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }
    
    /// Set the secondary price
    pub fn with_price2(mut self, price2: impl Into<String>) -> Self {
        self.price2 = Some(price2.into());
        self
    }
    
    /// Set the leverage
    pub fn with_leverage(mut self, leverage: impl Into<String>) -> Self {
        self.leverage = Some(leverage.into());
        self
    }
    
    /// Add order flags
    pub fn with_flags(mut self, flags: &[OrderFlag]) -> Self {
        let flags_str = flags
            .iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>()
            .join(",");
        
        self.oflags = Some(flags_str);
        self
    }
    
    /// Set the start time
    pub fn with_start_time(mut self, start_time: impl Into<String>) -> Self {
        self.starttm = Some(start_time.into());
        self
    }
    
    /// Set the expiration time
    pub fn with_expiration_time(mut self, expiration_time: impl Into<String>) -> Self {
        self.expiretm = Some(expiration_time.into());
        self
    }
    
    /// Set the user reference ID
    pub fn with_user_ref(mut self, user_ref: impl Into<String>) -> Self {
        self.userref = Some(user_ref.into());
        self
    }
    
    /// Set the validate flag
    pub fn with_validate(mut self, validate: bool) -> Self {
        self.validate = Some(validate);
        self
    }
    
    /// Set the close order type
    pub fn with_close_order_type(mut self, close_order_type: OrderType) -> Self {
        self.close_ordertype = Some(close_order_type);
        self
    }
    
    /// Set the close order price
    pub fn with_close_price(mut self, close_price: impl Into<String>) -> Self {
        self.close_price = Some(close_price.into());
        self
    }
    
    /// Set the close order secondary price
    pub fn with_close_price2(mut self, close_price2: impl Into<String>) -> Self {
        self.close_price2 = Some(close_price2.into());
        self
    }
}

/// Order response
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResponse {
    /// Order description info
    pub descr: OrderResponseDescription,
    
    /// Transaction IDs
    pub txid: Vec<String>,
}

/// Order response description
#[derive(Debug, Clone, Deserialize)]
pub struct OrderResponseDescription {
    /// Order description
    pub order: String,
    
    /// Conditional close order description (if conditional close set)
    pub close: Option<String>,
}

/// Order info
#[derive(Debug, Clone, Deserialize)]
pub struct OrderInfo {
    /// Transaction ID
    pub txid: String,
    
    /// User reference ID
    pub userref: Option<i64>,
    
    /// Status of order
    pub status: String,
    
    /// Unix timestamp of when order was placed
    pub opentm: f64,
    
    /// Unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    
    /// Unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    
    /// Order description info
    pub descr: OrderDescription,
    
    /// Volume of order (base currency)
    pub vol: String,
    
    /// Volume executed (base currency)
    pub vol_exec: String,
    
    /// Total cost (quote currency unless viqc set in oflags)
    pub cost: String,
    
    /// Total fee (quote currency)
    pub fee: String,
    
    /// Average price (quote currency unless viqc set in oflags)
    pub price: String,
    
    /// Stop price (quote currency, for trailing stops)
    pub stopprice: Option<String>,
    
    /// Triggered limit price (quote currency, when limit based order type triggered)
    pub limitprice: Option<String>,
    
    /// Comma delimited list of miscellaneous info
    pub misc: String,
    
    /// Comma delimited list of order flags
    pub oflags: String,
}

/// Trade info
#[derive(Debug, Clone, Deserialize)]
pub struct TradeInfo {
    /// Order ID
    pub ordertxid: String,
    
    /// Position ID
    pub postxid: String,
    
    /// Asset pair
    pub pair: String,
    
    /// Unix timestamp of trade
    pub time: f64,
    
    /// Type of order (buy/sell)
    pub type_: String,
    
    /// Order type
    pub ordertype: String,
    
    /// Average price order was executed at (quote currency)
    pub price: String,
    
    /// Total cost of order (quote currency)
    pub cost: String,
    
    /// Total fee (quote currency)
    pub fee: String,
    
    /// Volume (base currency)
    pub vol: String,
    
    /// Initial margin (quote currency)
    pub margin: String,
    
    /// Comma delimited list of miscellaneous info
    pub misc: String,
}
