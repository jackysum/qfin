use async_trait::async_trait;
use reqwest::header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE};

use crate::oanda::{request, response, Candle, Instrument, OandaClient};

#[derive(Clone)]
pub struct Client {
    client: reqwest::Client,
    account_id: String,
    auth_token: String,
    url: String,
}

impl Client {
    pub fn new(
        client: reqwest::Client,
        account_id: String,
        auth_token: String,
        url: String,
    ) -> Self {
        Client {
            client,
            account_id,
            auth_token,
            url,
        }
    }

    async fn get(&self, url: String) -> Result<reqwest::Response, qfin_error::Error> {
        let resp = self
            .client
            .get(url)
            .header(AUTHORIZATION, format!("Bearer {}", self.auth_token))
            .header(CONNECTION, "Keep-Alive")
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await
            .map_err(|err| qfin_error::Error::OandaApi(err.to_string()))?;

        Ok(resp)
    }
}

#[async_trait]
impl OandaClient for Client {
    async fn candles(
        &self,
        instrument_name: &str,
        req: Option<request::Candles>,
    ) -> Result<Vec<Candle>, qfin_error::Error> {
        let req = req.unwrap_or(request::Candles::default());
        let url = req.url(self.url.as_str(), instrument_name)?;

        let resp = self
            .get(url)
            .await?
            .json::<response::Candles>()
            .await
            .map_err(|err| {
                qfin_error::Error::OandaApi(format!("deserializing candles response: {}", err))
            })?;

        Ok(resp.candles)
    }

    async fn instruments(&self) -> Result<Vec<Instrument>, qfin_error::Error> {
        let url = format!("{}/v3/accounts/{}/instruments", self.url, self.account_id);
        let resp = self
            .get(url)
            .await?
            .json::<response::Instruments>()
            .await
            .map_err(|err| {
                qfin_error::Error::OandaApi(format!("deserializing instruments response: {}", err))
            })?;

        Ok(resp.instruments)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use reqwest::header::{AUTHORIZATION, CONNECTION, CONTENT_TYPE};

    use crate::oanda::{
        candle,
        instrument::{
            DayOfWeek, Financing, FinancingDayOfWeek, GuaranteedStopLossOrderMode, Tag, Type,
        },
        request, Candle, Client, Instrument, OandaClient,
    };

    const ACCOUNT_ID: &str = "account_id";
    const AUTH_TOKEN: &str = "auth_token";
    const INSTRUMENT_NAME: &str = "EUR_USD";

    #[tokio::test]
    async fn test_candles() {
        let mut svr = mockito::Server::new_async().await;
        let url = svr.url();
        let body = r#"
            {
                "candles": [
                    {
                        "complete": true,
                        "volume": 161938,
                        "time": "2025-05-04T21:00:00.000000000Z",
                        "bid": {
                            "o": "1.13013",
                            "h": "1.13643",
                            "l": "1.12922",
                            "c": "1.13140"
                        },
                        "mid": {
                            "o": "1.13062",
                            "h": "1.13650",
                            "l": "1.12962",
                            "c": "1.13150"
                        },
                        "ask": {
                            "o": "1.13110",
                            "h": "1.13658",
                            "l": "1.12980",
                            "c": "1.13159"
                        }
                    }
                ]
            }
        "#;

        let mock = svr
            .mock(
                "GET",
                format!("/v3/instruments/{}/candles", INSTRUMENT_NAME).as_str(),
            )
            .match_header(AUTHORIZATION, format!("Bearer {}", AUTH_TOKEN).as_str())
            .match_header(CONNECTION, "Keep-Alive")
            .match_header(CONTENT_TYPE, "application/json")
            .with_status(200)
            .with_body(body)
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID.to_string(),
            AUTH_TOKEN.to_string(),
            url,
        );

        let got = client
            .candles(INSTRUMENT_NAME, Some(request::Candles::default()))
            .await
            .unwrap();
        let got_candle = got.first().unwrap();

        let want = Candle {
            time: DateTime::parse_from_rfc3339("2025-05-04T21:00:00.000000000Z")
                .unwrap()
                .with_timezone(&Utc),
            bid: Some(candle::Data {
                open: 1.13013,
                high: 1.13643,
                low: 1.12922,
                close: 1.13140,
            }),
            mid: Some(candle::Data {
                open: 1.13062,
                high: 1.13650,
                low: 1.12962,
                close: 1.13150,
            }),
            ask: Some(candle::Data {
                open: 1.13110,
                high: 1.13658,
                low: 1.12980,
                close: 1.13159,
            }),
            volume: 161938,
            complete: true,
        };

        assert_eq!(&want, got_candle);
        mock.assert()
    }

