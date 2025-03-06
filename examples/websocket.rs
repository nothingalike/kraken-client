use kraken_client::{
    client::KrakenClient,
    models::websocket::{WebSocketMessage, WebSocketSubscriptionRequest},
    error::Result,
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a new client
    let client = KrakenClient::default()?;
    
    // Create a new WebSocket API instance
    let mut ws_api = client.websocket();
    
    // Connect to the WebSocket API
    let mut message_rx = ws_api.connect().await?;
    
    // Subscribe to the ticker channel for BTC/USD
    let subscription = WebSocketSubscriptionRequest::new()
        .add_pair("XBT/USD")
        .add_subscription("ticker");
    
    ws_api.subscribe(subscription).await?;
    
    // Process messages for 10 seconds
    let start = std::time::Instant::now();
    while start.elapsed() < Duration::from_secs(10) {
        if let Some(message) = message_rx.recv().await {
            match message {
                Ok(WebSocketMessage::SubscriptionStatus { channel_id, channel_name, status, pair, subscription, .. }) => {
                    println!("Subscription status: {}", status);
                    if let Some(pair) = pair {
                        println!("Pair: {}", pair);
                    }
                    if let Some(channel_id) = channel_id {
                        println!("Channel ID: {}", channel_id);
                    }
                    if let Some(channel_name) = channel_name {
                        println!("Channel Name: {}", channel_name);
                    }
                    println!("Subscription: {:?}", subscription);
                }
                Ok(WebSocketMessage::DataArray(data)) => {
                    println!("Received data: {:?}", data);
                }
                Ok(msg) => {
                    println!("Received message: {:?}", msg);
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
    
    // Close the connection
    ws_api.close().await?;
    
    Ok(())
}
