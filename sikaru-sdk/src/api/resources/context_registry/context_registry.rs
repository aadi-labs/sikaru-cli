use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ContextRegistryClient {
    pub http_client: HttpClient,
}

impl ContextRegistryClient {
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
    ///         .context_registry
    ///         .create_context_registry_change(
    ///             &"project_id".to_string(),
    ///             &CreateContextRegistryChangeRequest {
    ///                 name: "name".to_string(),
    ///                 repo_id: "repoId".to_string(),
    ///                 action: None,
    ///                 files: None,
    ///                 kind: None,
    ///                 linked_signal: None,
    ///                 owner: None,
    ///                 production_commit: None,
    ///                 production_tag: None,
    ///                 repo_type: None,
    ///                 source_trace_ids: None,
    ///                 staging_commit: None,
    ///                 staging_tag: None,
    ///                 tools: None,
    ///                 variables: None,
    ///                 visibility: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_context_registry_change(
        &self,
        project_id: &str,
        request: &CreateContextRegistryChangeRequest,
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
                &format!("v1/projects/{}/context-registry", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
