use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct ResponsesClient {
    pub http_client: HttpClient,
}

impl ResponsesClient {
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
    ///         .responses
    ///         .create_stream(
    ///             &ResponseCreateParamsStreaming {
    ///                 model: "model".to_string(),
    ///                 stream: true,
    ///                 access_programs: None,
    ///                 background: None,
    ///                 context_management: None,
    ///                 conversation: None,
    ///                 include: None,
    ///                 input: None,
    ///                 instructions: None,
    ///                 max_output_tokens: None,
    ///                 max_tool_calls: None,
    ///                 metadata: None,
    ///                 models: None,
    ///                 moderation: None,
    ///                 parallel_tool_calls: None,
    ///                 previous_response_id: None,
    ///                 prompt: None,
    ///                 prompt_cache_key: None,
    ///                 prompt_cache_options: None,
    ///                 prompt_cache_retention: None,
    ///                 provider: None,
    ///                 reasoning: None,
    ///                 route: None,
    ///                 safety_identifier: None,
    ///                 service_tier: None,
    ///                 store: None,
    ///                 stream_options: None,
    ///                 temperature: None,
    ///                 text: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_logprobs: None,
    ///                 top_p: None,
    ///                 transforms: None,
    ///                 truncation: None,
    ///                 usage: None,
    ///                 user: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_stream(
        &self,
        request: &ResponseCreateParamsStreaming,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<ResponseStreamEvent>, ApiError> {
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
            .execute_sse_request(
                Method::POST,
                "v1/responses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                None,
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
    ///         .responses
    ///         .create(
    ///             &ResponseCreateParams {
    ///                 model: "model".to_string(),
    ///                 stream: false,
    ///                 access_programs: None,
    ///                 background: None,
    ///                 context_management: None,
    ///                 conversation: None,
    ///                 include: None,
    ///                 input: None,
    ///                 instructions: None,
    ///                 max_output_tokens: None,
    ///                 max_tool_calls: None,
    ///                 metadata: None,
    ///                 models: None,
    ///                 moderation: None,
    ///                 parallel_tool_calls: None,
    ///                 previous_response_id: None,
    ///                 prompt: None,
    ///                 prompt_cache_key: None,
    ///                 prompt_cache_options: None,
    ///                 prompt_cache_retention: None,
    ///                 provider: None,
    ///                 reasoning: None,
    ///                 route: None,
    ///                 safety_identifier: None,
    ///                 service_tier: None,
    ///                 store: None,
    ///                 stream_options: None,
    ///                 temperature: None,
    ///                 text: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_logprobs: None,
    ///                 top_p: None,
    ///                 transforms: None,
    ///                 truncation: None,
    ///                 usage: None,
    ///                 user: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ResponseCreateParams,
        options: Option<RequestOptions>,
    ) -> Result<ModelResponse, ApiError> {
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
                "v1/responses",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
