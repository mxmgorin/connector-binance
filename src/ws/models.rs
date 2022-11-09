pub use crate::common::{Coin, Id, MarketType, OrderInfo, Side, Symbol, TradeInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum WsChannel {
    DepthOrderbook(String),
}

pub(crate) mod string_or_float {
    use std::fmt;

    use serde::{de, Serializer, Deserialize, Deserializer};

    pub fn serialize<T, S>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: fmt::Display,
        S: Serializer,
    {
        serializer.collect_str(value)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrFloat {
            String(String),
            Float(f64),
        }

        match StringOrFloat::deserialize(deserializer)? {
            StringOrFloat::String(s) => {
                if s == "INF" {
                    Ok(f64::INFINITY)
                } else {
                    s.parse().map_err(de::Error::custom)
                }
            }
            StringOrFloat::Float(i) => Ok(i),
        }
    }
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum WsDataEvent {
    DepthOrderbook(DepthOrderbookData),
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
    pub bids: Vec<Level>,

    #[serde(rename = "a")]
    pub asks: Vec<Level>,
}

#[derive(PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Level {
    #[serde(with = "string_or_float")]
    pub price: f64,
    #[serde(with = "string_or_float")]
    pub qty: f64,
}

impl Level {
    pub fn new(price: f64, qty: f64) -> Level {
        Level { price, qty }
    }
}