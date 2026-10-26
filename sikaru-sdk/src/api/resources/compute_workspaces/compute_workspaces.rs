use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ComputeWorkspacesClient {
    pub http_client: HttpClient,
}

impl ComputeWorkspacesClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Record a verified uploaded manifest as this session's workspace checkpoint.
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
    ///         .compute_workspaces
    ///         .record(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &WorkspaceCheckpointInput {
    ///                 manifest_digest: "manifest_digest".to_string(),
    ///                 trigger: WorkspaceCheckpointInputTrigger::Turn,
    ///                 run_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn record(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &WorkspaceCheckpointInput,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceCheckpointView, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/workspace-checkpoints",
                    project_id, attachment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Upload file or manifest bytes before acknowledging a workspace checkpoint.
    ///
    /// # Arguments
    ///
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn upload(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &Vec<u8>,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceContentView, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_bytes_request(
                Method::PUT,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/workspace-content",
                    project_id, attachment_id
                ),
                Some(request.to_vec()),
                None,
                options,
            )
            .await
    }

    /// Read the pinned publication rules before capturing workspace files.
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
    ///         .compute_workspaces
    ///         .policy(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &PolicyQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn policy(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &PolicyQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<WorkspacePolicyView, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(&options, &[&["BearerAuth"] as &[&str]])
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/workspace-policy",
                    project_id, attachment_id
                ),
                None,
                QueryBuilder::new()
                    .serialize("run_id", request.run_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
