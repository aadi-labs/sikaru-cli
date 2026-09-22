use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ComputeWorkersClient {
    pub http_client: HttpClient,
}

impl ComputeWorkersClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Issue a new secret once; retries issue independent revocable credentials.
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
    ///         .compute_workers
    ///         .issue_credential(
    ///             &"project_id".to_string(),
    ///             &"environment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn issue_credential(
        &self,
        project_id: &str,
        environment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CredentialIssued, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-environments/{}/credentials",
                    project_id, environment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Bounded queue snapshot. wait_seconds is a maximum; server may return immediately.
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
    ///         .compute_workers
    ///         .poll(
    ///             &"project_id".to_string(),
    ///             &"environment_id".to_string(),
    ///             &ComputeWorkersPollQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn poll(
        &self,
        project_id: &str,
        environment_id: &str,
        request: &ComputeWorkersPollQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueuePage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-environments/{}/queue",
                    project_id, environment_id
                ),
                None,
                QueryBuilder::new()
                    .int("wait_seconds", request.wait_seconds.clone())
                    .int("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }
}
