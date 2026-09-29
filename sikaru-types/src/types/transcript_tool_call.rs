pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptToolCall {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
}

impl TranscriptToolCall {
    pub fn builder() -> TranscriptToolCallBuilder {
        <TranscriptToolCallBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptToolCallBuilder {
    arguments: Option<serde_json::Value>,
    function_name: Option<String>,
    tool_call_id: Option<String>,
}

impl TranscriptToolCallBuilder {
    pub fn arguments(mut self, value: serde_json::Value) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn function_name(mut self, value: impl Into<String>) -> Self {
        self.function_name = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptToolCall`].
    pub fn build(self) -> Result<TranscriptToolCall, BuildError> {
        Ok(TranscriptToolCall {
            arguments: self.arguments,
            function_name: self.function_name,
            tool_call_id: self.tool_call_id,
        })
    }
}
