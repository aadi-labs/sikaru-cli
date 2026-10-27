pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public Response fields at this level (RESPONSE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<ModelResponseInstructions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ModelResponseToolChoice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ResponseUsage>,
}

impl ModelResponse {
    pub fn builder() -> ModelResponseBuilder {
        <ModelResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelResponseBuilder {
    completed_at: Option<f64>,
    created_at: Option<f64>,
    error: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    incomplete_details: Option<HashMap<String, serde_json::Value>>,
    instructions: Option<ModelResponseInstructions>,
    max_output_tokens: Option<i64>,
    max_tool_calls: Option<i64>,
    model: Option<String>,
    object: Option<String>,
    output: Option<Vec<HashMap<String, serde_json::Value>>>,
    output_text: Option<String>,
    parallel_tool_calls: Option<bool>,
    reasoning: Option<HashMap<String, serde_json::Value>>,
    status: Option<String>,
    temperature: Option<f64>,
    text: Option<HashMap<String, serde_json::Value>>,
    tool_choice: Option<ModelResponseToolChoice>,
    tools: Option<Vec<HashMap<String, serde_json::Value>>>,
    top_logprobs: Option<i64>,
    top_p: Option<f64>,
    truncation: Option<String>,
    usage: Option<ResponseUsage>,
}

impl ModelResponseBuilder {
    pub fn completed_at(mut self, value: f64) -> Self {
        self.completed_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: f64) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn error(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.error = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn incomplete_details(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.incomplete_details = Some(value);
        self
    }

    pub fn instructions(mut self, value: ModelResponseInstructions) -> Self {
        self.instructions = Some(value);
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

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn output(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.output = Some(value);
        self
    }

    pub fn output_text(mut self, value: impl Into<String>) -> Self {
        self.output_text = Some(value.into());
        self
    }

    pub fn parallel_tool_calls(mut self, value: bool) -> Self {
        self.parallel_tool_calls = Some(value);
        self
    }

    pub fn reasoning(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
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

    pub fn tool_choice(mut self, value: ModelResponseToolChoice) -> Self {
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

    pub fn truncation(mut self, value: impl Into<String>) -> Self {
        self.truncation = Some(value.into());
        self
    }

    pub fn usage(mut self, value: ResponseUsage) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ModelResponseBuilder::id)
    /// - [`object`](ModelResponseBuilder::object)
    pub fn build(self) -> Result<ModelResponse, BuildError> {
        Ok(ModelResponse {
            completed_at: self.completed_at,
            created_at: self.created_at,
            error: self.error,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            incomplete_details: self.incomplete_details,
            instructions: self.instructions,
            max_output_tokens: self.max_output_tokens,
            max_tool_calls: self.max_tool_calls,
            model: self.model,
            object: self.object.ok_or_else(|| BuildError::missing_field("object"))?,
            output: self.output,
            output_text: self.output_text,
            parallel_tool_calls: self.parallel_tool_calls,
            reasoning: self.reasoning,
            status: self.status,
            temperature: self.temperature,
            text: self.text,
            tool_choice: self.tool_choice,
            tools: self.tools,
            top_logprobs: self.top_logprobs,
            top_p: self.top_p,
            truncation: self.truncation,
            usage: self.usage,
        })
    }
}
