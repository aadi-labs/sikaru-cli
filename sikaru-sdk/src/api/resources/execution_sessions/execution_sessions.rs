use crate::api::*;
use crate::{ApiError, ByteStream, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ExecutionSessionsClient {
    pub http_client: HttpClient,
}

impl ExecutionSessionsClient {
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
    ///         .execution_sessions
    ///         .list(
    ///             &"project_id".to_string(),
    ///             &ListQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        request: &ListQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/execution-sessions", project_id),
                None,
                QueryBuilder::new()
                    .serialize("harness_id", request.harness_id.clone())
                    .serialize("after", request.after.clone())
                    .int("limit", request.limit.clone())
                    .serialize("agent_slug", request.agent_slug.clone())
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
    ///         .execution_sessions
    ///         .get(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}",
                    project_id, session_id
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
    ///         .execution_sessions
    ///         .branch(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &BranchInput {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 source_run_id: "source_run_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn branch(
        &self,
        project_id: &str,
        session_id: &str,
        request: &BranchInput,
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
                    "v1/projects/{}/execution-sessions/{}/branches",
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
    ///         .execution_sessions
    ///         .list_files(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_files(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/files",
                    project_id, session_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn upload_file(
        &self,
        project_id: &str,
        session_id: &str,
        request: &UploadFileRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_bytes_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/files",
                    project_id, session_id
                ),
                Some(request.body.to_vec()),
                QueryBuilder::new()
                    .string("filename", request.filename.clone())
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
    ///         .execution_sessions
    ///         .delete_file(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"file_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_file(
        &self,
        project_id: &str,
        session_id: &str,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/files/{}",
                    project_id, session_id, file_id
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
    ///         .execution_sessions
    ///         .download_file(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"file_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn download_file(
        &self,
        project_id: &str,
        session_id: &str,
        file_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<ByteStream, ApiError> {
        self.http_client
            .execute_stream_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/files/{}/content",
                    project_id, session_id, file_id
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
    ///         .execution_sessions
    ///         .list_session_inputs(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_session_inputs(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/inputs",
                    project_id, session_id
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
    ///         .execution_sessions
    ///         .spend(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn spend(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SessionSpend, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/spend",
                    project_id, session_id
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
    ///         .execution_sessions
    ///         .append_turn(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &TurnInput {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 input: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 capability_grants: None,
    ///                 compute_attachment_id: None,
    ///                 delivery_mode: None,
    ///                 file_ids: None,
    ///                 product_context: None,
    ///                 run_mode: None,
    ///                 tool_provider_refs: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn append_turn(
        &self,
        project_id: &str,
        session_id: &str,
        request: &TurnInput,
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
                    "v1/projects/{}/execution-sessions/{}/turns",
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
    ///         .execution_sessions
    ///         .create(
    ///             &"project_id".to_string(),
    ///             &"harness_id".to_string(),
    ///             &SessionInput {
    ///                 tenant_id: "tenant_id".to_string(),
    ///                 user_id: "user_id".to_string(),
    ///                 auto_improve: None,
    ///                 conversation_id: None,
    ///                 environment: None,
    ///                 final_output_schema: None,
    ///                 idempotency_key: None,
    ///                 model: None,
    ///                 reasoning_effort: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        harness_id: &str,
        request: &SessionInput,
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
                    "v1/projects/{}/harnesses/{}/execution-sessions",
                    project_id, harness_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
