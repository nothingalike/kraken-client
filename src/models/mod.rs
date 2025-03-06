//! Data models for the Kraken API

pub mod market;
pub mod account;
pub mod trading;
pub mod websocket;

// Re-export commonly used types
pub use market::{Ticker, Orderbook, Trade, OHLC};
pub use account::{Balance, TradeBalance, OpenOrders, ClosedOrders};
pub use trading::{OrderType, OrderSide, OrderStatus, Order, OrderInfo, TradeInfo};
pub use websocket::{WebSocketMessage, WebSocketSubscription};
