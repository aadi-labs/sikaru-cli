use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct WorkflowIntentsClient {
    pub http_client: HttpClient,
}

impl WorkflowIntentsClient {
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
    ///         .workflow_intents
    ///         .create_project_workflow_intent(
    ///             &"project_id".to_string(),
    ///             &CreateProductWorkflowIntentRequest {
    ///                 description: "description".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_project_workflow_intent(
        &self,
        project_id: &str,
        request: &CreateProductWorkflowIntentRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, String>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/workflow-intents", project_id),
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
    ///         .workflow_intents
    ///         .compile_project_workflow_intent(&"project_id".to_string(), &"intent_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn compile_project_workflow_intent(
        &self,
        project_id: &str,
        intent_id: &str,
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
                &format!(
                    "v1/projects/{}/workflow-intents/{}/compile",
                    project_id, intent_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
