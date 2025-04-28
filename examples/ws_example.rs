use connector_binance::ws::{
    BinanceWsClient, BinanceWsSetting, EventHandler, KlineInterval, WsChannel, WsDataEvent,
};
use rust_extensions::Logger;
use std::collections::HashMap;
use std::{sync::Arc, time::Duration};

#[derive(Default)]
pub struct ExampleEventHandler {}

#[async_trait::async_trait]
impl EventHandler for ExampleEventHandler {
    async fn on_data(&self, event: WsDataEvent) {
        match event {
            WsDataEvent::DepthOrderbook(data) => {
                println!("Received DepthOrderbook:");
                println!("{:?}", data);
                println!("-------------------------------");
            }
            WsDataEvent::BookTicker(data) => {
                println!("Received BookTicker:");
                println!("{:?}", data);
                println!("-------------------------------");
            }
            WsDataEvent::Kline(data) => {
                println!("Received Kline:");
                println!("{:?}", data);
                println!("-------------------------------");
            }
        }
    }

    async fn on_connected(&self) {
        println!("Connected to ws");
    }
}

#[derive(Default)]
pub struct ExampleLogger {}

impl Logger for ExampleLogger {
    fn write_info(
        &self,
        _process: String,
        _message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
    }

    fn write_warning(
        &self,
        _process: String,
        _message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
    }

    fn write_error(
        &self,
        _process: String,
        _message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
    }

    fn write_fatal_error(
        &self,
        _process: String,
        _message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
    }

    fn write_debug_info(
        &self,
        _process: String,
        _message: String,
        _ctx: Option<HashMap<String, String>>,
    ) {
    }
}

pub struct ExampleSettings {
    pub channels: Vec<WsChannel>,
}

#[async_trait::async_trait]
impl BinanceWsSetting for ExampleSettings {
    async fn get_channels(&self) -> Vec<WsChannel> {
        self.channels.clone()
    }
}

#[tokio::main]
async fn main() {
    let channels = vec![
        //WsChannel::BookTicker("ethbtc".to_owned()),
        //WsChannel::DepthOrderbook("ethbtc".to_owned()),
        WsChannel::Kline("ethbtc".to_owned(), KlineInterval::OneMinute),
    ];
    let settings = ExampleSettings { channels };
    let ws_client = BinanceWsClient::new(
        Arc::new(ExampleEventHandler::default()),
        Arc::new(ExampleLogger::default()),
        Arc::new(settings),
    );

    BinanceWsClient::start(Arc::new(ws_client));

    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
