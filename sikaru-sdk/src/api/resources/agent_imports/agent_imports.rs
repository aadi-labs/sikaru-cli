use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct AgentImportsClient {
    pub http_client: HttpClient,
}

impl AgentImportsClient {
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
    ///         .agent_imports
    ///         .list_agent_imports(&"project_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_agent_imports(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/agent-imports", project_id),
                None,
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
    ///         .agent_imports
    ///         .create_agent_import(
    ///             &"project_id".to_string(),
    ///             &CreateAgentImportRequest {
    ///                 improve: AgentImportImproveRequest {
    ///                     mode: "mode".to_string(),
    ///                     objective: "objective".to_string(),
    ///                     promotion_gate: "promotionGate".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 name: "name".to_string(),
    ///                 eval_suites: None,
    ///                 model_capture: None,
    ///                 runner: None,
    ///                 source_refs: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_agent_import(
        &self,
        project_id: &str,
        request: &CreateAgentImportRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/agent-imports", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
