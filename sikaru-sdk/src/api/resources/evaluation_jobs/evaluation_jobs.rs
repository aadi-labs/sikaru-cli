use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct EvaluationJobsClient {
    pub http_client: HttpClient,
}

impl EvaluationJobsClient {
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
    ///         .evaluation_jobs
    ///         .list_jobs(
    ///             &"project_id".to_string(),
    ///             &ListJobsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_jobs(
        &self,
        project_id: &str,
        request: &ListJobsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/evaluation-jobs", project_id),
                None,
                QueryBuilder::new()
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
    ///         .evaluation_jobs
    ///         .create_job(
    ///             &"project_id".to_string(),
    ///             &JobInput {
    ///                 evaluator: "evaluator".to_string(),
    ///                 request_id: "requestId".to_string(),
    ///                 revision: "revision".to_string(),
    ///                 rubric: "rubric".to_string(),
    ///                 targets: vec![TargetInput {
    ///                     target: JudgmentTarget {
    ///                         account_id: "accountId".to_string(),
    ///                         conversation_id: None,
    ///                         kind: JudgmentTargetKind::Message,
    ///                         message_id: None,
    ///                         span_id: None,
    ///                     },
    ///                     trace_id: "traceId".to_string(),
    ///                 }],
    ///                 environment: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_job(
        &self,
        project_id: &str,
        request: &JobInput,
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
                &format!("v1/projects/{}/evaluation-jobs", project_id),
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
    ///         .evaluation_jobs
    ///         .get_job(&"project_id".to_string(), &"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_job(
        &self,
        project_id: &str,
        job_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/evaluation-jobs/{}", project_id, job_id),
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
    ///         .evaluation_jobs
    ///         .cancel_job(&"project_id".to_string(), &"job_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_job(
        &self,
        project_id: &str,
        job_id: &str,
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
                    "v1/projects/{}/evaluation-jobs/{}/cancel",
                    project_id, job_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
