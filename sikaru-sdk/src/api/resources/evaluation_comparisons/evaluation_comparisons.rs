use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct EvaluationComparisonsClient {
    pub http_client: HttpClient,
}

impl EvaluationComparisonsClient {
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
    ///         .evaluation_comparisons
    ///         .list_comparisons(
    ///             &"project_id".to_string(),
    ///             &ListComparisonsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_comparisons(
        &self,
        project_id: &str,
        request: &ListComparisonsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/evaluation-comparisons", project_id),
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
    ///         .evaluation_comparisons
    ///         .create_comparison(
    ///             &"project_id".to_string(),
    ///             &ComparisonInput {
    ///                 cases: vec![CaseInput {
    ///                     baseline: AnswerRef {
    ///                         account_id: "accountId".to_string(),
    ///                         conversation_id: "conversationId".to_string(),
    ///                         message_id: "messageId".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     candidate: AnswerRef {
    ///                         account_id: "accountId".to_string(),
    ///                         conversation_id: "conversationId".to_string(),
    ///                         message_id: "messageId".to_string(),
    ///                         ..Default::default()
    ///                     },
    ///                     case_id: "caseId".to_string(),
    ///                     ..Default::default()
    ///                 }],
    ///                 evaluator: "evaluator".to_string(),
    ///                 id: "id".to_string(),
    ///                 revision: "revision".to_string(),
    ///                 rubric: "rubric".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_comparison(
        &self,
        project_id: &str,
        request: &ComparisonInput,
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
                &format!("v1/projects/{}/evaluation-comparisons", project_id),
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
    ///         .evaluation_comparisons
    ///         .get_comparison(
    ///             &"project_id".to_string(),
    ///             &"comparison_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_comparison(
        &self,
        project_id: &str,
        comparison_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/evaluation-comparisons/{}",
                    project_id, comparison_id
                ),
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
    ///         .evaluation_comparisons
    ///         .cancel_comparison(
    ///             &"project_id".to_string(),
    ///             &"comparison_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_comparison(
        &self,
        project_id: &str,
        comparison_id: &str,
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
                    "v1/projects/{}/evaluation-comparisons/{}/cancel",
                    project_id, comparison_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
