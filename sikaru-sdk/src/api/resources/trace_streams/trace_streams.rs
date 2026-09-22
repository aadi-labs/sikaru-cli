use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct TraceStreamsClient {
    pub http_client: HttpClient,
}

impl TraceStreamsClient {
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
    ///         .trace_streams
    ///         .stream_openinference_spans(
    ///             &TraceStreamRequest {
    ///                 format: TraceStreamRequestFormat::OpeninferenceV1,
    ///                 metadata: TraceMetadata {
    ///                     account_id: "account_id".to_string(),
    ///                     project_id: "project_id".to_string(),
    ///                     source: "source".to_string(),
    ///                     ..Default::default()
    ///                 },
    ///                 dataset: None,
    ///                 resource_spans: None,
    ///                 spans: None,
    ///                 trajectories: None,
    ///                 trajectory: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn stream_openinference_spans(
        &self,
        request: &TraceStreamRequest,
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
                "v1/trace-streams",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
