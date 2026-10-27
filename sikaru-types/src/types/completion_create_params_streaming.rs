pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionCreateParamsStreaming {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_p: Option<f64>,
    #[serde(default)]
    pub model: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub models: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    pub prompt: CreateCompletionsStreamRequestPrompt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repetition_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seed: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<CreateCompletionsStreamRequestStop>,
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_k: Option<i64>,
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
}

impl CompletionCreateParamsStreaming {
    pub fn builder() -> CompletionCreateParamsStreamingBuilder {
        <CompletionCreateParamsStreamingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionCreateParamsStreamingBuilder {
    best_of: Option<i64>,
    echo: Option<bool>,
    frequency_penalty: Option<f64>,
    logit_bias: Option<HashMap<String, serde_json::Value>>,
    logprobs: Option<i64>,
    max_tokens: Option<i64>,
    min_p: Option<f64>,
    model: Option<String>,
    models: Option<Vec<String>>,
    n: Option<i64>,
    presence_penalty: Option<f64>,
    prompt: Option<CreateCompletionsStreamRequestPrompt>,
    provider: Option<HashMap<String, serde_json::Value>>,
    repetition_penalty: Option<f64>,
    route: Option<String>,
    seed: Option<i64>,
    stop: Option<CreateCompletionsStreamRequestStop>,
    stream: Option<bool>,
    stream_options: Option<HashMap<String, serde_json::Value>>,
    suffix: Option<String>,
    temperature: Option<f64>,
    top_k: Option<i64>,
    top_p: Option<f64>,
    transforms: Option<Vec<String>>,
    typical_p: Option<f64>,
    usage: Option<HashMap<String, serde_json::Value>>,
    user: Option<String>,
}

impl CompletionCreateParamsStreamingBuilder {
    pub fn best_of(mut self, value: i64) -> Self {
        self.best_of = Some(value);
        self
    }

    pub fn echo(mut self, value: bool) -> Self {
        self.echo = Some(value);
        self
    }

    pub fn frequency_penalty(mut self, value: f64) -> Self {
        self.frequency_penalty = Some(value);
        self
    }

    pub fn logit_bias(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.logit_bias = Some(value);
        self
    }

    pub fn logprobs(mut self, value: i64) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    pub fn min_p(mut self, value: f64) -> Self {
        self.min_p = Some(value);
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

    pub fn n(mut self, value: i64) -> Self {
        self.n = Some(value);
        self
    }

    pub fn presence_penalty(mut self, value: f64) -> Self {
        self.presence_penalty = Some(value);
        self
    }

    pub fn prompt(mut self, value: CreateCompletionsStreamRequestPrompt) -> Self {
        self.prompt = Some(value);
        self
    }

    pub fn provider(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn repetition_penalty(mut self, value: f64) -> Self {
        self.repetition_penalty = Some(value);
        self
    }

    pub fn route(mut self, value: impl Into<String>) -> Self {
        self.route = Some(value.into());
        self
    }

    pub fn seed(mut self, value: i64) -> Self {
        self.seed = Some(value);
        self
    }

    pub fn stop(mut self, value: CreateCompletionsStreamRequestStop) -> Self {
        self.stop = Some(value);
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

    pub fn suffix(mut self, value: impl Into<String>) -> Self {
        self.suffix = Some(value.into());
        self
    }

    pub fn temperature(mut self, value: f64) -> Self {
        self.temperature = Some(value);
        self
    }

    pub fn top_k(mut self, value: i64) -> Self {
        self.top_k = Some(value);
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

    /// Consumes the builder and constructs a [`CompletionCreateParamsStreaming`].
    /// This method will fail if any of the following fields are not set:
    /// - [`model`](CompletionCreateParamsStreamingBuilder::model)
    /// - [`prompt`](CompletionCreateParamsStreamingBuilder::prompt)
    /// - [`stream`](CompletionCreateParamsStreamingBuilder::stream)
    pub fn build(self) -> Result<CompletionCreateParamsStreaming, BuildError> {
        Ok(CompletionCreateParamsStreaming {
            best_of: self.best_of,
            echo: self.echo,
            frequency_penalty: self.frequency_penalty,
            logit_bias: self.logit_bias,
            logprobs: self.logprobs,
            max_tokens: self.max_tokens,
            min_p: self.min_p,
            model: self.model.ok_or_else(|| BuildError::missing_field("model"))?,
            models: self.models,
            n: self.n,
            presence_penalty: self.presence_penalty,
            prompt: self.prompt.ok_or_else(|| BuildError::missing_field("prompt"))?,
            provider: self.provider,
            repetition_penalty: self.repetition_penalty,
            route: self.route,
            seed: self.seed,
            stop: self.stop,
            stream: self.stream.ok_or_else(|| BuildError::missing_field("stream"))?,
            stream_options: self.stream_options,
            suffix: self.suffix,
            temperature: self.temperature,
            top_k: self.top_k,
            top_p: self.top_p,
            transforms: self.transforms,
            typical_p: self.typical_p,
            usage: self.usage,
            user: self.user,
        })
    }
}

