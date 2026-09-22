use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct JudgeAlignmentClient {
    pub http_client: HttpClient,
}

impl JudgeAlignmentClient {
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
    ///         .judge_alignment
    ///         .get_judge_alignment(
    ///             &"project_id".to_string(),
    ///             &GetJudgeAlignmentQueryRequest {
    ///                 evaluator: "evaluator".to_string(),
    ///                 revision: "revision".to_string(),
    ///                 environment: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_judge_alignment(
        &self,
        project_id: &str,
        request: &GetJudgeAlignmentQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/judge-alignment", project_id),
                None,
                QueryBuilder::new()
                    .string("evaluator", request.evaluator.clone())
                    .string("revision", request.revision.clone())
                    .serialize("environment", request.environment.clone())
                    .build(),
                options,
            )
            .await
    }
}
