//! Public API endpoints for the Kraken API

use std::collections::HashMap;
use serde::Deserialize;
use serde_json::Value;

use crate::client::KrakenClient;
use crate::error::{Error, Result};
use crate::models::market::{AssetInfo, AssetPair, OHLC, Orderbook, OrderbookEntry, ServerTime, Ticker, Trade};
use crate::utils::build_url;

/// Response wrapper for Kraken API responses
#[derive(Debug, Deserialize)]
struct KrakenResponse<T> {
    /// Error messages
    error: Vec<String>,
    
    /// Result data
    result: Option<T>,
}

/// Public API endpoints
pub struct PublicApi<'a> {
    /// Reference to the Kraken client
    client: &'a KrakenClient,
}

impl<'a> PublicApi<'a> {
    /// Create a new public API instance
    pub fn new(client: &'a KrakenClient) -> Self {
        Self { client }
    }
    
    /// Get server time
    pub async fn get_server_time(&self) -> Result<ServerTime> {
        let url = format!("{}/0/public/Time", self.client.config.api_url);
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<ServerTime>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        response.result.ok_or_else(|| Error::Api("No result data".to_string()))
    }
    
    /// Get asset info
    pub async fn get_assets(&self, assets: Option<Vec<&str>>) -> Result<HashMap<String, AssetInfo>> {
        let mut params = HashMap::new();
        
        if let Some(assets) = assets {
            params.insert("asset".to_string(), assets.join(","));
        }
        
        let url = build_url(&self.client.config.api_url, "/0/public/Assets", Some(&params));
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<HashMap<String, AssetInfo>>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        response.result.ok_or_else(|| Error::Api("No result data".to_string()))
    }
    
    /// Get tradable asset pairs
    pub async fn get_asset_pairs(&self, pairs: Option<Vec<&str>>) -> Result<HashMap<String, AssetPair>> {
        let mut params = HashMap::new();
        
        if let Some(pairs) = pairs {
            params.insert("pair".to_string(), pairs.join(","));
        }
        
        let url = build_url(&self.client.config.api_url, "/0/public/AssetPairs", Some(&params));
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<HashMap<String, AssetPair>>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        response.result.ok_or_else(|| Error::Api("No result data".to_string()))
    }
    
