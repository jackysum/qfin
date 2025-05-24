use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::oanda::entity::de_decimal_number;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Candle {
    pub time: DateTime<Utc>,
    pub bid: Option<Data>,
    pub ask: Option<Data>,
    pub mid: Option<Data>,
    pub volume: i64,
    pub complete: bool,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Data {
    #[serde(rename = "o", deserialize_with = "de_decimal_number")]
    pub open: f64,
    #[serde(rename = "h", deserialize_with = "de_decimal_number")]
    pub high: f64,
    #[serde(rename = "l", deserialize_with = "de_decimal_number")]
    pub low: f64,
    #[serde(rename = "c", deserialize_with = "de_decimal_number")]
    pub close: f64,
}
