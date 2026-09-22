use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct FeedbackClient {
    pub http_client: HttpClient,
}

impl FeedbackClient {
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
    ///         .feedback
    ///         .create_feedback(
    ///             &"project_id".to_string(),
    ///             &CreateFeedbackRequest {
    ///                 kind: CreateFeedbackRequestKind::ThumbsUp,
    ///                 target: CreateFeedbackRequestTarget::Trace,
    ///                 target_id: "targetId".to_string(),
    ///                 issue_id: None,
    ///                 note: None,
    ///                 session_id: None,
    ///                 span_id: None,
    ///                 tag: None,
    ///                 trace_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_feedback(
        &self,
        project_id: &str,
        request: &CreateFeedbackRequest,
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
                &format!("v1/projects/{}/feedback", project_id),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
