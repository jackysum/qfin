use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE};
use serde::Deserialize;

use crate::{
    oanda::{DataGetter, Instrument},
    Error,
};

use super::{entity::Candle, CandlesRequest};

pub struct Client {
    client: reqwest::Client,
    account_id: String,
    auth_token: String,
    url: String,
}

impl Client {
    pub fn new(client: reqwest::Client, account_id: &str, auth_token: &str, url: &str) -> Self {
        Client {
            client: client.clone(),
            account_id: account_id.into(),
            auth_token: auth_token.into(),
            url: url.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
struct CandlesResponse {
    candles: Vec<Candle>,
}

#[derive(Debug, Deserialize)]
struct InstrumentsResponse {
    instruments: Vec<Instrument>,
}

#[async_trait]
impl DataGetter for Client {
    async fn candles(
        &self,
        instrument_name: &str,
        req: CandlesRequest,
    ) -> Result<Vec<Candle>, Error> {
        let url = req.url(&self.url, instrument_name)?;
        let resp = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token))
            .header(CONNECTION, "Keep-Alive")
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|e| Error::Request(e.to_string()))?;

        let resp = resp
            .error_for_status()
            .map_err(|e| Error::HttpStatus(e.status().unwrap()))?;

        let candles = resp
            .json::<CandlesResponse>()
            .await
            .map_err(|e| Error::DeserializeResponse(e.to_string()))?
            .candles;

        Ok(candles)
    }

    async fn instruments(&self) -> Result<Vec<Instrument>, Error> {
        let url = format!("{}/v3/accounts/{}/instruments", self.url, self.account_id);
        let resp = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token))
            .header(CONNECTION, "Keep-Alive")
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|e| Error::Request(e.to_string()))?;

        let resp = resp
            .error_for_status()
            .map_err(|e| Error::HttpStatus(e.status().unwrap()))?;

        let instruments = resp
            .json::<InstrumentsResponse>()
            .await
            .map_err(|e| Error::DeserializeResponse(e.to_string()))?
            .instruments;

        Ok(instruments)
    }
}

