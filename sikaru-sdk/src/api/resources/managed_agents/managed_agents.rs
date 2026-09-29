use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ManagedAgentsClient {
    pub http_client: HttpClient,
}

impl ManagedAgentsClient {
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
    ///         .managed_agents
    ///         .list_managed_agents(&"project_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_managed_agents(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/managed-agents", project_id),
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
    ///         .managed_agents
    ///         .create_managed_agent(
    ///             &"project_id".to_string(),
    ///             &CreateManagedAgentRequest {
    ///                 agent_slug: "agentSlug".to_string(),
    ///                 active_harness_version_id: None,
    ///                 compatibility_profile_id: None,
    ///                 display_name: None,
    ///                 harness_id: None,
    ///                 source: None,
    ///                 status: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_managed_agent(
        &self,
        project_id: &str,
        request: &CreateManagedAgentRequest,
        options: Option<RequestOptions>,
    ) -> Result<CreatedManagedAgent, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/managed-agents", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Stage a changed definition as a draft revision; the live definition is a no-op.
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
    ///         .managed_agents
    ///         .create_definition_revision(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &DefinitionRevisionRequest {
    ///                 content_digest: "contentDigest".to_string(),
    ///                 definition: AgentDefinition {
    ///                     instructions: None,
    ///                     outcomes: None,
    ///                     schema: AgentDefinitionSchema::SikaruAgentContractV1,
    ///                     setup: None,
    ///                     sources: None,
    ///                     tools: None,
    ///                     web: None,
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_definition_revision(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &DefinitionRevisionRequest,
        options: Option<RequestOptions>,
    ) -> Result<DefinitionRevisionResult, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/managed-agents/{}/definition-revisions",
                    project_id, agent_slug
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
    ///         .managed_agents
    ///         .get_definition_revision(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"changeset_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_definition_revision(
        &self,
        project_id: &str,
        agent_slug: &str,
        changeset_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<DefinitionRevisionView, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/definition-revisions/{}",
                    project_id, agent_slug, changeset_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
