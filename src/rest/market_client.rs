use super::{
    enpoints::{BinanceApiEndpoint, BinanceSpotEnpoint},
    rest_client::RestClient,
    models::OrderbookSnapshot,
    util::build_request,
};
use crate::rest::errors::Result;
use std::collections::BTreeMap;

#[derive(Clone)]
pub struct BinanceMarketClient {
    pub client: RestClient,
    pub recv_window: u64,
}

// Market Data endpoints
impl BinanceMarketClient {
    // Order book at the default depth of 100
    pub async fn get_depth<S>(&self, symbol: S) -> Result<OrderbookSnapshot>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        let request = build_request(parameters);
        self.client.get(
            BinanceApiEndpoint::Spot(BinanceSpotEnpoint::Depth),
            Some(request),
        ).await
    }

    // Order book at a custom depth. Currently supported values
    // are 5, 10, 20, 50, 100, 500, 1000 and 5000
    pub async fn get_custom_depth<S>(&self, symbol: S, depth: u64) -> Result<OrderbookSnapshot>
    where
        S: Into<String>,
    {
        let mut parameters: BTreeMap<String, String> = BTreeMap::new();
        parameters.insert("symbol".into(), symbol.into());
        parameters.insert("limit".into(), depth.to_string());
        let request = build_request(parameters);
        self.client.get(
            BinanceApiEndpoint::Spot(BinanceSpotEnpoint::Depth),
            Some(request),
        ).await
    }
}
