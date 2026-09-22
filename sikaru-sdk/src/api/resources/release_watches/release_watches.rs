use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ReleaseWatchesClient {
    pub http_client: HttpClient,
}

impl ReleaseWatchesClient {
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
    ///         .release_watches
    ///         .create_release_watch(
    ///             &"project_id".to_string(),
    ///             &CreateReleaseWatchRequest {
    ///                 trace_ids: vec!["traceIds".to_string()],
    ///                 agent_id: None,
    ///                 backfill_window: None,
    ///                 filter_expression: None,
    ///                 issue_id: None,
    ///                 issue_title: None,
    ///                 release_id: None,
    ///                 rule_id: None,
    ///                 rule_name: None,
    ///                 sample_rate: None,
    ///                 signal_tags: None,
    ///                 target: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_release_watch(
        &self,
        project_id: &str,
        request: &CreateReleaseWatchRequest,
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
                &format!("v1/projects/{}/release-watches", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
