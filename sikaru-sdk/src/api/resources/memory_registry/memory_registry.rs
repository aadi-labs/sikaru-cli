use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct MemoryRegistryClient {
    pub http_client: HttpClient,
}

impl MemoryRegistryClient {
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
    ///         .memory_registry
    ///         .create_memory_registry_change(
    ///             &"project_id".to_string(),
    ///             &CreateMemoryRegistryChangeRequest {
    ///                 memory_id: "memoryId".to_string(),
    ///                 memory_type: "memoryType".to_string(),
    ///                 name: "name".to_string(),
    ///                 scope: "scope".to_string(),
    ///                 action: None,
    ///                 candidate_release_id: None,
    ///                 confidence: None,
    ///                 policy: None,
    ///                 provenance: None,
    ///                 source: None,
    ///                 source_trace_ids: None,
    ///                 ttl: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_memory_registry_change(
        &self,
        project_id: &str,
        request: &CreateMemoryRegistryChangeRequest,
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
                &format!("v1/projects/{}/memory-registry", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
