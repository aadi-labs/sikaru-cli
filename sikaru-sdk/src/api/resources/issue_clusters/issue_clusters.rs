use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct IssueClustersClient {
    pub http_client: HttpClient,
}

impl IssueClustersClient {
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
    ///         .issue_clusters
    ///         .list_issue_clusters(
    ///             &"project_id".to_string(),
    ///             &ListIssueClustersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_issue_clusters(
        &self,
        project_id: &str,
        request: &ListIssueClustersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/issue-clusters", project_id),
                None,
                QueryBuilder::new()
                    .serialize("status", request.status.clone())
                    .serialize("severity", request.severity.clone())
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
    ///         .issue_clusters
    ///         .upsert_issue_cluster(
    ///             &"project_id".to_string(),
    ///             &UpsertIssueClusterRequest {
    ///                 label: "label".to_string(),
    ///                 trace_ids: vec!["traceIds".to_string()],
    ///                 agent_id: None,
    ///                 impact: None,
    ///                 issue_id: None,
    ///                 issue_title: None,
    ///                 signal_tags: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn upsert_issue_cluster(
        &self,
        project_id: &str,
        request: &UpsertIssueClusterRequest,
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
                &format!("v1/projects/{}/issue-clusters", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Run one failure-analysis agent pass over the project's recent traces.
    ///
    /// This is a model-latency operation on a request path: the endpoint is sync so
    /// the harness runs in the threadpool, and a project may only have one pass in
    /// flight. A background job queue is the long-term home for this work; the
    /// in-flight guard below is the interim bound.
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
    ///         .issue_clusters
    ///         .mine_project_issue_clusters(
    ///             &"project_id".to_string(),
    ///             &MineProjectIssueClustersQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn mine_project_issue_clusters(
        &self,
        project_id: &str,
        request: &MineProjectIssueClustersQueryRequest,
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
                &format!("v1/projects/{}/issue-clusters/mine", project_id),
                None,
                QueryBuilder::new()
                    .serialize("since", request.since.clone())
                    .serialize("until", request.until.clone())
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
    ///         .issue_clusters
    ///         .get_issue_cluster(&"project_id".to_string(), &"cluster_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn get_issue_cluster(
        &self,
        project_id: &str,
        cluster_id: &str,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("v1/projects/{}/issue-clusters/{}", project_id, cluster_id),
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
    ///         .issue_clusters
    ///         .update_issue_cluster_status(
    ///             &"project_id".to_string(),
    ///             &"cluster_id".to_string(),
    ///             &UpdateIssueClusterStatusRequest {
    ///                 status: "status".to_string(),
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn update_issue_cluster_status(
        &self,
        project_id: &str,
        cluster_id: &str,
        request: &UpdateIssueClusterStatusRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        let options = {
            let mut o = options.unwrap_or_default();
            o.max_retries = Some(0);
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PATCH,
                &format!("v1/projects/{}/issue-clusters/{}", project_id, cluster_id),
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
    ///         .issue_clusters
    ///         .propose_issue_cluster_fix(&"project_id".to_string(), &"cluster_id".to_string(), None)
    ///         .await;
    /// }
    /// ```
    pub async fn propose_issue_cluster_fix(
        &self,
        project_id: &str,
        cluster_id: &str,
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
                    "v1/projects/{}/issue-clusters/{}/propose-fix",
                    project_id, cluster_id
                ),
                None,
                None,
                options,
            )
            .await
    }
}
