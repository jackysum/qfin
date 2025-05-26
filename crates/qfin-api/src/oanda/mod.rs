use async_trait::async_trait;

pub mod client;
pub use client::Client;

pub mod entity;
pub use entity::Candle;
pub use entity::Instrument;

pub mod request;
pub use request::CandlesRequest;

use crate::Error;

pub const LIVE_URL: &str = "https://api-fxtrade.oanda.com";
pub const PRACTICE_URL: &str = "https://api-fxpractice.oanda.com";

#[async_trait]
pub trait DataGetter {
    async fn candles(
        &self,
        instrument_name: &str,
        req: CandlesRequest,
    ) -> Result<Vec<Candle>, Error>;
    async fn instruments(&self) -> Result<Vec<Instrument>, Error>;
}
