use crate::oanda::{Api, Url};

pub struct Client {
    client: reqwest::Client,
    account_id: String,
    auth_token: String,
    url: Url,
}

impl Client {
    pub fn new(client: reqwest::Client, account_id: String, auth_token: String, url: Url) -> Self {
        Client {
            client,
            account_id,
            auth_token,
            url,
        }
    }
}

impl Api for Client {}
