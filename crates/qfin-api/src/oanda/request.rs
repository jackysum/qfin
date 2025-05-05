use chrono::{DateTime, Utc};
use qfin_error::Error;
use reqwest::Url;

use crate::oanda::candle::{Granularity, WeeklyAlignment};

#[derive(Debug)]
pub struct Candles {
    pub price: String,
    pub granularity: Granularity,
    pub count: i16,
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub smooth: bool,
    pub include_first: bool,
    pub daily_alignment: i16,
    pub alignment_timezone: String,
    pub weekly_alignment: WeeklyAlignment,
}

impl Candles {
    pub fn url(&self, base_url: &str, instrument_name: &str) -> Result<String, Error> {
        let mut url =
            Url::parse(format!("{}/v3/instruments/{}/candles", base_url, instrument_name).as_str())
                .map_err(|err| Error::OandaApi(err.to_string()))?;

        let default = Candles::default();

        if self.price != default.price {
            url.query_pairs_mut()
                .append_pair("price", self.price.as_str());
        }

        if self.granularity != default.granularity {
            url.query_pairs_mut()
                .append_pair("granularity", self.granularity.to_string().as_str());
        }

        if self.count != default.count {
            url.query_pairs_mut()
                .append_pair("count", self.count.to_string().as_str());
        }

        if self.from.is_some() {
            url.query_pairs_mut()
                .append_pair("from", self.from_unix_string().as_str());
        }

        if self.to.is_some() {
            url.query_pairs_mut()
                .append_pair("to", self.to_unix_string().as_str());
        }

        if self.smooth != default.smooth {
            url.query_pairs_mut()
                .append_pair("smooth", self.smooth.to_string().as_str());
        }

        if self.include_first != default.include_first {
            url.query_pairs_mut()
                .append_pair("include_first", self.include_first.to_string().as_str());
        }

        if self.daily_alignment != default.daily_alignment {
            url.query_pairs_mut()
                .append_pair("daily_alignment", self.daily_alignment.to_string().as_str());
        }

        if self.alignment_timezone != default.alignment_timezone {
            url.query_pairs_mut()
                .append_pair("alignment_timezone", self.alignment_timezone.as_str());
        }

        if self.weekly_alignment != default.weekly_alignment {
            url.query_pairs_mut().append_pair(
                "weekly_alignment",
                self.weekly_alignment.to_string().as_str(),
            );
        }

        Ok(url.to_string())
    }

    fn from_unix_string(&self) -> String {
        let from = self.from.unwrap_or(Utc::now());
        from.timestamp().to_string()
    }

    fn to_unix_string(&self) -> String {
        let to = self.to.unwrap_or(DateTime::from_timestamp(0, 0).unwrap());
        to.timestamp().to_string()
    }
}

impl Default for Candles {
    fn default() -> Self {
        Candles {
            price: "M".to_string(),
            granularity: Granularity::S5,
            count: 500,
            from: None,
            to: None,
            smooth: false,
            include_first: true,
            daily_alignment: 17,
            alignment_timezone: "America/New_York".to_string(),
            weekly_alignment: WeeklyAlignment::Friday,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};

    use crate::oanda::candle::WeeklyAlignment;
    use crate::oanda::PRACTICE_REST_URL;

    use crate::oanda::request::Candles;

    #[test]
    fn test_url() {
        let base_url = PRACTICE_REST_URL;
        let instrument_name = "EUR_USD";

        let req = Candles::default();
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            price: "MBA".to_string(),
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?price=MBA",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            granularity: crate::oanda::candle::Granularity::H1,
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?granularity=H1",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            count: 5000,
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?count=5000",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            from: DateTime::from_timestamp(1104559200, 0),
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?from=1104559200",
            req.url(base_url, instrument_name).unwrap()
        );

        let to = Utc::now();
        let to_unix = to.timestamp().to_string();
        let req = Candles {
            to: Some(to),
            ..Default::default()
        };
        assert_eq!(
            format!(
                "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?to={}",
                to_unix
            ),
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            smooth: true,
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?smooth=true",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            include_first: false,
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?include_first=false",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            daily_alignment: 12,
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?daily_alignment=12",
            req.url(base_url, instrument_name).unwrap()
        );

        let req = Candles {
            alignment_timezone: "America/Chicago".to_string(),
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?alignment_timezone=America%2FChicago",
            req.url(base_url, instrument_name)
                .unwrap()
        );

        let req = Candles {
            weekly_alignment: WeeklyAlignment::Monday,
            ..Default::default()
        };
        assert_eq!(
            "https://api-fxpractice.oanda.com/v3/instruments/EUR_USD/candles?weekly_alignment=Monday",
            req.url(base_url, instrument_name)
                .unwrap()
        );
    }
}
