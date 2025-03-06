//! Market data models for the Kraken API

use serde::{Deserialize, Serialize};

/// Ticker information
#[derive(Debug, Clone, Deserialize)]
pub struct Ticker {
    /// Ask array(<price>, <whole lot volume>, <lot volume>)
    pub a: Vec<String>,
    
    /// Bid array(<price>, <whole lot volume>, <lot volume>)
    pub b: Vec<String>,
    
    /// Last trade closed array(<price>, <lot volume>)
    pub c: Vec<String>,
    
    /// Volume array(<today>, <last 24 hours>)
    pub v: Vec<String>,
    
    /// Volume weighted average price array(<today>, <last 24 hours>)
    pub p: Vec<String>,
    
    /// Number of trades array(<today>, <last 24 hours>)
    pub t: Vec<i64>,
    
    /// Low array(<today>, <last 24 hours>)
    pub l: Vec<String>,
    
    /// High array(<today>, <last 24 hours>)
    pub h: Vec<String>,
    
    /// Today's opening price
    pub o: String,
}

/// Orderbook entry
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderbookEntry {
    /// Price
    pub price: String,
    
    /// Volume
    pub volume: String,
    
    /// Timestamp
    pub timestamp: Option<f64>,
}

/// Orderbook
#[derive(Debug, Clone, Deserialize)]
pub struct Orderbook {
    /// Ask side
    pub asks: Vec<OrderbookEntry>,
    
    /// Bid side
    pub bids: Vec<OrderbookEntry>,
}

/// Trade information
#[derive(Debug, Clone, Deserialize)]
pub struct Trade {
    /// Price
    pub price: String,
    
    /// Volume
    pub volume: String,
    
    /// Time
    pub time: f64,
    
    /// Buy/Sell
    pub side: String,
    
    /// Limit/Market
    pub order_type: String,
    
    /// Miscellaneous
    pub misc: String,
}

/// OHLC (Open, High, Low, Close) candle
#[derive(Debug, Clone, Deserialize)]
pub struct OHLC {
    /// Time
    pub time: i64,
    
    /// Open
    pub open: String,
    
    /// High
    pub high: String,
    
    /// Low
    pub low: String,
    
    /// Close
    pub close: String,
    
    /// Volume weighted average price
    pub vwap: String,
    
    /// Volume
    pub volume: String,
    
    /// Number of trades
    pub count: i64,
}

/// Asset information
#[derive(Debug, Clone, Deserialize)]
pub struct AssetInfo {
    /// Alternate name
    pub altname: String,
    
    /// Asset class
    pub aclass: String,
    
    /// Scaling decimal places for record keeping
    pub decimals: i64,
    
    /// Scaling decimal places for output display
    pub display_decimals: i64,
}

/// Asset pair information
#[derive(Debug, Clone, Deserialize)]
pub struct AssetPair {
    /// Alternate pair name
    pub altname: String,
    
    /// WebSocket pair name (if available)
    pub wsname: Option<String>,
    
    /// Asset class of base component
    pub aclass_base: String,
    
    /// Asset id of base component
    pub base: String,
    
    /// Asset class of quote component
    pub aclass_quote: String,
    
    /// Asset id of quote component
    pub quote: String,
    
    /// Lot multiplier
    pub lot: String,
    
    /// Scaling decimal places for pair
    pub pair_decimals: i64,
    
    /// Scaling decimal places for volume
    pub lot_decimals: i64,
    
    /// Amount to multiply lot volume by to get currency volume
    pub lot_multiplier: i64,
    
    /// Fee schedule array in [<volume>, <percent fee>] tuples
    pub fees: Vec<Vec<f64>>,
    
    /// Maker fee schedule array in [<volume>, <percent fee>] tuples (if on maker-taker)
    pub fees_maker: Option<Vec<Vec<f64>>>,
    
    /// Volume discount currency
    pub fee_volume_currency: String,
    
    /// Margin call level
    pub margin_call: i64,
    
    /// Stop-out/liquidation margin level
    pub margin_stop: i64,
    
    /// Minimum order size
    pub ordermin: Option<String>,
}

/// Server time
#[derive(Debug, Clone, Deserialize)]
pub struct ServerTime {
    /// Unix timestamp
    pub unixtime: i64,
    
    /// RFC 1123 time format
    pub rfc1123: String,
}
