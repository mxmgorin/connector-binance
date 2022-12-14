use my_web_socket_client::WsClientSettings;

use super::WsChannel;

pub struct BinanceWsSetting {
    endpoints: Vec<String>,
}

impl BinanceWsSetting {
    pub fn new(channels: Vec<WsChannel>) -> Self {
        let mut endpoints: Vec<String> = Vec::new();

        for channel in channels.into_iter() {
            match channel {
                WsChannel::DepthOrderbook(symbol) => {
                    endpoints.push(format!("{}@depth@100ms", symbol.to_lowercase()))
                }
                WsChannel::BookTicker(symbol) => {
                    endpoints.push(format!("{}@bookTicker", symbol.to_lowercase()))
                }
            }
        }
        Self { endpoints }
    }
}

#[async_trait::async_trait]
impl WsClientSettings for BinanceWsSetting {
    async fn get_url(&self) -> String {
        return BinanceWsUrl::MultiStream.params(&self.endpoints.join("/"));
    }
}

#[allow(clippy::all)]
enum BinanceWsUrl {
    MultiStream,
}

impl BinanceWsUrl {
    fn params(self, subscription: &str) -> String {
        match self {
            //BinanceWsUrl::Default => format!("wss://stream.binance.com:9443/ws/{}", subscription),
            BinanceWsUrl::MultiStream => format!(
                "wss://stream.binance.com:9443/stream?streams={}",
                subscription
            ),
        }
    }
}
