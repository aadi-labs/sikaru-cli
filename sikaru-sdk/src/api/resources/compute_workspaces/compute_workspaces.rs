use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
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
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &"run_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        attachment_id: &str,
        run_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceCheckpointView, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/workspace-checkpoints/{}",
                    project_id, attachment_id, run_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    pub async fn put_blob(
        &self,
        project_id: &str,
        attachment_id: &str,
        run_id: &str,
        sha256: &str,
        request: &Vec<u8>,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceBlobView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_bytes_request(
                Method::PUT,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/workspace-checkpoints/{}/blobs/{}",
                    project_id, attachment_id, run_id, sha256
                ),
                Some(request.to_vec()),
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
    ///         .compute_workspaces
    ///         .commit_tree(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &"run_id".to_string(),
    ///             &WorkspaceTreeInput {
    ///                 files: HashMap::from([(
    ///                     "key".to_string(),
    ///                     WorkspaceFile {
    ///                         chunks: vec![WorkspaceChunk {
    ///                             sha256: "sha256".to_string(),
    ///                             size: 1,
    ///                             ..Default::default()
    ///                         }],
    ///                         mode: 1,
    ///                         sha256: "sha256".to_string(),
    ///                         size: 1,
    ///                         ..Default::default()
    ///                     },
    ///                 )]),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn commit_tree(
        &self,
        project_id: &str,
        attachment_id: &str,
        run_id: &str,
        request: &WorkspaceTreeInput,
        options: Option<RequestOptions>,
    ) -> Result<WorkspaceCheckpointView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/workspace-checkpoints/{}/tree",
                    project_id, attachment_id, run_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
