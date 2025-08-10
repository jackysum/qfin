pub mod client;
pub use client::Client;

pub trait Api {}

pub enum Url {
    Live,
    Practice,
}

impl Url {
    pub fn value(&self) -> &str {
        match *self {
            Url::Live => "https://api-fxtrade.oanda.com",
            Url::Practice => "https://api-fxpractice.oanda.com",
        }
    }
}