#[cfg(test)]
mod tests {
    use mockito::Server;
    use reqwest::header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE};

    use crate::{
        oanda::{
            entity::{candle, instrument, Candle},
            CandlesRequest, Client, DataGetter, Instrument,
        },
        Error,
    };

    #[tokio::test]
    async fn client_candles_success() {
        let body = r#"
            {
                "instrument": "EUR_USD",
                "granularity": "D",
                "candles": [
                    {
                        "complete": true,
                        "volume": 192447,
                        "time": "2025-05-22T21:00:00.000000000Z",
                        "bid": {
                            "o": "1.12792",
                            "h": "1.13752",
                            "l": "1.12782",
                            "c": "1.13621"
                        },
                        "mid": {
                            "o": "1.12807",
                            "h": "1.13759",
                            "l": "1.12790",
                            "c": "1.13649"
                        },
                        "ask": {
                            "o": "1.12822",
                            "h": "1.13766",
                            "l": "1.12798",
                            "c": "1.13677"
                        }
                    }
                ]
            }
        "#;

        let account_id = "account_id";
        let auth_token = "auth_token";
        let instrument_name = "EUR_USD";

        let mut server = Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/instruments/{}/candles?price=MBA", &instrument_name).as_str(),
            )
            .with_status(200)
            .with_header(AUTHORIZATION, &format!("Bearer {}", auth_token))
            .with_header(CONNECTION, "Keep-Alive")
            .with_header(CONTENT_TYPE, "application/json")
            .with_body(body)
            .create_async()
            .await;

        let want = Candle {
            complete: true,
            volume: 192447,
            time: "2025-05-22T21:00:00.000000000Z".parse().unwrap(),
            bid: Some(candle::Data {
                open: 1.12792,
                high: 1.13752,
                low: 1.12782,
                close: 1.13621,
            }),
            mid: Some(candle::Data {
                open: 1.12807,
                high: 1.13759,
                low: 1.12790,
                close: 1.13649,
            }),
            ask: Some(candle::Data {
                open: 1.12822,
                high: 1.13766,
                low: 1.12798,
                close: 1.13677,
            }),
        };

        let client = Client::new(reqwest::Client::new(), account_id, auth_token, &url);
        let got = client
            .candles(instrument_name, CandlesRequest {})
            .await
            .unwrap();
        let got = got.first().unwrap();

        assert_eq!(&want, got)
    }

    #[tokio::test]
    async fn client_candles_error_making_request() {
        let account_id = "account_id";
        let auth_token = "auth_token";
        let instrument_name = "EUR_USD";

        let mut server = Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/instruments/{}/candles?price=MBA", &instrument_name).as_str(),
            )
            .with_status(500)
            .create_async()
            .await;

        let client = Client::new(reqwest::Client::new(), account_id, auth_token, &url);
        let got = client.candles(instrument_name, CandlesRequest {}).await;

        assert!(got.is_err_and(|e| matches!(e, Error::HttpStatus(_))))
    }

    #[tokio::test]
    async fn client_candles_error_deserializing_response() {
        let account_id = "account_id";
        let auth_token = "auth_token";
        let instrument_name = "EUR_USD";

        let mut server = Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/instruments/{}/candles?price=MBA", &instrument_name).as_str(),
            )
            .with_status(200)
            .with_body("bad json response")
            .create_async()
            .await;

        let client = Client::new(reqwest::Client::new(), account_id, auth_token, &url);
        let got = client.candles(instrument_name, CandlesRequest {}).await;

        assert!(got.is_err_and(|e| matches!(e, Error::DeserializeResponse(_))))
    }

    #[tokio::test]
    async fn client_instruments_success() {
        let body = r#"
            {
                "instruments": [
                    {
"name": "EUR_USD",
                        "type": "CURRENCY",
                        "displayName": "EUR/USD",
                        "pipLocation": -4,
                        "displayPrecision": 5,
                        "tradeUnitsPrecision": 0,
                        "minimumTradeSize": "1",
                        "maximumTrailingStopDistance": "1.00000",
                        "minimumTrailingStopDistance": "0.00050",
                        "maximumPositionSize": "0",
                        "maximumOrderUnits": "100000000",
                        "marginRate": "0.02",
                        "guaranteedStopLossOrderMode": "DISABLED",
                        "tags": [
                            {
                                "type": "ASSET_CLASS",
                                "name": "CURRENCY"
                            },
                            {
                                "type": "BRAIN_ASSET_CLASS",
                                "name": "FX"
                            }
                        ],
                        "financing": {
                            "longRate": "-0.0318",
                            "shortRate": "0.0115",
                            "financingDaysOfWeek": [
                                {
                                    "dayOfWeek": "MONDAY",
                                    "daysCharged": 1
                                },
                                {
                                    "dayOfWeek": "TUESDAY",
                                    "daysCharged": 1
                                },
                                {
                                    "dayOfWeek": "WEDNESDAY",
                                    "daysCharged": 1
                                },
                                {
                                    "dayOfWeek": "THURSDAY",
                                    "daysCharged": 1
                                },
                                {
                                    "dayOfWeek": "FRIDAY",
                                    "daysCharged": 1
                                },
                                {
                                    "dayOfWeek": "SATURDAY",
                                    "daysCharged": 0
                                },
                                {
                                    "dayOfWeek": "SUNDAY",
                                    "daysCharged": 0
                                }
                            ]
                        }
                    }
                ],
                "lastTransactionID": "164"
            }
        "#;

        let account_id = "account_id";
        let auth_token = "auth_token";

        let mut server = Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", account_id).as_str(),
            )
            .with_status(200)
            .with_header(AUTHORIZATION, &format!("Bearer {}", auth_token))
            .with_header(CONNECTION, "Keep-Alive")
            .with_header(CONTENT_TYPE, "application/json")
            .with_body(body)
            .create_async()
            .await;

        let want = Instrument {
            name: "EUR_USD".to_string(),
            instrument_type: instrument::Type::Currency,
            display_name: "EUR/USD".to_string(),
            pip_location: -4,
            display_precision: 5,
            trade_units_precision: 0,
            minimum_trade_size: 1.0,
            maximum_trailing_stop_distance: 1.0,
            minimum_trailing_stop_distance: 0.00050,
            maximum_position_size: 0.0,
            maximum_order_units: 100000000.0,
            margin_rate: 0.02,
            guaranteed_stop_loss_order_mode: instrument::GuaranteedStopLossOrderMode::Disabled,
            tags: vec![
                instrument::Tag {
                    tag_type: "ASSET_CLASS".to_string(),
                    name: "CURRENCY".to_string(),
                },
                instrument::Tag {
                    tag_type: "BRAIN_ASSET_CLASS".to_string(),
                    name: "FX".to_string(),
                },
            ],
            financing: instrument::Financing {
                long_rate: -0.0318,
                short_rate: 0.0115,
                financing_days_of_week: vec![
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Monday,
                        days_charged: 1,
                    },
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Tuesday,
                        days_charged: 1,
                    },
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Wednesday,
                        days_charged: 1,
                    },
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Thursday,
                        days_charged: 1,
                    },
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Friday,
                        days_charged: 1,
                    },
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Saturday,
                        days_charged: 0,
                    },
                    instrument::FinancingDayOfWeek {
                        day_of_week: instrument::DayOfWeek::Sunday,
                        days_charged: 0,
                    },
                ],
            },
        };

        let client = Client::new(reqwest::Client::new(), account_id, auth_token, &url);
        let got = client.instruments().await.unwrap();
        let got = got.first().unwrap();

        assert_eq!(&want, got)
    }

    #[tokio::test]
    async fn client_instruments_error_making_request() {
        let account_id = "account_id";
        let auth_token = "auth_token";

        let mut server = Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", account_id).as_str(),
            )
            .with_status(500)
            .create_async()
            .await;

        let client = Client::new(reqwest::Client::new(), account_id, auth_token, &url);
        let got = client.instruments().await;

        assert!(got.is_err_and(|e| matches!(e, Error::HttpStatus(_))))
    }

    #[tokio::test]
    async fn client_instruments_error_deserializing_response() {
        let account_id = "account_id";
        let auth_token = "auth_token";

        let mut server = Server::new_async().await;
        let url = server.url();

        server
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", account_id).as_str(),
            )
            .with_status(200)
            .with_body("bad json response")
            .create_async()
            .await;

        let client = Client::new(reqwest::Client::new(), account_id, auth_token, &url);
        let got = client.instruments().await;

        assert!(got.is_err_and(|e| matches!(e, Error::DeserializeResponse(_))))
    }
}
