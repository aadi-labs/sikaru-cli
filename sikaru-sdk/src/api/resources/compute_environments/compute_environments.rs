use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ComputeEnvironmentsClient {
    pub http_client: HttpClient,
}

impl ComputeEnvironmentsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Same key + same configuration returns the same environment; changed input conflicts.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
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
    ///     client
    ///         .compute_environments
    ///         .create(
    ///             &"project_id".to_string(),
    ///             &EnvironmentInput {
    ///                 environment_slug: "environment_slug".to_string(),
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 product_id: None,
    ///                 session_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        request: &EnvironmentInput,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/compute-environments", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
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
    ///     client
    ///         .compute_environments
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"environment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        environment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentView, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-environments/{}",
                    project_id, environment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Idempotently disable future claims and credentials; existing cleanup remains required.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
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
    ///     client
    ///         .compute_environments
    ///         .disable(
    ///             &"project_id".to_string(),
    ///             &"environment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn disable(
        &self,
        project_id: &str,
        environment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<EnvironmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-environments/{}/disable",
                    project_id, environment_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
