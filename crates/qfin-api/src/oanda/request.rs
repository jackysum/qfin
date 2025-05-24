use crate::Error;

pub struct CandlesRequest {}

impl CandlesRequest {
    pub fn url(&self, base_url: &str, instrument_name: &str) -> Result<reqwest::Url, Error> {
        let url = format!("{}/v3/instruments/{}/candles", base_url, instrument_name);
        let params = [("price", "MBA")];

        let url = reqwest::Url::parse_with_params(&url, &params)
            .map_err(|e| Error::UrlParse(e.to_string()))?;

        Ok(url)
    }
}
