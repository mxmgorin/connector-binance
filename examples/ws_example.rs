use std::{sync::Arc, time::Duration};

use connector_binance::ws::{
    EventHandler, BinanceWsClient, WsChannel, WsDataEvent
};
use rust_extensions::Logger;

pub struct OrderBookHandler {}

impl OrderBookHandler {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait::async_trait]
impl EventHandler for OrderBookHandler {
    async fn on_data(&self, event: WsDataEvent) {
        match event {
            WsDataEvent::DepthOrderbook(orderbook_data) => {
                println!("Recieved orderbook:");
                println!("{:?}", orderbook_data);
                println!("-------------------------------");
            },
        }
    }

    async fn on_connected(&self) {
        println!("Connected to ws");
    }
}

pub struct ConsoleLogger {}

impl Logger for ConsoleLogger {
    fn write_info(&self, _process: String, _message: String, _ctx: Option<std::collections::HashMap<String, String>>) {
        
    }

    fn write_warning(&self, _process: String, _message: String, _ctx: Option<std::collections::HashMap<String, String>>) {
    }

    fn write_error(&self, _process: String,_messagee: String, _ctx: Option<std::collections::HashMap<String, String>>) {
    }

    fn write_fatal_error(
        &self,
        _process: String,
        _message: String,
        _ctx: Option<std::collections::HashMap<String, String>>,
    ) {
    }
}

#[tokio::main]
async fn main() {
    let channels = vec![
        WsChannel::DepthOrderbook("ethbtc".to_owned()),
        //WsChannel::DepthOrderbook("ETH/USD".to_owned()),
    ];
    let event_handler = Arc::new(OrderBookHandler {});
    let ftx_ws = BinanceWsClient::new(
        event_handler,
        Arc::new(ConsoleLogger{}),
        channels,
    );

    BinanceWsClient::start(Arc::new(ftx_ws));


    loop {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
