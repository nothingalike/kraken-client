//! Private API endpoints for the Kraken API

use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;

use crate::auth::{generate_nonce, sign_message};
use crate::client::KrakenClient;
use crate::error::{Error, Result};
use crate::models::account::{Balance, TradeBalance, OpenOrders, ClosedOrders, Ledger, TradeHistory};
use crate::models::trading::{Order, OrderResponse, OrderInfo, TradeInfo};
use crate::utils::hashmap_to_url_encoded;

/// Response wrapper for Kraken API responses
#[derive(Debug, Deserialize)]
struct KrakenResponse<T> {
    /// Error messages
    error: Vec<String>,
    
    /// Result data
    result: Option<T>,
}

/// Private API endpoints
pub struct PrivateApi<'a> {
    /// Reference to the Kraken client
    client: &'a KrakenClient,
}

impl<'a> PrivateApi<'a> {
    /// Create a new private API instance
    pub fn new(client: &'a KrakenClient) -> Self {
        Self { client }
    }
    
    /// Make a private API request
    async fn private_request<T: for<'de> Deserialize<'de>>(&self, endpoint: &str, mut params: HashMap<String, String>) -> Result<T> {
        // Check if API key and secret are set
        let api_key = self.client.config.api_key.clone().ok_or_else(|| Error::Auth("API key not set".to_string()))?;
        let api_secret = self.client.config.api_secret.clone().ok_or_else(|| Error::Auth("API secret not set".to_string()))?;
        
        // Generate nonce
        let nonce = generate_nonce();
        params.insert("nonce".to_string(), nonce.to_string());
        
        // Create post data
        let post_data = hashmap_to_url_encoded(&params);
        
        // Sign the request
        let signature = sign_message(endpoint, nonce, &post_data, &api_secret)?;
        
        // Create the URL
        let url = format!("{}{}", self.client.config.api_url, endpoint);
        
        // Make the request
        let response = self.client.http_client()
            .post(&url)
            .header("API-Key", api_key)
            .header("API-Sign", signature)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(post_data)
            .send()
            .await?
            .json::<KrakenResponse<T>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        response.result.ok_or_else(|| Error::Api("No result data".to_string()))
    }
    
    /// Get account balance
    pub async fn get_balance(&self) -> Result<Balance> {
        self.private_request("/0/private/Balance", HashMap::new()).await
    }
    
    /// Get trade balance
    pub async fn get_trade_balance(&self, asset: Option<&str>) -> Result<TradeBalance> {
        let mut params = HashMap::new();
        
        if let Some(asset) = asset {
            params.insert("asset".to_string(), asset.to_string());
        }
        
        self.private_request("/0/private/TradeBalance", params).await
    }
    
    /// Get open orders
    pub async fn get_open_orders(&self, trades: Option<bool>, userref: Option<&str>) -> Result<OpenOrders> {
        let mut params = HashMap::new();
        
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        
        if let Some(userref) = userref {
            params.insert("userref".to_string(), userref.to_string());
        }
        
        let result: HashMap<String, Value> = self.private_request("/0/private/OpenOrders", params).await?;
        
        // Extract the open orders
        let open_orders = result.get("open").and_then(|v| v.as_object()).ok_or_else(|| Error::Api("Missing 'open' field".to_string()))?;
        
        // Parse the open orders
        let mut orders = HashMap::new();
        for (order_id, order_data) in open_orders {
            if let Ok(order) = serde_json::from_value(order_data.clone()) {
                orders.insert(order_id.clone(), order);
            }
        }
        
        Ok(orders)
    }
    
    /// Get closed orders
    pub async fn get_closed_orders(&self, trades: Option<bool>, userref: Option<&str>, start: Option<u64>, end: Option<u64>, ofs: Option<u64>, closetime: Option<&str>) -> Result<ClosedOrders> {
        let mut params = HashMap::new();
        
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        
        if let Some(userref) = userref {
            params.insert("userref".to_string(), userref.to_string());
        }
        
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        
        if let Some(ofs) = ofs {
            params.insert("ofs".to_string(), ofs.to_string());
        }
        
        if let Some(closetime) = closetime {
            params.insert("closetime".to_string(), closetime.to_string());
        }
        
        let result: HashMap<String, Value> = self.private_request("/0/private/ClosedOrders", params).await?;
        
        // Extract the closed orders
        let closed_orders = result.get("closed").and_then(|v| v.as_object()).ok_or_else(|| Error::Api("Missing 'closed' field".to_string()))?;
        
        // Parse the closed orders
        let mut orders = HashMap::new();
        for (order_id, order_data) in closed_orders {
            if let Ok(order) = serde_json::from_value(order_data.clone()) {
                orders.insert(order_id.clone(), order);
            }
        }
        
        Ok(orders)
    }
    
    /// Query orders info
    pub async fn query_orders(&self, txid: Vec<&str>, trades: Option<bool>, userref: Option<&str>) -> Result<HashMap<String, OrderInfo>> {
        let mut params = HashMap::new();
        params.insert("txid".to_string(), txid.join(","));
        
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        
        if let Some(userref) = userref {
            params.insert("userref".to_string(), userref.to_string());
        }
        
        self.private_request("/0/private/QueryOrders", params).await
    }
    
