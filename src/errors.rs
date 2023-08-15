use std::string::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("HTTP error: {0}")]
    Http(reqwest::Error),
    #[error("JSON error: {0}")]
    Json(serde_json::Error),
    #[error("URL error: {0}")]
    Url(ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientCreationError {
    #[error("Reqwest error: {0}")]
    Http(reqwest::Error),
    #[error("URL error: {0}")]
    Url(ParseError),
}