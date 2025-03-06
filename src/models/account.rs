//! Account data models for the Kraken API

use serde::Deserialize;
use std::collections::HashMap;

/// Account balance
pub type Balance = HashMap<String, String>;

/// Trade balance information
#[derive(Debug, Clone, Deserialize)]
pub struct TradeBalance {
    /// Equivalent balance (combined balance of all currencies)
    pub eb: String,
    
    /// Trade balance (combined balance of all equity currencies)
    pub tb: String,
    
    /// Margin amount of open positions
    pub m: String,
    
    /// Unrealized net profit/loss of open positions
    pub n: String,
    
    /// Cost basis of open positions
    pub c: String,
    
    /// Current floating valuation of open positions
    pub v: String,
    
    /// Equity = trade balance + unrealized net profit/loss
    pub e: String,
    
    /// Free margin = equity - initial margin (maximum margin available to open new positions)
    pub mf: String,
    
    /// Margin level = (equity / initial margin) * 100
    pub ml: Option<String>,
}

/// Open order
#[derive(Debug, Clone, Deserialize)]
pub struct OpenOrder {
    /// Referral order transaction ID that created this order
    pub refid: Option<String>,
    
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
    
    /// Array of trade IDs related to order (if trades info requested and data available)
    pub trades: Option<Vec<String>>,
}

/// Open orders
pub type OpenOrders = HashMap<String, OpenOrder>;

/// Closed order
#[derive(Debug, Clone, Deserialize)]
pub struct ClosedOrder {
    /// Referral order transaction ID that created this order
    pub refid: Option<String>,
    
    /// User reference ID
    pub userref: Option<i64>,
    
    /// Status of order
    pub status: String,
    
    /// Reason order was closed
    pub reason: Option<String>,
    
    /// Unix timestamp of when order was placed
    pub opentm: f64,
    
    /// Unix timestamp of order start time (or 0 if not set)
    pub starttm: f64,
    
    /// Unix timestamp of order end time (or 0 if not set)
    pub expiretm: f64,
    
    /// Unix timestamp of when order was closed
    pub closetm: f64,
    
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
    
    /// Array of trade IDs related to order (if trades info requested and data available)
    pub trades: Option<Vec<String>>,
}

/// Closed orders
pub type ClosedOrders = HashMap<String, ClosedOrder>;

/// Order description
#[derive(Debug, Clone, Deserialize)]
pub struct OrderDescription {
    /// Asset pair
    pub pair: String,
    
    /// Type of order (buy/sell)
    pub type_: String,
    
    /// Order type
    pub ordertype: String,
    
    /// Primary price
    pub price: String,
    
    /// Secondary price
    pub price2: String,
    
    /// Amount of leverage
    pub leverage: String,
    
    /// Order description
    pub order: String,
    
    /// Conditional close order description (if conditional close set)
    pub close: Option<String>,
}

/// Ledger entry
#[derive(Debug, Clone, Deserialize)]
pub struct LedgerEntry {
    /// Reference ID
    pub refid: String,
    
    /// Unix timestamp of ledger entry
    pub time: f64,
    
    /// Type of ledger entry
    pub type_: String,
    
    /// Asset class
    pub aclass: String,
    
    /// Asset
    pub asset: String,
    
    /// Amount
    pub amount: String,
    
    /// Fee
    pub fee: String,
    
    /// Balance
    pub balance: String,
}

/// Ledger entries
pub type Ledger = HashMap<String, LedgerEntry>;

/// Trade history entry
#[derive(Debug, Clone, Deserialize)]
pub struct TradeHistoryEntry {
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

/// Trade history
pub type TradeHistory = HashMap<String, TradeHistoryEntry>;
