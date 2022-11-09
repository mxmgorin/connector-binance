use my_web_socket_client::WebSocketClient;
use my_web_socket_client::WsCallback;
use my_web_socket_client::WsConnection;
use rust_extensions::Logger;
use serde_json::Error;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use super::binance_ws_settings::BinanceWsSetting;
use super::event_handler::*;
use super::models::*;
use tokio_tungstenite::tungstenite::Message;

pub struct BinanceWsClient {
    event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
    ws_client: WebSocketClient,
    logger: Arc<dyn Logger + Send + Sync + 'static>,
    is_started: AtomicBool,
}

impl BinanceWsClient {
    pub fn new(
        event_handler: Arc<dyn EventHandler + Send + Sync + 'static>,
        logger: Arc<dyn Logger + Send + Sync + 'static>,
        channels: Vec<WsChannel>,
    ) -> Self {
        let settings = Arc::new(BinanceWsSetting::new(channels));
        Self {
            event_handler,
            ws_client: WebSocketClient::new("Binance".to_string(), settings, logger.clone()),
            logger,
            is_started: AtomicBool::new(false),
        }
    }

    pub fn start(ftx_ws_client: Arc<BinanceWsClient>) {
        if !ftx_ws_client
            .is_started
            .load(std::sync::atomic::Ordering::Relaxed)
        {
            let ping_message = Message::Ping(vec!());
            ftx_ws_client
                .ws_client
                .start(ping_message, ftx_ws_client.clone());
            ftx_ws_client
                .is_started
                .store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }

    fn parse_msg(&self, msg: &str) -> Result<WsDataEvent, String> {
        let value: Result<serde_json::Value, Error> = serde_json::from_str(msg);

        if let Ok(value) = value {
            if let Some(data) = value.get("data") {
                return self.parse_msg(&data.to_string());
            }

            if let Ok(event) = serde_json::from_value::<WsDataEvent>(value) {
                return Ok(event);
            }
        }

        return Err(format!("Failed to parse message: {}", msg));
    }
}

#[async_trait::async_trait]
impl WsCallback for BinanceWsClient {
    async fn on_connected(&self, _: Arc<WsConnection>) {
        self.logger.write_info(
            "BinanceWsClient".to_string(),
            "Connected to Binance websocket".to_string(),
            None,
        );
    }

    async fn on_disconnected(&self, _: Arc<WsConnection>) {}

    async fn on_data(&self, connection: Arc<WsConnection>, data: Message) {
        match data {
            Message::Text(msg) => {
                let event = self.parse_msg(&msg);
                match event {
                    Ok(event) =>{
                        self.event_handler.on_data(event).await;
                    },
                    Err(err) => {
                        self.logger.write_info(
                            "BinanceWsClient".to_string(),
                            format!("Disconnecting... {} ", err),
                            None,
                        );
                        connection.disconnect().await;
                    }
                }               
            }
            Message::Ping(_) => (), // todo: send pong
            Message::Pong(_) | Message::Binary(_) | Message::Frame(_) => (),
            Message::Close(_) => {
                self.logger.write_info(
                    "BinanceWsClient".to_string(),
                    format!("Disconnecting... Recieved close ws message"),
                    None,
                );
            }
        }
    }
}