    #[tokio::test]
    async fn test_candles_response_not_success_error() {
        let mut svr = mockito::Server::new_async().await;
        let url = svr.url();

        let mock = svr
            .mock(
                "GET",
                format!("/v3/instruments/{}/candles", INSTRUMENT_NAME).as_str(),
            )
            .with_status(500)
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID.to_string(),
            AUTH_TOKEN.to_string(),
            url,
        );

        let got = client.candles(INSTRUMENT_NAME, None).await;
        assert!(got.is_err());

        mock.assert()
    }

    #[tokio::test]
    async fn test_candles_response_body_deserialization_error() {
        let mut svr = mockito::Server::new_async().await;
        let url = svr.url();

        let mock = svr
            .mock(
                "GET",
                format!("/v3/instruments/{}/candles", INSTRUMENT_NAME).as_str(),
            )
            .with_status(200)
            .with_body("bad json")
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID.to_string(),
            AUTH_TOKEN.to_string(),
            url,
        );

        let got = client.candles(INSTRUMENT_NAME, None).await;
        assert!(got.is_err());

        mock.assert()
    }

    #[tokio::test]
    async fn test_instruments() {
        let mut svr = mockito::Server::new_async().await;
        let url = svr.url();
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
                            "longRate": "-0.0323",
                            "shortRate": "0.012",
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

        let mock = svr
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", ACCOUNT_ID).as_str(),
            )
            .match_header(AUTHORIZATION, format!("Bearer {}", AUTH_TOKEN).as_str())
            .match_header(CONNECTION, "Keep-Alive")
            .match_header(CONTENT_TYPE, "application/json")
            .with_status(200)
            .with_body(body)
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID.to_string(),
            AUTH_TOKEN.to_string(),
            url,
        );

        let got = client.instruments().await.unwrap();
        let got_instrument = got.first().unwrap();

        let want = Instrument {
            name: "EUR_USD".to_string(),
            instrument_type: Type::Currency,
            display_name: "EUR/USD".to_string(),
            pip_location: -4,
            display_precision: 5,
            trade_units_precision: 0,
            minimum_trade_size: 1.0,
            maximum_trailing_stop_distance: 1.00000,
            minimum_trailing_stop_distance: 0.00050,
            maximum_position_size: 0.0,
            maximum_order_units: 100000000.0,
            margin_rate: 0.02,
            guaranteed_stop_loss_order_mode: GuaranteedStopLossOrderMode::Disabled,
            tags: vec![
                Tag {
                    tag_type: "ASSET_CLASS".to_string(),
                    name: "CURRENCY".to_string(),
                },
                Tag {
                    tag_type: "BRAIN_ASSET_CLASS".to_string(),
                    name: "FX".to_string(),
                },
            ],
            financing: Financing {
                long_rate: -0.0323,
                short_rate: 0.012,
                financing_days_of_week: vec![
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Monday,
                        days_charged: 1,
                    },
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Tuesday,
                        days_charged: 1,
                    },
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Wednesday,
                        days_charged: 1,
                    },
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Thursday,
                        days_charged: 1,
                    },
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Friday,
                        days_charged: 1,
                    },
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Saturday,
                        days_charged: 0,
                    },
                    FinancingDayOfWeek {
                        day_of_week: DayOfWeek::Sunday,
                        days_charged: 0,
                    },
                ],
            },
        };

        assert_eq!(&want, got_instrument);
        mock.assert()
    }

    #[tokio::test]
    async fn test_instruments_response_not_success_error() {
        let mut svr = mockito::Server::new_async().await;
        let url = svr.url();

        let mock = svr
            .mock(
                "GET",
                format!("/v3/accounts/{}/instruments", ACCOUNT_ID).as_str(),
            )
            .with_status(500)
            .create_async()
            .await;

        let client = Client::new(
            reqwest::Client::new(),
            ACCOUNT_ID.to_string(),
            AUTH_TOKEN.to_string(),
            url,
        );

        let got = client.instruments().await;
        assert!(got.is_err());

        mock.assert()
    }

    #[tokio::test]
    async fn test_instruments_response_body_deserialization_error() {
        let mut svr = mockito::Server::new_async().await;
        let url = svr.url();

        let mock = svr
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
            ACCOUNT_ID.to_string(),
            AUTH_TOKEN.to_string(),
            url,
        );

        let got = client.instruments().await;
        assert!(got.is_err());

        mock.assert()
    }
}
