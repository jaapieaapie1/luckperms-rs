use url::ParseError;

#[derive(Debug, thiserror::Error)]
pub enum RequestError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("URL error: {0}")]
    Url(#[from] ParseError),
}

#[derive(Debug, thiserror::Error)]
pub enum ClientCreationError {
    #[error("Reqwest error: {0}")]
    Http(#[from] reqwest::Error),
    #[error("URL error: {0}")]
    Url(#[from] ParseError),
}