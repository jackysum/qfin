use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("oanda api error: {0}")]
    OandaApi(String),
}
