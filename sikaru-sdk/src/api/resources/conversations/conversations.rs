use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;
use std::collections::HashMap;

pub struct ConversationsClient {
    pub http_client: HttpClient,
}

impl ConversationsClient {
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
    ///         .conversations
    ///         .list_messages(
    ///             &"project_id".to_string(),
    ///             &"conversation_id".to_string(),
    ///             &ListMessagesQueryRequest {
    ///                 account_id: "account_id".to_string(),
    ///                 limit: None,
    ///                 cursor: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_messages(
        &self,
        project_id: &str,
        conversation_id: &str,
        request: &ListMessagesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<HashMap<String, serde_json::Value>, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!(
                    "v1/projects/{}/conversations/{}/messages",
                    project_id, conversation_id
                ),
                None,
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .int("limit", request.limit.clone())
                    .serialize("cursor", request.cursor.clone())
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
    ///         .conversations
    ///         .record_message(
    ///             &"project_id".to_string(),
    ///             &"conversation_id".to_string(),
    ///             &DeliveredMessage {
    ///                 account_id: "account_id".to_string(),
    ///                 content: "content".to_string(),
    ///                 delivered_at: DateTime::parse_from_rfc3339("2024-01-15T09:30:00Z").unwrap(),
    ///                 message_id: "messageId".to_string(),
    ///                 position: 1,
    ///                 role: DeliveredMessageRole::User,
    ///                 environment: None,
    ///                 trace_id: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn record_message(
        &self,
        project_id: &str,
        conversation_id: &str,
        request: &DeliveredMessage,
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
                    "v1/projects/{}/conversations/{}/messages",
                    project_id, conversation_id
                ),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .string("account_id", request.account_id.clone())
                    .build(),
                options,
            )
            .await
    }
}
