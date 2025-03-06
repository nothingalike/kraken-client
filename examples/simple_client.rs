use kraken_client::{KrakenClient, Config, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a client with default configuration
    let client = KrakenClient::default()?;
    
    println!("Fetching server time...");
    let server_time = client.public().get_server_time().await?;
    println!("Server time: {}", server_time.rfc1123);
    
    println!("\nFetching asset pairs...");
    let asset_pairs = client.public().get_asset_pairs(Some(vec!["XBTUSD", "ETHUSD"])).await?;
    for (pair_name, pair_info) in asset_pairs {
        println!("Pair: {}", pair_name);
        println!("  Altname: {}", pair_info.altname);
        println!("  Base: {}", pair_info.base);
        println!("  Quote: {}", pair_info.quote);
        println!("  Pair decimals: {}", pair_info.pair_decimals);
        println!("  Lot decimals: {}", pair_info.lot_decimals);
        println!();
    }
    
    println!("Fetching ticker information for BTC/USD...");
    let ticker = client.public().get_ticker(vec!["XBTUSD"]).await?;
    if let Some(btc_ticker) = ticker.get("XXBTZUSD") {
        println!("BTC/USD Ticker:");
        println!("  Last trade: {}", btc_ticker.c[0]);
        println!("  Today's volume: {}", btc_ticker.v[0]);
        println!("  Today's high: {}", btc_ticker.h[0]);
        println!("  Today's low: {}", btc_ticker.l[0]);
        println!("  Today's opening price: {}", btc_ticker.o);
    }
    
    // Uncomment to use authenticated endpoints
    // Note: You need to set your API key and secret
    /*
    println!("\nSetting up authenticated client...");
    let config = Config::new()
        .with_api_key("your-api-key")
        .with_api_secret("your-api-secret");
    
    let auth_client = KrakenClient::new(config)?;
    
    println!("Fetching account balance...");
    match auth_client.private().get_balance().await {
        Ok(balance) => {
            println!("Account Balance:");
            for (asset, amount) in balance {
                println!("  {}: {}", asset, amount);
            }
        }
        Err(Error::Auth(msg)) => {
            println!("Authentication error: {}", msg);
            println!("Make sure to set your API key and secret correctly.");
        }
        Err(e) => {
            println!("Error fetching balance: {}", e);
        }
    }
    */
    
    Ok(())
}