    /// Get ticker information
    pub async fn get_ticker(&self, pairs: Vec<&str>) -> Result<HashMap<String, Ticker>> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pairs.join(","));
        
        let url = build_url(&self.client.config.api_url, "/0/public/Ticker", Some(&params));
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<HashMap<String, Ticker>>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        response.result.ok_or_else(|| Error::Api("No result data".to_string()))
    }
    
    /// Get OHLC data
    pub async fn get_ohlc(&self, pair: &str, interval: Option<u32>, since: Option<u64>) -> Result<(Vec<OHLC>, u64)> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair.to_string());
        
        if let Some(interval) = interval {
            params.insert("interval".to_string(), interval.to_string());
        }
        
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        
        let url = build_url(&self.client.config.api_url, "/0/public/OHLC", Some(&params));
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<HashMap<String, Value>>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        let result = response.result.ok_or_else(|| Error::Api("No result data".to_string()))?;
        
        // Extract the last field which is the 'last' timestamp
        let last = result.get("last").and_then(|v| v.as_u64()).ok_or_else(|| Error::Api("Missing 'last' field".to_string()))?;
        
        // Extract the OHLC data
        let ohlc_data = result.get(pair).and_then(|v| v.as_array()).ok_or_else(|| Error::Api(format!("Missing data for pair {}", pair)))?;
        
        // Parse the OHLC data
        let mut ohlc_vec = Vec::new();
        for item in ohlc_data {
            if let Some(arr) = item.as_array() {
                if arr.len() >= 8 {
                    let ohlc = OHLC {
                        time: arr[0].as_i64().unwrap_or(0),
                        open: arr[1].as_str().unwrap_or("0").to_string(),
                        high: arr[2].as_str().unwrap_or("0").to_string(),
                        low: arr[3].as_str().unwrap_or("0").to_string(),
                        close: arr[4].as_str().unwrap_or("0").to_string(),
                        vwap: arr[5].as_str().unwrap_or("0").to_string(),
                        volume: arr[6].as_str().unwrap_or("0").to_string(),
                        count: arr[7].as_i64().unwrap_or(0),
                    };
                    ohlc_vec.push(ohlc);
                }
            }
        }
        
        Ok((ohlc_vec, last))
    }
    
    /// Get order book
    pub async fn get_orderbook(&self, pair: &str, count: Option<u32>) -> Result<HashMap<String, Orderbook>> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair.to_string());
        
        if let Some(count) = count {
            params.insert("count".to_string(), count.to_string());
        }
        
        let url = build_url(&self.client.config.api_url, "/0/public/Depth", Some(&params));
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<HashMap<String, Value>>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        let result = response.result.ok_or_else(|| Error::Api("No result data".to_string()))?;
        
        let mut orderbooks = HashMap::new();
        
        for (pair_name, orderbook_data) in result {
            let asks_data = orderbook_data.get("asks").and_then(|v| v.as_array()).ok_or_else(|| Error::Api("Missing 'asks' field".to_string()))?;
            let bids_data = orderbook_data.get("bids").and_then(|v| v.as_array()).ok_or_else(|| Error::Api("Missing 'bids' field".to_string()))?;
            
            let mut asks = Vec::new();
            for ask in asks_data {
                if let Some(arr) = ask.as_array() {
                    if arr.len() >= 2 {
                        let entry = OrderbookEntry {
                            price: arr[0].as_str().unwrap_or("0").to_string(),
                            volume: arr[1].as_str().unwrap_or("0").to_string(),
                            timestamp: arr.get(2).and_then(|v| v.as_f64()),
                        };
                        asks.push(entry);
                    }
                }
            }
            
            let mut bids = Vec::new();
            for bid in bids_data {
                if let Some(arr) = bid.as_array() {
                    if arr.len() >= 2 {
                        let entry = OrderbookEntry {
                            price: arr[0].as_str().unwrap_or("0").to_string(),
                            volume: arr[1].as_str().unwrap_or("0").to_string(),
                            timestamp: arr.get(2).and_then(|v| v.as_f64()),
                        };
                        bids.push(entry);
                    }
                }
            }
            
            orderbooks.insert(pair_name, Orderbook { asks, bids });
        }
        
        Ok(orderbooks)
    }
    
    /// Get recent trades
    pub async fn get_trades(&self, pair: &str, since: Option<u64>) -> Result<(Vec<Trade>, u64)> {
        let mut params = HashMap::new();
        params.insert("pair".to_string(), pair.to_string());
        
        if let Some(since) = since {
            params.insert("since".to_string(), since.to_string());
        }
        
        let url = build_url(&self.client.config.api_url, "/0/public/Trades", Some(&params));
        
        let response = self.client.http_client()
            .get(&url)
            .send()
            .await?
            .json::<KrakenResponse<HashMap<String, Value>>>()
            .await?;
        
        if !response.error.is_empty() {
            return Err(Error::Api(response.error.join(", ")));
        }
        
        let result = response.result.ok_or_else(|| Error::Api("No result data".to_string()))?;
        
        // Extract the last field which is the 'last' timestamp
        let last = result.get("last").and_then(|v| v.as_u64()).ok_or_else(|| Error::Api("Missing 'last' field".to_string()))?;
        
        // Extract the trades data
        let trades_data = result.get(pair).and_then(|v| v.as_array()).ok_or_else(|| Error::Api(format!("Missing data for pair {}", pair)))?;
        
        // Parse the trades data
        let mut trades_vec = Vec::new();
        for item in trades_data {
            if let Some(arr) = item.as_array() {
                if arr.len() >= 6 {
                    let trade = Trade {
                        price: arr[0].as_str().unwrap_or("0").to_string(),
                        volume: arr[1].as_str().unwrap_or("0").to_string(),
                        time: arr[2].as_f64().unwrap_or(0.0),
                        side: arr[3].as_str().unwrap_or("").to_string(),
                        order_type: arr[4].as_str().unwrap_or("").to_string(),
                        misc: arr[5].as_str().unwrap_or("").to_string(),
                    };
                    trades_vec.push(trade);
                }
            }
        }
        
        Ok((trades_vec, last))
    }
}
