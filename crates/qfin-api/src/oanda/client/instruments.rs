use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde::Deserialize;

use crate::{
    oanda::{Api, Client, Instrument},
    Error,
};

#[derive(Deserialize)]
struct InstrumentsData {
    instruments: Vec<Instrument>,
}

#[async_trait]
impl Api for Client {
    async fn instruments(&self) -> Result<Vec<Instrument>, Error> {
        let url = format!(
            "{}/v3/accounts/{}/instruments",
            self.url.to_string(),
            self.account_id
        );

        let resp = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|err| Error::Request(err))?;

        if resp.status() != reqwest::StatusCode::OK {
            return Err(Error::StatusNotOK(resp.status()));
        }

        let data = resp
            .json::<InstrumentsData>()
            .await
            .map_err(|err| Error::Deserialize(err))?;

        Ok(data.instruments)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

    use crate::{
        oanda::{
            fixtures::{instruments, instruments_json},
            Api, Client, Url,
        },
        Error,
    };

    static ACCOUNT_ID: &str = "account_id";
    static AUTH_TOKEN: &str = "auth_token";

    #[tokio::test]
    async fn test_instruments() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        let mock = server
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", ACCOUNT_ID).as_str(),
            )
            .with_header(AUTHORIZATION, format!("Bearer {}", AUTH_TOKEN).as_str())
            .with_header(CONTENT_TYPE, "application/json")
            .with_status(200)
            .with_body(format!(r#"{{"instruments": {}}}"#, instruments_json()))
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID,
            AUTH_TOKEN,
            Url::Custom(url),
        );

        let results = client.instruments().await.unwrap();
        let instruments = instruments();

        assert_eq!(instruments.len(), results.len());

        mock.assert();
    }

    #[tokio::test]
    async fn test_instruments_status_not_ok_error() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", ACCOUNT_ID).as_str(),
            )
            .with_status(500)
            .with_body("Internal Server Error")
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID,
            AUTH_TOKEN,
            Url::Custom(url),
        );

        let err = client.instruments().await;
        assert!(err.is_err_and(|err| matches!(
            err,
            Error::StatusNotOK(reqwest::StatusCode::INTERNAL_SERVER_ERROR)
        )))
    }

    #[tokio::test]
    async fn test_instruments_deserialize_error() {
        let mut server = mockito::Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", ACCOUNT_ID).as_str(),
            )
            .with_status(200)
            .with_body("bad json")
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID,
            AUTH_TOKEN,
            Url::Custom(url),
        );

        let err = client.instruments().await;
        assert!(err.is_err_and(|err| matches!(err, Error::Deserialize(_),)))
    }
}
