use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ChecksClient {
    pub http_client: HttpClient,
}

impl ChecksClient {
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
    ///         .checks
    ///         .list(&"project_id".to_string(), &"agent_slug".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list(
        &self,
        project_id: &str,
        agent_slug: &str,
        options: Option<RequestOptions>,
    ) -> Result<CheckList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/checks",
                    project_id, agent_slug
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Same key and definition return the same check; a changed definition conflicts.
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
    ///         .checks
    ///         .create(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &CreateCheck {
    ///                 environment: CheckEnvironment {
    ///                     compute_environment_id: None,
    ///                     kind: CheckEnvironmentKind::Managed,
    ///                     workspace_provenance: None,
    ///                 },
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 name: "name".to_string(),
    ///                 task: HarborTaskFiles {
    ///                     files: HashMap::from([("key".to_string(), "value".to_string())]),
    ///                     ..Default::default()
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        agent_slug: &str,
        request: &CreateCheck,
        options: Option<RequestOptions>,
    ) -> Result<CheckResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/managed-agents/{}/checks",
                    project_id, agent_slug
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
    ///         .checks
    ///         .list_results(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"check_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_results(
        &self,
        project_id: &str,
        agent_slug: &str,
        check_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<CheckResultList, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/managed-agents/{}/checks/{}/results",
                    project_id, agent_slug, check_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Start the check's task as a real run of the agent's active release; the result settles later.
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
    ///         .checks
    ///         .run(
    ///             &"project_id".to_string(),
    ///             &"agent_slug".to_string(),
    ///             &"check_id".to_string(),
    ///             &RunCheck {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn run(
        &self,
        project_id: &str,
        agent_slug: &str,
        check_id: &str,
        request: &RunCheck,
        options: Option<RequestOptions>,
    ) -> Result<CheckResultResponse, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/managed-agents/{}/checks/{}/runs",
                    project_id, agent_slug, check_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
