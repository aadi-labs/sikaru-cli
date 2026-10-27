pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateChatCompletionStreamResponse fields at this level (CHAT_STREAM_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletionChunk {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ChatCompletionChunkChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,
}

impl ChatCompletionChunk {
    pub fn builder() -> ChatCompletionChunkBuilder {
        <ChatCompletionChunkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionChunkBuilder {
    choices: Option<Vec<ChatCompletionChunkChoice>>,
    created: Option<i64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<CompletionUsage>,
}

impl ChatCompletionChunkBuilder {
    pub fn choices(mut self, value: Vec<ChatCompletionChunkChoice>) -> Self {
        self.choices = Some(value);
        self
    }

    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn usage(mut self, value: CompletionUsage) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ChatCompletionChunk`].
    pub fn build(self) -> Result<ChatCompletionChunk, BuildError> {
        Ok(ChatCompletionChunk {
            choices: self.choices,
            created: self.created,
            id: self.id,
            model: self.model,
            object: self.object,
            usage: self.usage,
        })
    }
}