    /// Get trades history
    pub async fn get_trades_history(&self, type_: Option<&str>, trades: Option<bool>, start: Option<u64>, end: Option<u64>, ofs: Option<u64>) -> Result<TradeHistory> {
        let mut params = HashMap::new();
        
        if let Some(type_) = type_ {
            params.insert("type".to_string(), type_.to_string());
        }
        
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        
        if let Some(ofs) = ofs {
            params.insert("ofs".to_string(), ofs.to_string());
        }
        
        let result: HashMap<String, Value> = self.private_request("/0/private/TradesHistory", params).await?;
        
        // Extract the trades
        let trades = result.get("trades").and_then(|v| v.as_object()).ok_or_else(|| Error::Api("Missing 'trades' field".to_string()))?;
        
        // Parse the trades
        let mut trades_history = HashMap::new();
        for (trade_id, trade_data) in trades {
            if let Ok(trade) = serde_json::from_value(trade_data.clone()) {
                trades_history.insert(trade_id.clone(), trade);
            }
        }
        
        Ok(trades_history)
    }
    
    /// Query trades info
    pub async fn query_trades(&self, txid: Vec<&str>, trades: Option<bool>) -> Result<HashMap<String, TradeInfo>> {
        let mut params = HashMap::new();
        params.insert("txid".to_string(), txid.join(","));
        
        if let Some(trades) = trades {
            params.insert("trades".to_string(), trades.to_string());
        }
        
        self.private_request("/0/private/QueryTrades", params).await
    }
    
    /// Get ledgers info
    pub async fn get_ledgers(&self, asset: Option<Vec<&str>>, type_: Option<&str>, start: Option<u64>, end: Option<u64>, ofs: Option<u64>) -> Result<Ledger> {
        let mut params = HashMap::new();
        
        if let Some(asset) = asset {
            params.insert("asset".to_string(), asset.join(","));
        }
        
        if let Some(type_) = type_ {
            params.insert("type".to_string(), type_.to_string());
        }
        
        if let Some(start) = start {
            params.insert("start".to_string(), start.to_string());
        }
        
        if let Some(end) = end {
            params.insert("end".to_string(), end.to_string());
        }
        
        if let Some(ofs) = ofs {
            params.insert("ofs".to_string(), ofs.to_string());
        }
        
        let result: HashMap<String, Value> = self.private_request("/0/private/Ledgers", params).await?;
        
        // Extract the ledgers
        let ledgers = result.get("ledger").and_then(|v| v.as_object()).ok_or_else(|| Error::Api("Missing 'ledger' field".to_string()))?;
        
        // Parse the ledgers
        let mut ledger_entries = HashMap::new();
        for (ledger_id, ledger_data) in ledgers {
            if let Ok(ledger) = serde_json::from_value(ledger_data.clone()) {
                ledger_entries.insert(ledger_id.clone(), ledger);
            }
        }
        
        Ok(ledger_entries)
    }
    
    /// Add order
    pub async fn add_order(&self, order: &Order) -> Result<OrderResponse> {
        let mut params = HashMap::new();
        
        // Convert order to parameters
        params.insert("pair".to_string(), order.pair.clone());
        params.insert("type".to_string(), order.type_.to_string());
        params.insert("ordertype".to_string(), order.ordertype.to_string());
        params.insert("volume".to_string(), order.volume.clone());
        
        if let Some(ref price) = order.price {
            params.insert("price".to_string(), price.clone());
        }
        
        if let Some(ref price2) = order.price2 {
            params.insert("price2".to_string(), price2.clone());
        }
        
        if let Some(ref leverage) = order.leverage {
            params.insert("leverage".to_string(), leverage.clone());
        }
        
        if let Some(ref oflags) = order.oflags {
            params.insert("oflags".to_string(), oflags.clone());
        }
        
        if let Some(ref starttm) = order.starttm {
            params.insert("starttm".to_string(), starttm.clone());
        }
        
        if let Some(ref expiretm) = order.expiretm {
            params.insert("expiretm".to_string(), expiretm.clone());
        }
        
        if let Some(ref userref) = order.userref {
            params.insert("userref".to_string(), userref.clone());
        }
        
        if let Some(validate) = order.validate {
            params.insert("validate".to_string(), validate.to_string());
        }
        
        if let Some(ref close_ordertype) = order.close_ordertype {
            params.insert("close[ordertype]".to_string(), close_ordertype.to_string());
        }
        
        if let Some(ref close_price) = order.close_price {
            params.insert("close[price]".to_string(), close_price.clone());
        }
        
        if let Some(ref close_price2) = order.close_price2 {
            params.insert("close[price2]".to_string(), close_price2.clone());
        }
        
        self.private_request("/0/private/AddOrder", params).await
    }
    
    /// Cancel order
    pub async fn cancel_order(&self, txid: &str) -> Result<HashMap<String, Value>> {
        let mut params = HashMap::new();
        params.insert("txid".to_string(), txid.to_string());
        
        self.private_request("/0/private/CancelOrder", params).await
    }
    
    /// Cancel all orders
    pub async fn cancel_all_orders(&self) -> Result<HashMap<String, Value>> {
        self.private_request("/0/private/CancelAll", HashMap::new()).await
    }
}
