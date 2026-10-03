use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct AuthClient {
    pub http_client: HttpClient,
}

impl AuthClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// # Examples
    ///
    /// ```no_run
    /// use sikaru_sdk::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         token: Some("<token>".to_string()),
    ///         ..Default::default()
    ///     };
    ///     let client = SikaruClient::new(config).expect("Failed to build client");
    ///     client.auth.get_device_configuration(None).await;
    /// }
    /// ```
    pub async fn get_device_configuration(
        &self,
        options: Option<RequestOptions>,
    ) -> Result<DeviceConfiguration, ApiError> {
        self.http_client
            .execute_request(Method::GET, "v1/auth/device/config", None, None, options)
            .await
    }
}
