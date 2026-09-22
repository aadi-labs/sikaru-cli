use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct EvaluatorRunsClient {
    pub http_client: HttpClient,
}

impl EvaluatorRunsClient {
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
    ///         .evaluator_runs
    ///         .create_evaluator_run(
    ///             &"project_id".to_string(),
    ///             &CreateEvaluatorRunRequest {
    ///                 trace_ids: vec!["traceIds".to_string()],
    ///                 agent_id: None,
    ///                 backfill_window: None,
    ///                 dataset_name: None,
    ///                 evaluator_name: None,
    ///                 filter_expression: None,
    ///                 issue_id: None,
    ///                 issue_title: None,
    ///                 sample_rate: None,
    ///                 signal_tags: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_evaluator_run(
        &self,
        project_id: &str,
        request: &CreateEvaluatorRunRequest,
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
                &format!("v1/projects/{}/evaluator-runs", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
