pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateChatCompletionStreamResponse fields at this level (CHAT_STREAM_CHOICE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionChunkChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<ChatCompletionDelta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<HashMap<String, serde_json::Value>>,
}

impl ChatCompletionChunkChoice {
    pub fn builder() -> ChatCompletionChunkChoiceBuilder {
        <ChatCompletionChunkChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionChunkChoiceBuilder {
    delta: Option<ChatCompletionDelta>,
    finish_reason: Option<String>,
    index: Option<i64>,
    logprobs: Option<HashMap<String, serde_json::Value>>,
}

impl ChatCompletionChunkChoiceBuilder {
    pub fn delta(mut self, value: ChatCompletionDelta) -> Self {
        self.delta = Some(value);
        self
    }

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

    /// Consumes the builder and constructs a [`ChatCompletionChunkChoice`].
    pub fn build(self) -> Result<ChatCompletionChunkChoice, BuildError> {
        Ok(ChatCompletionChunkChoice {
            delta: self.delta,
            finish_reason: self.finish_reason,
            index: self.index,
            logprobs: self.logprobs,
        })
    }
}
