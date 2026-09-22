use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ToolProvidersClient {
    pub http_client: HttpClient,
}

impl ToolProvidersClient {
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
    ///         .tool_providers
    ///         .register_tool_provider(
    ///             &"project_id".to_string(),
    ///             &RegisterToolProviderRequest {
    ///                 broker_endpoint_ref: "broker_endpoint_ref".to_string(),
    ///                 capability_prefix: "capability_prefix".to_string(),
    ///                 display_name: "display_name".to_string(),
    ///                 provider_type: "provider_type".to_string(),
    ///                 tool_catalog_ref: "tool_catalog_ref".to_string(),
    ///                 approval_policy_ref: None,
    ///                 credential_owner: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn register_tool_provider(
        &self,
        project_id: &str,
        request: &RegisterToolProviderRequest,
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
                &format!("v1/projects/{}/tool-providers", project_id),
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
    ///         .tool_providers
    ///         .attach_source_tool_skill(
    ///             &"project_id".to_string(),
    ///             &"tool_provider_id".to_string(),
    ///             &AttachSourceToolSkillRequest {
    ///                 capability_refs: vec!["capability_refs".to_string()],
    ///                 description: "description".to_string(),
    ///                 source: ToolSkillSourceRequest {
    ///                     kind: "kind".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn attach_source_tool_skill(
        &self,
        project_id: &str,
        tool_provider_id: &str,
        request: &AttachSourceToolSkillRequest,
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
                    "v1/projects/{}/tool-providers/{}/skills",
                    project_id, tool_provider_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
