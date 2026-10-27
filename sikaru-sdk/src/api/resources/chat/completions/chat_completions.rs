use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct CompletionsClient2 {
    pub http_client: HttpClient,
}

impl CompletionsClient2 {
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
    ///         .chat
    ///         .completions
    ///         .create_stream(
    ///             &ChatCompletionCreateParamsStreaming {
    ///                 messages: vec![HashMap::from([(
    ///                     "key".to_string(),
    ///                     serde_json::json!("value"),
    ///                 )])],
    ///                 model: "model".to_string(),
    ///                 stream: true,
    ///                 audio: None,
    ///                 frequency_penalty: None,
    ///                 function_call: None,
    ///                 functions: None,
    ///                 include_reasoning: None,
    ///                 logit_bias: None,
    ///                 logprobs: None,
    ///                 max_completion_tokens: None,
    ///                 max_tokens: None,
    ///                 metadata: None,
    ///                 min_p: None,
    ///                 modalities: None,
    ///                 models: None,
    ///                 moderation: None,
    ///                 n: None,
    ///                 parallel_tool_calls: None,
    ///                 prediction: None,
    ///                 presence_penalty: None,
    ///                 prompt_cache_key: None,
    ///                 prompt_cache_options: None,
    ///                 prompt_cache_retention: None,
    ///                 provider: None,
    ///                 reasoning: None,
    ///                 reasoning_effort: None,
    ///                 reasoning_history: None,
    ///                 repetition_penalty: None,
    ///                 response_format: None,
    ///                 route: None,
    ///                 safety_identifier: None,
    ///                 seed: None,
    ///                 service_tier: None,
    ///                 stop: None,
    ///                 store: None,
    ///                 stream_options: None,
    ///                 temperature: None,
    ///                 thinking: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_k: None,
    ///                 top_logprobs: None,
    ///                 top_p: None,
    ///                 transforms: None,
    ///                 typical_p: None,
    ///                 usage: None,
    ///                 user: None,
    ///                 verbosity: None,
    ///                 web_search_options: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create_stream(
        &self,
        request: &ChatCompletionCreateParamsStreaming,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<ChatCompletionChunk>, ApiError> {
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
                "v1/chat/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
                Some("[DONE]".to_string()),
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
    ///         .chat
    ///         .completions
    ///         .create(
    ///             &ChatCompletionCreateParams {
    ///                 messages: vec![HashMap::from([(
    ///                     "key".to_string(),
    ///                     serde_json::json!("value"),
    ///                 )])],
    ///                 model: "model".to_string(),
    ///                 stream: false,
    ///                 audio: None,
    ///                 frequency_penalty: None,
    ///                 function_call: None,
    ///                 functions: None,
    ///                 include_reasoning: None,
    ///                 logit_bias: None,
    ///                 logprobs: None,
    ///                 max_completion_tokens: None,
    ///                 max_tokens: None,
    ///                 metadata: None,
    ///                 min_p: None,
    ///                 modalities: None,
    ///                 models: None,
    ///                 moderation: None,
    ///                 n: None,
    ///                 parallel_tool_calls: None,
    ///                 prediction: None,
    ///                 presence_penalty: None,
    ///                 prompt_cache_key: None,
    ///                 prompt_cache_options: None,
    ///                 prompt_cache_retention: None,
    ///                 provider: None,
    ///                 reasoning: None,
    ///                 reasoning_effort: None,
    ///                 reasoning_history: None,
    ///                 repetition_penalty: None,
    ///                 response_format: None,
    ///                 route: None,
    ///                 safety_identifier: None,
    ///                 seed: None,
    ///                 service_tier: None,
    ///                 stop: None,
    ///                 store: None,
    ///                 stream_options: None,
    ///                 temperature: None,
    ///                 thinking: None,
    ///                 tool_choice: None,
    ///                 tools: None,
    ///                 top_k: None,
    ///                 top_logprobs: None,
    ///                 top_p: None,
    ///                 transforms: None,
    ///                 typical_p: None,
    ///                 usage: None,
    ///                 user: None,
    ///                 verbosity: None,
    ///                 web_search_options: None,
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &ChatCompletionCreateParams,
        options: Option<RequestOptions>,
    ) -> Result<ChatCompletion, ApiError> {
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
                "v1/chat/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
