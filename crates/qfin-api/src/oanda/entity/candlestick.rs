use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::oanda::entity::de_decimal_number;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Candlestick {
    time: DateTime<Utc>,
    bid: Option<Data>,
    ask: Option<Data>,
    mid: Option<Data>,
    volume: i64,
    complete: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    #[serde(rename = "o", deserialize_with = "de_decimal_number")]
    open: f64,
    #[serde(rename = "h", deserialize_with = "de_decimal_number")]
    high: f64,
    #[serde(rename = "l", deserialize_with = "de_decimal_number")]
    low: f64,
    #[serde(rename = "c", deserialize_with = "de_decimal_number")]
    close: f64,
}
