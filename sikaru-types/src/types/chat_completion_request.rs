pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Generated from CreateChatCompletionRequest plus Sikaru extensions; unknown fields are rejected.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionRequestFunctionCall>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub functions: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_reasoning: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_completion_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(default)]
    pub messages: Vec<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modalities: Option<Vec<String>>,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prediction: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_retention: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<ChatCompletionRequestReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_history: Option<ChatCompletionRequestReasoningHistory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<ChatCompletionRequestStop>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ChatCompletionRequestToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typical_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_search_options: Option<HashMap<String, serde_json::Value>>,
}

impl ChatCompletionRequest {
    pub fn builder() -> ChatCompletionRequestBuilder {
        <ChatCompletionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionRequestBuilder {
    audio: Option<HashMap<String, serde_json::Value>>,
    frequency_penalty: Option<f64>,
    function_call: Option<ChatCompletionRequestFunctionCall>,
    functions: Option<Vec<HashMap<String, serde_json::Value>>>,
    include_reasoning: Option<bool>,
    logit_bias: Option<HashMap<String, serde_json::Value>>,
    logprobs: Option<bool>,
    max_completion_tokens: Option<i64>,
    max_tokens: Option<i64>,
    messages: Option<Vec<HashMap<String, serde_json::Value>>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    min_p: Option<f64>,
    modalities: Option<Vec<String>>,
    model: Option<String>,
    models: Option<Vec<String>>,
    moderation: Option<HashMap<String, serde_json::Value>>,
    n: Option<i64>,
    parallel_tool_calls: Option<bool>,
    prediction: Option<HashMap<String, serde_json::Value>>,
    presence_penalty: Option<f64>,
    prompt_cache_key: Option<String>,
    prompt_cache_options: Option<HashMap<String, serde_json::Value>>,
    prompt_cache_retention: Option<String>,
    provider: Option<HashMap<String, serde_json::Value>>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    reasoning_effort: Option<ChatCompletionRequestReasoningEffort>,
    reasoning_history: Option<ChatCompletionRequestReasoningHistory>,
    repetition_penalty: Option<f64>,
    response_format: Option<HashMap<String, serde_json::Value>>,
    route: Option<String>,
    safety_identifier: Option<String>,
    seed: Option<i64>,
    service_tier: Option<String>,
    stop: Option<ChatCompletionRequestStop>,
    store: Option<bool>,
    stream: Option<bool>,
    stream_options: Option<HashMap<String, serde_json::Value>>,
    temperature: Option<f64>,
    thinking: Option<HashMap<String, serde_json::Value>>,
    tool_choice: Option<ChatCompletionRequestToolChoice>,
    tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    top_k: Option<i64>,
    top_logprobs: Option<i64>,
    top_p: Option<f64>,
    transforms: Option<Vec<String>>,
    typical_p: Option<f64>,
    usage: Option<HashMap<String, serde_json::Value>>,
    user: Option<String>,
    verbosity: Option<String>,
    web_search_options: Option<HashMap<String, serde_json::Value>>,
}

impl ChatCompletionRequestBuilder {
    pub fn audio(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn function_call(mut self, value: ChatCompletionRequestFunctionCall) -> Self {
        self.function_call = Some(value);
        self
    }

    pub fn functions(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.functions = Some(value);
        self
    }

    pub fn include_reasoning(mut self, value: bool) -> Self {
        self.include_reasoning = Some(value);
        self
    }

    pub fn logit_bias(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.logit_bias = Some(value);
        self
    }

    pub fn logprobs(mut self, value: bool) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn max_completion_tokens(mut self, value: i64) -> Self {
        self.max_completion_tokens = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn min_p(mut self, value: f64) -> Self {
        self.min_p = Some(value);
        self
    }

    pub fn modalities(mut self, value: Vec<String>) -> Self {
        self.modalities = Some(value);
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn models(mut self, value: Vec<String>) -> Self {
        self.models = Some(value);
        self
    }

    pub fn moderation(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.moderation = Some(value);
        self
    }

    pub fn n(mut self, value: i64) -> Self {
        self.n = Some(value);
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn prediction(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.prediction = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn prompt_cache_key(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_key = Some(value.into());
        self
    }

    pub fn prompt_cache_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.prompt_cache_options = Some(value);
        self
    }

    pub fn prompt_cache_retention(mut self, value: impl Into<String>) -> Self {
        self.prompt_cache_retention = Some(value.into());
        self
    }

    pub fn provider(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn reasoning(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn reasoning_effort(mut self, value: ChatCompletionRequestReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn reasoning_history(mut self, value: ChatCompletionRequestReasoningHistory) -> Self {
        self.reasoning_history = Some(value);
        self
    }

    pub fn repetition_penalty(mut self, value: f64) -> Self {
        self.repetition_penalty = Some(value);
        self
    }

    pub fn response_format(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.response_format = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn safety_identifier(mut self, value: impl Into<String>) -> Self {
        self.safety_identifier = Some(value.into());
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn service_tier(mut self, value: impl Into<String>) -> Self {
        self.service_tier = Some(value.into());
        self
    }

    pub fn stop(mut self, value: ChatCompletionRequestStop) -> Self {
        self.stop = Some(value);
        self
    }

    pub fn store(mut self, value: bool) -> Self {
        self.store = Some(value);
        self
    }

    pub fn stream(mut self, value: bool) -> Self {
        self.stream = Some(value);
        self
    }

    pub fn stream_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.stream_options = Some(value);
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn thinking(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.thinking = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ChatCompletionRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
        self
    }

    pub fn top_logprobs(mut self, value: i64) -> Self {
        self.top_logprobs = Some(value);
        self
    }

    pub fn top_p(mut self, value: f64) -> Self {
        self.top_p = Some(value);
        self
    }

    pub fn transforms(mut self, value: Vec<String>) -> Self {
        self.transforms = Some(value);
        self
    }

    pub fn typical_p(mut self, value: f64) -> Self {
        self.typical_p = Some(value);
        self
    }

    pub fn usage(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.usage = Some(value);
        self
    }

    pub fn user(mut self, value: impl Into<String>) -> Self {
        self.user = Some(value.into());
        self
    }

    pub fn verbosity(mut self, value: impl Into<String>) -> Self {
        self.verbosity = Some(value.into());
        self
    }

    pub fn web_search_options(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.web_search_options = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`messages`](ChatCompletionRequestBuilder::messages)
    /// - [`model`](ChatCompletionRequestBuilder::model)
    pub fn build(self) -> Result<ChatCompletionRequest, BuildError> {
        Ok(ChatCompletionRequest {
            audio: self.audio,
            frequency_penalty: self.frequency_penalty,
            function_call: self.function_call,
            functions: self.functions,
            include_reasoning: self.include_reasoning,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            max_completion_tokens: self.max_completion_tokens,
            max_tokens: self.max_tokens,
            messages: self.messages.ok_or_else(|| BuildError::missing_field("messages"))?,
            metadata: self.metadata,
            min_p: self.min_p,
            modalities: self.modalities,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            models: self.models,
            moderation: self.moderation,
            n: self.n,
            parallel_tool_calls: self.parallel_tool_calls,
            prediction: self.prediction,
            presence_penalty: self.presence_penalty,
            prompt_cache_key: self.prompt_cache_key,
            prompt_cache_options: self.prompt_cache_options,
            prompt_cache_retention: self.prompt_cache_retention,
            provider: self.provider,
            reasoning: self.reasoning,
            reasoning_effort: self.reasoning_effort,
            reasoning_history: self.reasoning_history,
            repetition_penalty: self.repetition_penalty,
            response_format: self.response_format,
            route: self.route,
            safety_identifier: self.safety_identifier,
            seed: self.seed,
            service_tier: self.service_tier,
            stop: self.stop,
            store: self.store,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            thinking: self.thinking,
            tool_choice: self.tool_choice,
            tools: self.tools,
            top_k: self.top_k,
            top_logprobs: self.top_logprobs,
            top_p: self.top_p,
            transforms: self.transforms,
            typical_p: self.typical_p,
            usage: self.usage,
            user: self.user,
            verbosity: self.verbosity,
            web_search_options: self.web_search_options,
        })
    }
}
