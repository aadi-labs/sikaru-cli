use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ComputeOperationsClient {
    pub http_client: HttpClient,
}

impl ComputeOperationsClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Only exact immutable receipt retries are idempotent; changed content conflicts.
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
    ///         .compute_operations
    ///         .submit_receipt(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ReceiptInput {
    ///                 capability_name: None,
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 payload: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 request_digest: "request_digest".to_string(),
    ///                 run_id: "run_id".to_string(),
    ///                 status: ReceiptInputStatus::Completed,
    ///                 tool_call_id: "tool_call_id".to_string(),
    ///                 tool_provider_id: "tool_provider_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn submit_receipt(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &ReceiptInput,
        options: Option<RequestOptions>,
    ) -> Result<ReceiptView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/receipts",
                    project_id, attachment_id
                ),
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
    ///         .compute_operations
    ///         .poll(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ComputeOperationsPollQueryRequest {
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
        attachment_id: &str,
        request: &ComputeOperationsPollQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkPage, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/work",
                    project_id, attachment_id
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
