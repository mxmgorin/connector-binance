use super::WsChannel;
use my_web_socket_client::WsClientSettings;
use std::sync::Arc;

#[async_trait::async_trait]
pub trait BinanceWsSetting {
    async fn get_channels(&self) -> Vec<WsChannel>;
}

pub(crate) struct BinanceWsSettingWrapper {
    inner: Arc<dyn BinanceWsSetting + Send + Sync>,
}

impl BinanceWsSettingWrapper {
    pub fn new(inner: Arc<dyn BinanceWsSetting + Send + Sync>) -> Self {
        Self { inner }
    }

    pub async fn get_endpoints(&self) -> Vec<String> {
        let channels = self.inner.get_channels().await;

        let mut endpoints: Vec<String> = Vec::new();

        for channel in channels.into_iter() {
            match channel {
                WsChannel::DepthOrderbook(symbol) => {
                    endpoints.push(format!("{}@depth@100ms", symbol.to_lowercase()))
                }
                WsChannel::BookTicker(symbol) => {
                    endpoints.push(format!("{}@bookTicker", symbol.to_lowercase()))
                }
                WsChannel::Kline(symbol, interval) => {
                    endpoints.push(format!("{}@kline_{}", symbol.to_lowercase(), interval));
                }
            }
        }

        endpoints
    }
}

#[async_trait::async_trait]
impl WsClientSettings for BinanceWsSettingWrapper {
    async fn get_url(&self, _client_name: &str) -> Option<String> {
        let endpoints = self.get_endpoints().await;

        Some(BinanceWsUrl::MultiStream.params(&endpoints.join("/")))
    }
}

#[allow(clippy::all)]
enum BinanceWsUrl {
    MultiStream,
}

impl BinanceWsUrl {
    pub fn params(self, subscription: &str) -> String {
        match self {
            //BinanceWsUrl::Default => format!("wss://stream.binance.com:9443/ws/{}", subscription),
            BinanceWsUrl::MultiStream => format!(
                "wss://stream.binance.com:9443/stream?streams={}",
                subscription
            ),
        }
    }
}
