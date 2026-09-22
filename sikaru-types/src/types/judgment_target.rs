pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct JudgmentTarget {
    #[serde(rename = "accountId")]
    #[serde(default)]
    pub account_id: String,
    #[serde(rename = "conversationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    pub kind: JudgmentTargetKind,
    #[serde(rename = "messageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    #[serde(rename = "spanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

impl JudgmentTarget {
    pub fn builder() -> JudgmentTargetBuilder {
        <JudgmentTargetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgmentTargetBuilder {
    account_id: Option<String>,
    conversation_id: Option<String>,
    kind: Option<JudgmentTargetKind>,
    message_id: Option<String>,
    span_id: Option<String>,
}

impl JudgmentTargetBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: JudgmentTargetKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn message_id(mut self, value: impl Into<String>) -> Self {
        self.message_id = Some(value.into());
        self
    }

    pub fn span_id(mut self, value: impl Into<String>) -> Self {
        self.span_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JudgmentTarget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](JudgmentTargetBuilder::account_id)
    /// - [`kind`](JudgmentTargetBuilder::kind)
    pub fn build(self) -> Result<JudgmentTarget, BuildError> {
        Ok(JudgmentTarget {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            conversation_id: self.conversation_id,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            message_id: self.message_id,
            span_id: self.span_id,
        })
    }
}
