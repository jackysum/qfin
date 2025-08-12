use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("error making request: {}", 0.to_string())]
    Request(reqwest::Error),

    #[error("request failed with status code: {}", 0)]
    StatusNotOK(reqwest::StatusCode),

    #[error("error deserializing request json: {}", 0.to_string())]
    Deserialize(reqwest::Error),
}
