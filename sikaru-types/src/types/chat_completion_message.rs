pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateChatCompletionResponse fields at this level (CHAT_MESSAGE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_content: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_calls: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl ChatCompletionMessage {
    pub fn builder() -> ChatCompletionMessageBuilder {
        <ChatCompletionMessageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionMessageBuilder {
    annotations: Option<Vec<HashMap<String, serde_json::Value>>>,
    audio: Option<HashMap<String, serde_json::Value>>,
    content: Option<String>,
    function_call: Option<HashMap<String, serde_json::Value>>,
    reasoning_content: Option<String>,
    refusal: Option<String>,
    role: Option<String>,
    tool_calls: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl ChatCompletionMessageBuilder {
    pub fn annotations(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.annotations = Some(value);
        self
    }

    pub fn audio(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.audio = Some(value);
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn function_call(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.function_call = Some(value);
        self
    }

    pub fn reasoning_content(mut self, value: impl Into<String>) -> Self {
        self.reasoning_content = Some(value.into());
        self
    }

    pub fn refusal(mut self, value: impl Into<String>) -> Self {
        self.refusal = Some(value.into());
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn tool_calls(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionMessage`].
    pub fn build(self) -> Result<ChatCompletionMessage, BuildError> {
        Ok(ChatCompletionMessage {
            annotations: self.annotations,
            audio: self.audio,
            content: self.content,
            function_call: self.function_call,
            reasoning_content: self.reasoning_content,
            refusal: self.refusal,
            role: self.role,
            tool_calls: self.tool_calls,
        })
    }
}
