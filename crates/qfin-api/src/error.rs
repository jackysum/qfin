#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("error deserializing data from response: {0}")]
    DeserializeResponse(String),

    #[error("error deserializing data from response: {0}")]
    HttpStatus(reqwest::StatusCode),

    #[error("error making request: {0}")]
    Request(String),
}
