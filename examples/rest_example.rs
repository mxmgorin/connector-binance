use std::time::Duration;

use connector_binance::rest::{BinanceMarketClient, BinanceRestClient};

#[tokio::main]
async fn main() {
    let market: BinanceMarketClient = BinanceRestClient::new(None, None);

    loop {
        // Order book at default depth
        match market.get_depth("BNBETH").await {
            Ok(answer) => println!("Default depth: {:?}", answer),
            Err(e) => println!("Error: {}", e),
        }
        println!("=====================================================");

        tokio::time::sleep(Duration::from_secs(2)).await;

        // Order book at depth 500
        match market.get_custom_depth("BNBETH", 5).await {
            Ok(answer) => println!("Custom depth: {:?}", answer),
            Err(e) => println!("Error: {}", e),
        }
        println!("=====================================================");

        tokio::time::sleep(Duration::from_secs(2)).await;
    }
}
