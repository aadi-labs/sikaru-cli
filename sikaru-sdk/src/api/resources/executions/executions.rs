use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ExecutionsClient {
    pub http_client: HttpClient,
}

impl ExecutionsClient {
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
    ///     client
    ///         .executions
    ///         .execution_runtime_lineage(
    ///             &"project_id".to_string(),
    ///             &"trace_id".to_string(),
    ///             &ExecutionRuntimeLineageQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 inference_after: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn execution_runtime_lineage(
        &self,
        project_id: &str,
        trace_id: &str,
        request: &ExecutionRuntimeLineageQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/executions/{}/runtime", project_id, trace_id),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .serialize("inference_after", request.inference_after.clone())
                    .build(),
                options,
            )
            .await
    }
}
