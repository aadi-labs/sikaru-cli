use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions, SseStream};
use reqwest::Method;
use std::collections::HashMap;

pub struct RunsClient {
    pub http_client: HttpClient,
}

impl RunsClient {
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
    ///         .runs
    ///         .start(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &StartHarnessRunRequest {
    ///                 input: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 policy: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 product_context: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 tenant_id: "tenant_id".to_string(),
    ///                 user_id: "user_id".to_string(),
    ///                 auto_improve: None,
    ///                 capability_grants: None,
    ///                 compute_environment_id: None,
    ///                 compute_workspace_provenance: None,
    ///                 conversation_id: None,
    ///                 correlation_id: None,
    ///                 event_delivery: None,
    ///                 idempotency_key: None,
    ///                 job_id: None,
    ///                 model: None,
    ///                 run_mode: None,
    ///                 tool_provider_refs: None,
    ///                 trace_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn start(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &StartHarnessRunRequest,
        options: Option<RequestOptions>,
    ) -> Result<ManagedRun, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("v1/projects/{}/harnesses/{}/runs", project_id, harness_id),
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
    ///         .runs
    ///         .get(&"project_id".to_string(), &"run_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ManagedRun, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/runs/{}", project_id, run_id),
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
    ///         .runs
    ///         .pending_actions(&"project_id".to_string(), &"run_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn pending_actions(
        &self,
        project_id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/runs/{}/actions", project_id, run_id),
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
    ///         .runs
    ///         .cancel(&"project_id".to_string(), &"run_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        project_id: &str,
        run_id: &str,
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
                &format!("v1/projects/{}/runs/{}/cancel", project_id, run_id),
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
    ///         .runs
    ///         .events(
    ///             &"project_id".to_string(),
    ///             &"run_id".to_string(),
    ///             &EventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn events(
        &self,
        project_id: &str,
        run_id: &str,
        request: &EventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<RunEvents, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/runs/{}/events", project_id, run_id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("limit", request.limit.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Replay and stream durable run events; resume using after or Last-Event-ID.
    ///
    /// A terminal run event ends the stream. Idle streams may also close: callers
    /// can resume from their last delivered event without restarting the run.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// Server-Sent Events stream (use futures::StreamExt to iterate)
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
    ///         .runs
    ///         .stream_events(
    ///             &"project_id".to_string(),
    ///             &"run_id".to_string(),
    ///             &StreamEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_events(
        &self,
        project_id: &str,
        run_id: &str,
        request: &StreamEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<RunEvent>, ApiError> {
        self.http_client
            .execute_sse_request(
                Method::GET,
                &format!("v1/projects/{}/runs/{}/events/stream", project_id, run_id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .build(),
                options,
                None,
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
    ///         .runs
    ///         .recover(
    ///             &"project_id".to_string(),
    ///             &"run_id".to_string(),
    ///             &RecoverRunRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn recover(
        &self,
        project_id: &str,
        run_id: &str,
        request: &RecoverRunRequest,
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
                &format!("v1/projects/{}/runs/{}/recover", project_id, run_id),
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
    ///         .runs
    ///         .decide_approval(
    ///             &"project_id".to_string(),
    ///             &"run_id".to_string(),
    ///             &"tool_call_id".to_string(),
    ///             &ApprovalInput {
    ///                 decision: ApprovalInputDecision::Approved,
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn decide_approval(
        &self,
        project_id: &str,
        run_id: &str,
        tool_call_id: &str,
        request: &ApprovalInput,
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
                    "v1/projects/{}/runs/{}/tool-calls/{}/approval",
                    project_id, run_id, tool_call_id
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
    ///         .runs
    ///         .submit_tool_result(
    ///             &"project_id".to_string(),
    ///             &"run_id".to_string(),
    ///             &SubmitToolResultRequest {
    ///                 capability_name: "capability_name".to_string(),
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 payload: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 status: SubmitToolResultRequestStatus::Completed,
    ///                 tool_call_id: "tool_call_id".to_string(),
    ///                 tool_provider_id: "tool_provider_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn submit_tool_result(
        &self,
        project_id: &str,
        run_id: &str,
        request: &SubmitToolResultRequest,
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
                &format!("v1/projects/{}/runs/{}/tool-results", project_id, run_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Read retained ATIF structure and usage with private content redacted.
    ///
    /// This is a committed snapshot and can be partial while a run is active or
    /// interrupted. Messages, reasoning, tool payloads and provider metadata are
    /// omitted. No trajectory is synthesized when retained evidence is unavailable.
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
    ///         .runs
    ///         .get_trajectory(&"project_id".to_string(), &"run_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_trajectory(
        &self,
        project_id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/runs/{}/trajectory", project_id, run_id),
                None,
                None,
                options,
            )
            .await
    }
}
