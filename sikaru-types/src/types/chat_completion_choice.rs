pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateChatCompletionResponse fields at this level (CHAT_CHOICE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<ChatCompletionMessage>,
}

impl ChatCompletionChoice {
    pub fn builder() -> ChatCompletionChoiceBuilder {
        <ChatCompletionChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionChoiceBuilder {
    finish_reason: Option<String>,
    index: Option<i64>,
    logprobs: Option<HashMap<String, serde_json::Value>>,
    message: Option<ChatCompletionMessage>,
}

impl ChatCompletionChoiceBuilder {
    pub fn finish_reason(mut self, value: impl Into<String>) -> Self {
        self.finish_reason = Some(value.into());
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn logprobs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn message(mut self, value: ChatCompletionMessage) -> Self {
        self.message = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionChoice`].
    pub fn build(self) -> Result<ChatCompletionChoice, BuildError> {
        Ok(ChatCompletionChoice {
            finish_reason: self.finish_reason,
            index: self.index,
            logprobs: self.logprobs,
            message: self.message,
        })
    }
}
