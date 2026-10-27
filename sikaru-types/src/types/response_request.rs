pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Generated from CreateResponse plus Sikaru extensions; unknown fields are rejected.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponseRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_programs: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context_management: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation: Option<ResponseRequestConversation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<ResponseRequestInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moderation: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<HashMap<String, serde_json::Value>>,
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
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ResponseRequestToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl ResponseRequest {
    pub fn builder() -> ResponseRequestBuilder {
        <ResponseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseRequestBuilder {
    access_programs: Option<HashMap<String, serde_json::Value>>,
    background: Option<bool>,
    context_management: Option<Vec<HashMap<String, serde_json::Value>>>,
    conversation: Option<ResponseRequestConversation>,
    include: Option<Vec<String>>,
    input: Option<ResponseRequestInput>,
    instructions: Option<String>,
    max_output_tokens: Option<i64>,
    max_tool_calls: Option<i64>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    model: Option<String>,
    models: Option<Vec<String>>,
    moderation: Option<HashMap<String, serde_json::Value>>,
    parallel_tool_calls: Option<bool>,
    previous_response_id: Option<String>,
    prompt: Option<HashMap<String, serde_json::Value>>,
    prompt_cache_key: Option<String>,
    prompt_cache_options: Option<HashMap<String, serde_json::Value>>,
    prompt_cache_retention: Option<String>,
    provider: Option<HashMap<String, serde_json::Value>>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    route: Option<String>,
    safety_identifier: Option<String>,
    service_tier: Option<String>,
    store: Option<bool>,
    stream: Option<bool>,
    stream_options: Option<HashMap<String, serde_json::Value>>,
    temperature: Option<f64>,
    text: Option<HashMap<String, serde_json::Value>>,
    tool_choice: Option<ResponseRequestToolChoice>,
    tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    top_logprobs: Option<i64>,
    top_p: Option<f64>,
    transforms: Option<Vec<String>>,
    truncation: Option<String>,
    usage: Option<HashMap<String, serde_json::Value>>,
    user: Option<String>,
}

impl ResponseRequestBuilder {
    pub fn access_programs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.access_programs = Some(value);
        self
    }

    pub fn background(mut self, value: bool) -> Self {
        self.background = Some(value);
        self
    }

    pub fn context_management(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.context_management = Some(value);
        self
    }

    pub fn conversation(mut self, value: ResponseRequestConversation) -> Self {
        self.conversation = Some(value);
        self
    }

    pub fn include(mut self, value: Vec<String>) -> Self {
        self.include = Some(value);
        self
    }

    pub fn input(mut self, value: ResponseRequestInput) -> Self {
        self.input = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn max_output_tokens(mut self, value: i64) -> Self {
        self.max_output_tokens = Some(value);
        self
    }

    pub fn max_tool_calls(mut self, value: i64) -> Self {
        self.max_tool_calls = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
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

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn previous_response_id(mut self, value: impl Into<String>) -> Self {
        self.previous_response_id = Some(value.into());
        self
    }

    pub fn prompt(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.prompt = Some(value);
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

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn safety_identifier(mut self, value: impl Into<String>) -> Self {
        self.safety_identifier = Some(value.into());
        self
    }

    pub fn service_tier(mut self, value: impl Into<String>) -> Self {
        self.service_tier = Some(value.into());
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

    pub fn text(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.text = Some(value);
        self
    }

    pub fn tool_choice(mut self, value: ResponseRequestToolChoice) -> Self {
        self.tool_choice = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.tools = Some(value);
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

    pub fn truncation(mut self, value: impl Into<String>) -> Self {
        self.truncation = Some(value.into());
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

    /// Consumes the builder and constructs a [`ResponseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](ResponseRequestBuilder::model)
    pub fn build(self) -> Result<ResponseRequest, BuildError> {
        Ok(ResponseRequest {
            access_programs: self.access_programs,
            background: self.background,
            context_management: self.context_management,
            conversation: self.conversation,
            include: self.include,
            input: self.input,
            instructions: self.instructions,
            max_output_tokens: self.max_output_tokens,
            max_tool_calls: self.max_tool_calls,
            metadata: self.metadata,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            models: self.models,
            moderation: self.moderation,
            parallel_tool_calls: self.parallel_tool_calls,
            previous_response_id: self.previous_response_id,
            prompt: self.prompt,
            prompt_cache_key: self.prompt_cache_key,
            prompt_cache_options: self.prompt_cache_options,
            prompt_cache_retention: self.prompt_cache_retention,
            provider: self.provider,
            reasoning: self.reasoning,
            route: self.route,
            safety_identifier: self.safety_identifier,
            service_tier: self.service_tier,
            store: self.store,
            stream: self.stream,
            stream_options: self.stream_options,
            temperature: self.temperature,
            text: self.text,
            tool_choice: self.tool_choice,
            tools: self.tools,
            top_logprobs: self.top_logprobs,
            top_p: self.top_p,
            transforms: self.transforms,
            truncation: self.truncation,
            usage: self.usage,
            user: self.user,
        })
    }
}
