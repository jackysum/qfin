use serde::Deserialize;

use crate::oanda::{Candle, Instrument};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candles {
    pub candles: Vec<Candle>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruments {
    pub instruments: Vec<Instrument>,
}
