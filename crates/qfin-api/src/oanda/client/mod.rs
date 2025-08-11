use crate::oanda::Url;

pub mod instruments;

pub struct Client {
    client: reqwest::Client,
    account_id: String,
    auth_token: String,
    url: Url,
}

impl Client {
    pub fn new(client: reqwest::Client, account_id: &str, auth_token: &str, url: Url) -> Self {
        Client {
            client,
            account_id: account_id.to_string(),
            auth_token: auth_token.to_string(),
            url,
        }
    }
}
