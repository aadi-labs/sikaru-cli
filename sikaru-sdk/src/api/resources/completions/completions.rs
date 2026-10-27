use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, RequestOptions, SseStream};
use reqwest::Method;

pub struct CompletionsClient {
    pub http_client: HttpClient,
}

impl CompletionsClient {
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
    ///         .completions
    ///         .create_stream(
    ///             &CompletionCreateParamsStreaming {
    ///                 model: "model".to_string(),
    ///                 prompt: CreateCompletionsStreamRequestPrompt::String("prompt".to_string()),
    ///                 stream: true,
    ///                 best_of: None,
    ///                 echo: None,
    ///                 frequency_penalty: None,
    ///                 logit_bias: None,
    ///                 logprobs: None,
    ///                 max_tokens: None,
    ///                 min_p: None,
    ///                 models: None,
    ///                 n: None,
    ///                 presence_penalty: None,
    ///                 provider: None,
    ///                 repetition_penalty: None,
    ///                 route: None,
    ///                 seed: None,
    ///                 stop: None,
    ///                 stream_options: None,
    ///                 suffix: None,
    ///                 temperature: None,
    ///                 top_k: None,
    ///                 top_p: None,
    ///                 transforms: None,
    ///                 typical_p: None,
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
        request: &CompletionCreateParamsStreaming,
        options: Option<RequestOptions>,
    ) -> Result<SseStream<Completion>, ApiError> {
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
                "v1/completions",
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
    ///         .completions
    ///         .create(
    ///             &CompletionCreateParams {
    ///                 model: "model".to_string(),
    ///                 prompt: CreateCompletionsRequestPrompt::String("prompt".to_string()),
    ///                 stream: false,
    ///                 best_of: None,
    ///                 echo: None,
    ///                 frequency_penalty: None,
    ///                 logit_bias: None,
    ///                 logprobs: None,
    ///                 max_tokens: None,
    ///                 min_p: None,
    ///                 models: None,
    ///                 n: None,
    ///                 presence_penalty: None,
    ///                 provider: None,
    ///                 repetition_penalty: None,
    ///                 route: None,
    ///                 seed: None,
    ///                 stop: None,
    ///                 stream_options: None,
    ///                 suffix: None,
    ///                 temperature: None,
    ///                 top_k: None,
    ///                 top_p: None,
    ///                 transforms: None,
    ///                 typical_p: None,
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
        request: &CompletionCreateParams,
        options: Option<RequestOptions>,
    ) -> Result<Completion, ApiError> {
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
                "v1/completions",
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }
}
