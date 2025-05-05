use async_trait::async_trait;

pub mod candle;
pub use candle::Candle;

pub mod client;
pub use client::*;

pub mod instrument;
pub use instrument::Instrument;

pub mod request;
mod response;

pub const LIVE_REST_URL: &str = "https://api-fxtrade.oanda.com";
pub const PRACTICE_REST_URL: &str = "https://api-fxpractice.oanda.com";

#[async_trait]
pub trait OandaClient {
    async fn candles(
        &self,
        instrument_name: &str,
        req: Option<request::Candles>,
    ) -> Result<Vec<Candle>, qfin_error::Error>;
    async fn instruments(&self) -> Result<Vec<Instrument>, qfin_error::Error>;
}
