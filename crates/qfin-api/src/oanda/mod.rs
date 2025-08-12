use std::fmt;

use async_trait::async_trait;

pub mod client;
pub use client::Client;

pub mod instrument;
pub use instrument::*;

use crate::Error;

#[async_trait]
pub trait Api {
    async fn instruments(&self) -> Result<Vec<Instrument>, Error>;
}

pub enum Url {
    Custom(String),
    Live,
    Practice,
}

impl fmt::Display for Url {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Url::Custom(s) => write!(f, "{}", s),
            Url::Live => write!(f, "https://api-fxtrade.oanda.com"),
            Url::Practice => write!(f, "https://api-fxpractice.oanda.com"),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::oanda::Url;

    #[test]
    fn test_url_to_string_custom() {
        let want = "https://example.com";
        let custom_url = want.to_string();

        assert_eq!(want, Url::Custom(custom_url).to_string())
    }

    #[test]
    fn test_url_to_string_live() {
        let want = "https://api-fxtrade.oanda.com";
        assert_eq!(want, Url::Live.to_string())
    }

    #[test]
    fn test_url_to_string_practive() {
        let want = "https://api-fxpractice.oanda.com";
        assert_eq!(want, Url::Practice.to_string())
    }
}
