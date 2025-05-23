use async_trait::async_trait;

pub mod client;
pub use client::Client;

pub mod entity;
pub use entity::Instrument;

use crate::Error;

pub const LIVE_URL: &str = "https://api-fxtrade.oanda.com";
pub const PRACTICE_URL: &str = "https://api-fxpractice.oanda.com";

#[async_trait]
pub trait DataGetter {
    async fn instruments(&self) -> Result<Vec<Instrument>, Error>;
}
