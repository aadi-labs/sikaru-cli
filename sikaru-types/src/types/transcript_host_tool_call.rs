pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptHostToolCall {
    pub arguments: serde_json::Value,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub operation_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(default)]
    pub status: String,
}

impl TranscriptHostToolCall {
    pub fn builder() -> TranscriptHostToolCallBuilder {
        <TranscriptHostToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptHostToolCallBuilder {
    arguments: Option<serde_json::Value>,
    name: Option<String>,
    operation_id: Option<String>,
    result: Option<serde_json::Value>,
    status: Option<String>,
}

impl TranscriptHostToolCallBuilder {
    pub fn arguments(mut self, value: serde_json::Value) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn operation_id(mut self, value: impl Into<String>) -> Self {
        self.operation_id = Some(value.into());
        self
    }

    pub fn result(mut self, value: serde_json::Value) -> Self {
        self.result = Some(value);
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptHostToolCall`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](TranscriptHostToolCallBuilder::arguments)
    /// - [`name`](TranscriptHostToolCallBuilder::name)
    /// - [`operation_id`](TranscriptHostToolCallBuilder::operation_id)
    /// - [`status`](TranscriptHostToolCallBuilder::status)
    pub fn build(self) -> Result<TranscriptHostToolCall, BuildError> {
        Ok(TranscriptHostToolCall {
            arguments: self.arguments.ok_or_else(|| BuildError::missing_field("arguments"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            operation_id: self.operation_id.ok_or_else(|| BuildError::missing_field("operation_id"))?,
            result: self.result,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
