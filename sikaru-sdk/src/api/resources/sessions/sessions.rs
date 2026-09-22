use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct SessionsClient {
    pub http_client: HttpClient,
}

impl SessionsClient {
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
    ///         .sessions
    ///         .get_managed_session(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_managed_session(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/sessions/{}", project_id, session_id),
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
    ///         .sessions
    ///         .list_managed_session_events(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &ListManagedSessionEventsQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_managed_session_events(
        &self,
        project_id: &str,
        session_id: &str,
        request: &ListManagedSessionEventsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/sessions/{}/events", project_id, session_id),
                None,
                QueryBuilder::new()
                    .string("after", request.after.clone())
                    .string("limit", request.limit.clone())
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
    ///         .sessions
    ///         .list_managed_session_files(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_managed_session_files(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/sessions/{}/files", project_id, session_id),
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
    ///         .sessions
    ///         .create_managed_interpreter(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &CreateManagedInterpreterRequest {
    ///                 language: "language".to_string(),
    ///                 resource_limits: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_managed_interpreter(
        &self,
        project_id: &str,
        session_id: &str,
        request: &CreateManagedInterpreterRequest,
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
                    "v1/projects/{}/sessions/{}/interpreters",
                    project_id, session_id
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
    ///         .sessions
    ///         .execute_managed_interpreter(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"interpreter_id".to_string(),
    ///             &ExecuteManagedInterpreterRequest {
    ///                 code: "code".to_string(),
    ///                 timeout_seconds: 1,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn execute_managed_interpreter(
        &self,
        project_id: &str,
        session_id: &str,
        interpreter_id: &str,
        request: &ExecuteManagedInterpreterRequest,
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
                    "v1/projects/{}/sessions/{}/interpreters/{}/execute",
                    project_id, session_id, interpreter_id
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
    ///         .sessions
    ///         .list_managed_session_plan(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_managed_session_plan(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/sessions/{}/plan", project_id, session_id),
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
    ///         .sessions
    ///         .start_managed_sandbox_execution(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &CreateSandboxExecutionRequest {
    ///                 context_package_ref: "contextPackageRef".to_string(),
    ///                 idempotency_key: "idempotencyKey".to_string(),
    ///                 timeout_seconds: 1,
    ///                 workflow_ref: "workflowRef".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn start_managed_sandbox_execution(
        &self,
        project_id: &str,
        session_id: &str,
        request: &CreateSandboxExecutionRequest,
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
                    "v1/projects/{}/sessions/{}/sandbox-executions",
                    project_id, session_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
