use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct EvaluationResultsClient {
    pub http_client: HttpClient,
}

impl EvaluationResultsClient {
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
    ///         .evaluation_results
    ///         .list_results(
    ///             &"project_id".to_string(),
    ///             &ListResultsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_results(
        &self,
        project_id: &str,
        request: &ListResultsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/evaluation-results", project_id),
                None,
                QueryBuilder::new()
                    .serialize("environment", request.environment.clone())
                    .serialize("evaluator", request.evaluator.clone())
                    .serialize("verdict", request.verdict.clone())
                    .int("limit", request.limit.clone())
                    .serialize("cursor", request.cursor.clone())
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
    ///         .evaluation_results
    ///         .record_result(
    ///             &"project_id".to_string(),
    ///             &EvaluationResult {
    ///                 environment: EvaluationResultEnvironment::Production,
    ///                 evaluator: "evaluator".to_string(),
    ///                 id: "id".to_string(),
    ///                 public_reason: "publicReason".to_string(),
    ///                 revision: "revision".to_string(),
    ///                 source: EvaluationResultSource::Human,
    ///                 trace_id: "traceId".to_string(),
    ///                 verdict: EvaluationResultVerdict::Pass,
    ///                 adjudicates: None,
    ///                 context: None,
    ///                 target: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn record_result(
        &self,
        project_id: &str,
        request: &EvaluationResult,
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
                &format!("v1/projects/{}/evaluation-results", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
