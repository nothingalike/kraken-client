# Kraken API Client

A Rust client library for the [Kraken cryptocurrency exchange API](https://docs.kraken.com/rest/).

## Features

- Complete API coverage for both public and private endpoints
- WebSocket support for real-time data
- Strongly typed request and response models
- Async-first design using Tokio
- Builder pattern for fluent API
- Comprehensive error handling
- Rate limiting to avoid API throttling
- Optional blocking API support

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
kraken_client = "0.1.0"
```

## Usage

### Basic Example

```rust
use kraken_client::{KrakenClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with default configuration
    let client = KrakenClient::default()?;
    
    // Get server time
    let server_time = client.public().get_server_time().await?;
    println!("Server time: {}", server_time.rfc1123);
    
    Ok(())
}
```

### Authentication

```rust
use kraken_client::{KrakenClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with API key and secret
    let config = Config::new()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret");
    
    let client = KrakenClient::new(config)?;
    
    // Get account balance
    let balance = client.private().get_balance().await?;
    println!("Balance: {:?}", balance);
    
    Ok(())
}
```

### Market Data

```rust
use kraken_client::{KrakenClient, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::default()?;
    
    // Get ticker information for BTC/USD
    let ticker = client.public().get_ticker(vec!["XBTUSD"]).await?;
    println!("Ticker: {:?}", ticker);
    
    // Get order book for BTC/USD
    let orderbook = client.public().get_orderbook("XBTUSD", Some(10)).await?;
    println!("Orderbook: {:?}", orderbook);
    
    // Get recent trades for BTC/USD
    let (trades, last) = client.public().get_trades("XBTUSD", None).await?;
    println!("Trades: {:?}", trades);
    println!("Last: {}", last);
    
    Ok(())
}
```

### Trading

```rust
use kraken_client::{KrakenClient, Config};
use kraken_client::models::trading::{Order, OrderSide, OrderType};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::new()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret");
    
    let client = KrakenClient::new(config)?;
    
    // Create a limit order to buy 0.01 BTC at $30,000
    let order = Order::new("XBTUSD", OrderSide::Buy, OrderType::Limit, "0.01")
        .with_price("30000")
        .with_validate(true); // Validate only, don't submit
    
    // Add the order
    let response = client.private().add_order(&order).await?;
    println!("Order response: {:?}", response);
    
    Ok(())
}
```

### WebSocket

```rust
use kraken_client::{KrakenClient, Config};
use kraken_client::models::websocket::{WebSocketSubscriptionRequest, WebSocketSubscriptionType, WebSocketMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = KrakenClient::default()?;
    
    // Create a WebSocket API instance
    let mut ws = client.websocket();
    
    // Connect to the WebSocket API
    ws.connect().await?;
    
    // Subscribe to ticker for BTC/USD
    let request = WebSocketSubscriptionRequest::new(WebSocketSubscriptionType::Ticker)
        .with_pairs(vec!["XBT/USD".to_string()]);
    
    ws.subscribe(request).await?;
    
    // Receive messages
    while let Ok(message) = ws.receive().await {
        match message {
            WebSocketMessage::DataArray(data) => {
                println!("Received data: {:?}", data);
            }
            _ => {
                println!("Received message: {:?}", message);
            }
        }
    }
    
    Ok(())
}
```

## API Documentation

For detailed API documentation, please refer to the [Kraken API documentation](https://docs.kraken.com/rest/).

## License

This project is licensed under the MIT License - see the LICENSE file for details.
