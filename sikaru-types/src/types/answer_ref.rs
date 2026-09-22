pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AnswerRef {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "conversationId")]
    #[serde(default)]
    pub conversation_id: String,
    #[serde(rename = "messageId")]
    #[serde(default)]
    pub message_id: String,
}

impl AnswerRef {
    pub fn builder() -> AnswerRefBuilder {
        <AnswerRefBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AnswerRefBuilder {
    account_id: Option<String>,
    conversation_id: Option<String>,
    message_id: Option<String>,
}

impl AnswerRefBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AnswerRef`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](AnswerRefBuilder::account_id)
    /// - [`conversation_id`](AnswerRefBuilder::conversation_id)
    /// - [`message_id`](AnswerRefBuilder::message_id)
    pub fn build(self) -> Result<AnswerRef, BuildError> {
        Ok(AnswerRef {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            conversation_id: self.conversation_id.ok_or_else(|| BuildError::missing_field("conversation_id"))?,
            message_id: self.message_id.ok_or_else(|| BuildError::missing_field("message_id"))?,
        })
    }
}
