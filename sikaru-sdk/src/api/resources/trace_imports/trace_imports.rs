use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TraceImportsClient {
    pub http_client: HttpClient,
}

impl TraceImportsClient {
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
    ///         .trace_imports
    ///         .list_trace_imports(&"project_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn list_trace_imports(
        &self,
        project_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/trace-imports", project_id),
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
    ///         .trace_imports
    ///         .create_trace_import(
    ///             &"project_id".to_string(),
    ///             &CreateTraceImportRequest {
    ///                 connection_id: "connectionId".to_string(),
    ///                 converter_version: "converterVersion".to_string(),
    ///                 dataset: "dataset".to_string(),
    ///                 external_project_id: "externalProjectId".to_string(),
    ///                 mode: CreateTraceImportRequestMode::Historical,
    ///                 provider: CreateTraceImportRequestProvider::Langsmith,
    ///                 scope: TraceImportScopeRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 source_instance: "sourceInstance".to_string(),
    ///                 grouping: None,
    ///                 options: None,
    ///                 tags: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_trace_import(
        &self,
        project_id: &str,
        request: &CreateTraceImportRequest,
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
                &format!("v1/projects/{}/trace-imports", project_id),
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
    ///         .trace_imports
    ///         .plan_trace_import(
    ///             &"project_id".to_string(),
    ///             &PlanTraceImportRequest {
    ///                 connection_id: "connectionId".to_string(),
    ///                 converter_version: "converterVersion".to_string(),
    ///                 dataset: "dataset".to_string(),
    ///                 external_project_id: "externalProjectId".to_string(),
    ///                 mode: PlanTraceImportRequestMode::Historical,
    ///                 provider: PlanTraceImportRequestProvider::Langsmith,
    ///                 scope: TraceImportScopeRequest {
    ///                     ..Default::default()
    ///                 },
    ///                 source_instance: "sourceInstance".to_string(),
    ///                 grouping: None,
    ///                 options: None,
    ///                 tags: None,
    ///                 window_duration_hours: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn plan_trace_import(
        &self,
        project_id: &str,
        request: &PlanTraceImportRequest,
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
                &format!("v1/projects/{}/trace-imports/plan", project_id),
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
    ///         .trace_imports
    ///         .get_trace_import(
    ///             &"project_id".to_string(),
    ///             &"trace_import_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_trace_import(
        &self,
        project_id: &str,
        trace_import_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/trace-imports/{}",
                    project_id, trace_import_id
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
    ///         .trace_imports
    ///         .cancel_trace_import(
    ///             &"project_id".to_string(),
    ///             &"trace_import_id".to_string(),
    ///             &VersionedTraceImportAction {
    ///                 expected_version: 1,
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn cancel_trace_import(
        &self,
        project_id: &str,
        trace_import_id: &str,
        request: &VersionedTraceImportAction,
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
                    "v1/projects/{}/trace-imports/{}/cancel",
                    project_id, trace_import_id
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
    ///         .trace_imports
    ///         .get_trace_import_receipt(
    ///             &"project_id".to_string(),
    ///             &"trace_import_id".to_string(),
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_trace_import_receipt(
        &self,
        project_id: &str,
        trace_import_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/trace-imports/{}/receipt",
                    project_id, trace_import_id
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
    ///         .trace_imports
    ///         .retry_trace_import(
    ///             &"project_id".to_string(),
    ///             &"trace_import_id".to_string(),
    ///             &VersionedTraceImportAction {
    ///                 expected_version: 1,
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn retry_trace_import(
        &self,
        project_id: &str,
        trace_import_id: &str,
        request: &VersionedTraceImportAction,
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
                    "v1/projects/{}/trace-imports/{}/retry",
                    project_id, trace_import_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
