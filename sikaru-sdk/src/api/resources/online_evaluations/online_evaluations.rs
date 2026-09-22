use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct OnlineEvaluationsClient {
    pub http_client: HttpClient,
}

impl OnlineEvaluationsClient {
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
    ///         .online_evaluations
    ///         .list_policies(
    ///             &"project_id".to_string(),
    ///             &ListPoliciesQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_policies(
        &self,
        project_id: &str,
        request: &ListPoliciesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/online-evaluations", project_id),
                None,
                QueryBuilder::new()
                    .serialize("after", request.after.clone())
                    .build(),
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
    ///         .online_evaluations
    ///         .create_policy(
    ///             &"project_id".to_string(),
    ///             &PolicyInput {
    ///                 evaluator: "evaluator".to_string(),
    ///                 id: "id".to_string(),
    ///                 revision: "revision".to_string(),
    ///                 rubric: "rubric".to_string(),
    ///                 sample_percent: 1,
    ///                 environment: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_policy(
        &self,
        project_id: &str,
        request: &PolicyInput,
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
                &format!("v1/projects/{}/online-evaluations", project_id),
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
    ///         .online_evaluations
    ///         .preview_policy_eligibility(
    ///             &"project_id".to_string(),
    ///             &PreviewPolicyEligibilityQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn preview_policy_eligibility(
        &self,
        project_id: &str,
        request: &PreviewPolicyEligibilityQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/online-evaluations/preview", project_id),
                None,
                QueryBuilder::new()
                    .serialize("environment", request.environment.clone())
                    .build(),
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
    ///         .online_evaluations
    ///         .update_policy(
    ///             &"project_id".to_string(),
    ///             &"policy_id".to_string(),
    ///             &PolicyState { enabled: true },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_policy(
        &self,
        project_id: &str,
        policy_id: &str,
        request: &PolicyState,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!(
                    "v1/projects/{}/online-evaluations/{}",
                    project_id, policy_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
