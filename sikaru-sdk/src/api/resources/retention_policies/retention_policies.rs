use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct RetentionPoliciesClient {
    pub http_client: HttpClient,
}

impl RetentionPoliciesClient {
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
    ///         .retention_policies
    ///         .create_retention_policy_update(
    ///             &"project_id".to_string(),
    ///             &CreateRetentionPolicyUpdateRequest {
    ///                 trace_ids: vec!["traceIds".to_string()],
    ///                 agent_id: None,
    ///                 backfill_window: None,
    ///                 filter_expression: None,
    ///                 issue_id: None,
    ///                 issue_title: None,
    ///                 retention_days: None,
    ///                 rule_id: None,
    ///                 rule_name: None,
    ///                 sample_rate: None,
    ///                 signal_tags: None,
    ///                 target: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_retention_policy_update(
        &self,
        project_id: &str,
        request: &CreateRetentionPolicyUpdateRequest,
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
                &format!("v1/projects/{}/retention-policies", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
