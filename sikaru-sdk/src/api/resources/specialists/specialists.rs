use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct SpecialistsClient {
    pub http_client: HttpClient,
}

impl SpecialistsClient {
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
    ///         .specialists
    ///         .list(&"project_id".to_string(), &"session_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        session_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SpecialistThreads, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/specialists",
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
    ///         .specialists
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"thread_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        session_id: &str,
        thread_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<SpecialistThread, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/specialists/{}",
                    project_id, session_id, thread_id
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
    ///         .specialists
    ///         .cancel(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"thread_id".to_string(),
    ///             &SpecialistCancel {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        project_id: &str,
        session_id: &str,
        thread_id: &str,
        request: &SpecialistCancel,
        options: Option<RequestOptions>,
    ) -> Result<SpecialistReceipt, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/specialists/{}/cancel",
                    project_id, session_id, thread_id
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
    ///         .specialists
    ///         .message(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &"thread_id".to_string(),
    ///             &SpecialistMessage {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 input: HashMap::from([("key".to_string(), serde_json::json!("value"))]),
    ///                 delivery_mode: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn message(
        &self,
        project_id: &str,
        session_id: &str,
        thread_id: &str,
        request: &SpecialistMessage,
        options: Option<RequestOptions>,
    ) -> Result<SpecialistReceipt, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/specialists/{}/messages",
                    project_id, session_id, thread_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
