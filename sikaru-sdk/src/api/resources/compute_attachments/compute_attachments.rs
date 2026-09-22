use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;

pub struct ComputeAttachmentsClient {
    pub http_client: HttpClient,
}

impl ComputeAttachmentsClient {
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
    ///         .compute_attachments
    ///         .get(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get(
        &self,
        project_id: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-attachments/{}",
                    project_id, attachment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Irreversibly abandon execution authority, preserving unknown effects. Cleanup is still required.
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
    ///         .compute_attachments
    ///         .abandon(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &AbandonInput {
    ///                 evidence: "evidence".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn abandon(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &AbandonInput,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/abandon",
                    project_id, attachment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Idempotently request stopping. This never asserts child cleanup.
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
    ///         .compute_attachments
    ///         .cancel(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel(
        &self,
        project_id: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/cancel",
                    project_id, attachment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Same attachment/key/worker returns the same launch identity, never another launch.
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
    ///         .compute_attachments
    ///         .claim(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ClaimInput {
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn claim(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &ClaimInput,
        options: Option<RequestOptions>,
    ) -> Result<ClaimView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/claim",
                    project_id, attachment_id
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
    ///         .compute_attachments
    ///         .cleanup(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &CleanupInput {
    ///                 children_terminated: true,
    ///                 evidence: "evidence".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cleanup(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &CleanupInput,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/cleanup",
                    project_id, attachment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Attest original workspace/journal. Reconnection never replays uncertain effects.
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
    ///         .compute_attachments
    ///         .connect(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ReadyInput {
    ///                 capabilities: vec![ReadyInputCapabilitiesItem::ComputeExecute],
    ///                 executor_instance_id: "executor_instance_id".to_string(),
    ///                 journal_id: "journal_id".to_string(),
    ///                 protocol_version: ReadyInputProtocolVersion::SikaruComputeV1,
    ///                 workspace_provenance: WorkspaceProvenance {
    ///                     identity: "identity".to_string(),
    ///                     kind: WorkspaceProvenanceKind::ExistingDirectory,
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn connect(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &ReadyInput,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/connect",
                    project_id, attachment_id
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
    ///         .compute_attachments
    ///         .issue_credential(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ExecutorCredentialInput {
    ///                 owner_epoch: 1,
    ///                 owner_id: "owner_id".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn issue_credential(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &ExecutorCredentialInput,
        options: Option<RequestOptions>,
    ) -> Result<CredentialIssued, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/credentials",
                    project_id, attachment_id
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
    ///         .compute_attachments
    ///         .heartbeat(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn heartbeat(
        &self,
        project_id: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/heartbeat",
                    project_id, attachment_id
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
    ///         .compute_attachments
    ///         .ready(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ReadyInput {
    ///                 capabilities: vec![ReadyInputCapabilitiesItem::ComputeExecute],
    ///                 executor_instance_id: "executor_instance_id".to_string(),
    ///                 journal_id: "journal_id".to_string(),
    ///                 protocol_version: ReadyInputProtocolVersion::SikaruComputeV1,
    ///                 workspace_provenance: WorkspaceProvenance {
    ///                     identity: "identity".to_string(),
    ///                     kind: WorkspaceProvenanceKind::ExistingDirectory,
    ///                 },
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn ready(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &ReadyInput,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/ready",
                    project_id, attachment_id
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
    ///         .compute_attachments
    ///         .reconcile(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &ReconcileInput {
    ///                 executor_instance_id: "executor_instance_id".to_string(),
    ///                 journal_id: "journal_id".to_string(),
    ///                 workspace_provenance: WorkspaceProvenance {
    ///                     identity: "identity".to_string(),
    ///                     kind: WorkspaceProvenanceKind::ExistingDirectory,
    ///                 },
    ///                 processes: None,
    ///                 receipts: None,
    ///                 uncertain_operation_ids: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn reconcile(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &ReconcileInput,
        options: Option<RequestOptions>,
    ) -> Result<ReconcileView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/reconcile",
                    project_id, attachment_id
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
    ///         .compute_attachments
    ///         .status(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn status(
        &self,
        project_id: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/status",
                    project_id, attachment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Executor can stop its own attachment and cancel its runs; cleanup is separately reported.
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
    ///         .compute_attachments
    ///         .stop(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stop(
        &self,
        project_id: &str,
        attachment_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/stop",
                    project_id, attachment_id
                ),
                None,
                None,
                options,
            )
            .await
    }

    /// Controller or owning worker attests sandbox teardown; clean parked turns stay resumable.
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
    ///         .compute_attachments
    ///         .teardown(
    ///             &"project_id".to_string(),
    ///             &"attachment_id".to_string(),
    ///             &TeardownInput {
    ///                 children_terminated: true,
    ///                 evidence: "evidence".to_string(),
    ///                 owner_epoch: 1,
    ///                 owner_id: "owner_id".to_string(),
    ///                 workspace_generation: "workspace_generation".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn teardown(
        &self,
        project_id: &str,
        attachment_id: &str,
        request: &TeardownInput,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/compute-attachments/{}/teardown",
                    project_id, attachment_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Same session/key/workspace returns the original binding; changed workspace conflicts.
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
    ///         .compute_attachments
    ///         .create(
    ///             &"project_id".to_string(),
    ///             &"session_id".to_string(),
    ///             &AttachmentInput {
    ///                 environment_id: "environment_id".to_string(),
    ///                 idempotency_key: "idempotency_key".to_string(),
    ///                 workspace_provenance: WorkspaceProvenance {
    ///                     identity: "identity".to_string(),
    ///                     kind: WorkspaceProvenanceKind::ExistingDirectory,
    ///                 },
    ///                 replace_existing: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        project_id: &str,
        session_id: &str,
        request: &AttachmentInput,
        options: Option<RequestOptions>,
    ) -> Result<AttachmentView, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!(
                    "v1/projects/{}/execution-sessions/{}/compute-attachments",
                    project_id, session_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
