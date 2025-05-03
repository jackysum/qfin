use async_trait::async_trait;

pub mod client;
pub use client::*;

pub mod instrument;
pub use instrument::Instrument;

pub const LIVE_REST_URL: &str = "https://api-fxtrade.oanda.com";
pub const PUBLIC_REST_URL: &str = "https://api-fxpractice.oanda.com";

#[async_trait]
pub trait OandaClient {
    async fn instruments(&self) -> Result<Vec<Instrument>, qfin_error::Error>;
}
