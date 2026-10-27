pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateChatCompletionResponse fields at this level (CHAT_RESPONSE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChatCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<ChatCompletionChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,
}

impl ChatCompletion {
    pub fn builder() -> ChatCompletionBuilder {
        <ChatCompletionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChatCompletionBuilder {
    choices: Option<Vec<ChatCompletionChoice>>,
    created: Option<i64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<CompletionUsage>,
}

impl ChatCompletionBuilder {
    pub fn choices(mut self, value: Vec<ChatCompletionChoice>) -> Self {
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

    /// Consumes the builder and constructs a [`ChatCompletion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChatCompletionBuilder::id)
    /// - [`object`](ChatCompletionBuilder::object)
    pub fn build(self) -> Result<ChatCompletion, BuildError> {
        Ok(ChatCompletion {
            choices: self.choices,
            created: self.created,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            model: self.model,
            object: self.object.ok_or_else(|| BuildError::missing_field("object"))?,
            usage: self.usage,
        })
    }
}
