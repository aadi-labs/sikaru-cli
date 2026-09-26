use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct EvalSeedsClient {
    pub http_client: HttpClient,
}

impl EvalSeedsClient {
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
    ///         .eval_seeds
    ///         .create_eval_seed(
    ///             &"project_id".to_string(),
    ///             &CreateEvalSeedRequest {
    ///                 issue_id: "issueId".to_string(),
    ///                 issue_title: "issueTitle".to_string(),
    ///                 trace_ids: vec!["traceIds".to_string()],
    ///                 agent_id: None,
    ///                 dataset_name: None,
    ///                 evaluator_name: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_eval_seed(
        &self,
        project_id: &str,
        request: &CreateEvalSeedRequest,
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
                &format!("v1/projects/{}/eval-seeds", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
