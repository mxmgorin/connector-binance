pub use crate::common::{Coin, Id, MarketType, OrderInfo, Side, Symbol, TradeInfo};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum WsChannel {
    DepthOrderbook(String),
    BookTicker(String),
    Kline(String, KlineInterval),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WsDataEvent {
    DepthOrderbook(DepthOrderbookData),
    BookTicker(BookTickerData),
    Kline(KlineData)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BookTickerData {
    #[serde(rename = "u")]
    pub update_id: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "b")]
    pub best_bid: String,

    #[serde(rename = "B")]
    pub best_bid_qty: String,

    #[serde(rename = "a")]
    pub best_ask: String,

    #[serde(rename = "A")]
    pub best_ask_qty: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum OrderbookEventType {
    DepthUpdate,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DepthOrderbookData {
    #[serde(rename = "e")]
    pub event_type: OrderbookEventType,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "U")]
    pub first_update_id: u64,

    #[serde(rename = "u")]
    pub final_update_id: u64,

    #[serde(rename = "pu")]
    #[serde(default)]
    pub previous_final_update_id: Option<u64>,

    #[serde(rename = "b")]
    pub bids: Vec<(Decimal, Decimal)>,

    #[serde(rename = "a")]
    pub asks: Vec<(Decimal, Decimal)>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum KlineInterval {
    #[serde(rename = "1s")]
    OneSecond,
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifteenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1M")]
    OneMonth,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct KlineData {
    #[serde(rename = "e")]
    pub event_type: String,

    #[serde(rename = "E")]
    pub event_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "k")]
    pub kline: Kline,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Kline {
    #[serde(rename = "t")]
    pub start_time: u64,

    #[serde(rename = "T")]
    pub close_time: u64,

    #[serde(rename = "s")]
    pub symbol: String,

    #[serde(rename = "i")]
    pub interval: String,

    #[serde(rename = "f")]
    pub first_trade_id: u64,

    #[serde(rename = "L")]
    pub last_trade_id: u64,

    #[serde(rename = "o")]
    pub open_price: String,

    #[serde(rename = "c")]
    pub close_price: String,

    #[serde(rename = "h")]
    pub high_price: String,

    #[serde(rename = "l")]
    pub low_price: String,

    #[serde(rename = "v")]
    pub base_volume: String,

    #[serde(rename = "n")]
    pub number_of_trades: u64,

    #[serde(rename = "x")]
    pub is_closed: bool,

    #[serde(rename = "q")]
    pub quote_volume: String,

    #[serde(rename = "V")]
    pub taker_buy_base_volume: String,

    #[serde(rename = "Q")]
    pub taker_buy_quote_volume: String,

    #[serde(rename = "B")]
    pub ignore: String,
}

impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            KlineInterval::OneSecond => "1s",
            KlineInterval::OneMinute => "1m",
            KlineInterval::ThreeMinutes => "3m",
            KlineInterval::FiveMinutes => "5m",
            KlineInterval::FifteenMinutes => "15m",
            KlineInterval::ThirtyMinutes => "30m",
            KlineInterval::OneHour => "1h",
            KlineInterval::TwoHours => "2h",
            KlineInterval::FourHours => "4h",
            KlineInterval::SixHours => "6h",
            KlineInterval::EightHours => "8h",
            KlineInterval::TwelveHours => "12h",
            KlineInterval::OneDay => "1d",
            KlineInterval::ThreeDays => "3d",
            KlineInterval::OneWeek => "1w",
            KlineInterval::OneMonth => "1M",
        };
        write!(f, "{}", s)
    }
}