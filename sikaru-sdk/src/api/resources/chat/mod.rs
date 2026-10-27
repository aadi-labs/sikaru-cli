use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient};

pub mod completions;
pub use completions::CompletionsClient2;
pub struct ChatClient {
    pub http_client: HttpClient,
    pub completions: CompletionsClient2,
}

impl ChatClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
            completions: CompletionsClient2::new(config.clone())?,
        })
    }
}
