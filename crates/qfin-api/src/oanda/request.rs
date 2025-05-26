use chrono::{DateTime, Utc};

use crate::{
    oanda::entity::candle::{Granularity, WeeklyAlignment},
    Error,
};

#[derive(Debug, PartialEq)]
pub struct CandlesRequest {
    price: String,
    granularity: Granularity,
    count: i64,
    smooth: bool,
    include_first: bool,
    daily_alignment: i8,
    alignment_timezone: String,
    weekly_alignment: WeeklyAlignment,
    from: Option<DateTime<Utc>>,
    to: Option<DateTime<Utc>>,
}

impl CandlesRequest {
    pub fn url(&self, base_url: &str, instrument_name: &str) -> Result<reqwest::Url, Error> {
        let url = format!("{}/v3/instruments/{}/candles", base_url, instrument_name);
        let mut params = vec![
            ("price", self.price.clone()),
            ("granularity", self.granularity.to_string()),
            ("count", self.count.clone().to_string()),
            ("smooth", self.smooth.to_string()),
            ("includeFirst", self.include_first.to_string()),
            ("dailyAlignment", self.daily_alignment.to_string()),
            ("alignmentTimezone", self.alignment_timezone.clone()),
            ("weeklyAlignment", self.weekly_alignment.to_string()),
        ];

        match self.from {
            Some(from) => params.push(("from", from.timestamp().to_string())),
            None => (),
        }

        match self.to {
            Some(to) => params.push(("to", to.timestamp().to_string())),
            None => (),
        }

        let url = reqwest::Url::parse_with_params(&url, &params)
            .map_err(|e| Error::UrlParse(e.to_string()))?;

        Ok(url)
    }
}

impl Default for CandlesRequest {
    fn default() -> Self {
        CandlesRequest {
            price: "M".into(),
            granularity: Granularity::S5,
            count: 500,
            smooth: false,
            include_first: true,
            daily_alignment: 17,
            alignment_timezone: "America/New_York".into(),
            weekly_alignment: WeeklyAlignment::Friday,
            from: None,
            to: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use crate::{
        oanda::{
            entity::candle::{Granularity, WeeklyAlignment},
            request::CandlesRequest,
        },
        Error,
    };

    #[test]
    fn test_candles_request_default() {
        let want = CandlesRequest {
            price: "M".into(),
            granularity: Granularity::S5,
            count: 500,
            smooth: false,
            include_first: true,
            daily_alignment: 17,
            alignment_timezone: "America/New_York".into(),
            weekly_alignment: WeeklyAlignment::Friday,
            from: None,
            to: None,
        };
        assert_eq!(want, CandlesRequest::default());
    }

    #[test]
    fn test_candles_request_url_success() {
        let request = CandlesRequest::default();
        let base_url = "https://api-fxtrade.oanda.com";
        let instrument_name = "EUR_USD";

        let want = format!(
            "{}/v3/instruments/{}/candles?price=M&granularity=S5&count=500&smooth=false&includeFirst=true&dailyAlignment=17&alignmentTimezone=America%2FNew_York&weeklyAlignment=Friday",
            base_url, instrument_name
        );
        assert_eq!(
            want,
            request.url(base_url, instrument_name).unwrap().to_string(),
        );
    }

    #[test]
    fn test_candles_request_url_with_from_success() {
        let from = Utc::now();
        let request = CandlesRequest {
            from: Some(from),
            ..Default::default()
        };
        assert!(request
            .url("https://api-fxtrade.oanda.com", "EUR_USD")
            .unwrap()
            .to_string()
            .contains(&format!("from={}", from.timestamp())))
    }

    #[test]
    fn test_candles_request_url_with_to_success() {
        let to = Utc::now();
        let request = CandlesRequest {
            to: Some(to),
            ..Default::default()
        };
        assert!(request
            .url("https://api-fxtrade.oanda.com", "EUR_USD")
            .unwrap()
            .to_string()
            .contains(&format!("to={}", to.timestamp())))
    }

    #[test]
    fn test_candles_request_url_error() {
        let request = CandlesRequest::default();
        let got = request.url("not an url", "EUR_USD");

        assert!(got.is_err_and(|e| matches!(e, Error::UrlParse(_))))
    }
}
