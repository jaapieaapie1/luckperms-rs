use std::string::ParseError;
use reqwest::header::HeaderMap;
use reqwest::{Client, Url};
use crate::errors::ClientCreationError;

pub mod users;
pub mod models;
pub mod errors;
pub mod requests;
pub mod groups;
pub mod actions;

/// A client for interacting with a LuckPerms instance.
pub struct LuckClient {
    base_url: Url,
    client: Client,
}

impl LuckClient {
    /// Create a new LuckClient.
    /// ```rust
    /// use luckperms_rs::LuckClient;
    ///
    /// fn main() {
    ///     let client = LuckClient::try_new("http://localhost:8080".to_string(), "YOUR API KEY".to_string()).unwrap();
    /// }
    pub fn try_new(base_url: String, api_key: String) -> Result<Self, ClientCreationError> {
        let url = Url::parse(&base_url)?;

        let mut headers = HeaderMap::new();
        headers.insert("X-API-KEY", api_key.parse().unwrap());

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers).build()?;

        Ok(LuckClient {
            base_url: url,
            client
        })
    }
}