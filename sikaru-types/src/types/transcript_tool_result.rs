pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TranscriptToolResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_call_id: Option<String>,
}

impl TranscriptToolResult {
    pub fn builder() -> TranscriptToolResultBuilder {
        <TranscriptToolResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptToolResultBuilder {
    content: Option<serde_json::Value>,
    source_call_id: Option<String>,
}

impl TranscriptToolResultBuilder {
    pub fn content(mut self, value: serde_json::Value) -> Self {
        self.content = Some(value);
        self
    }

    pub fn source_call_id(mut self, value: impl Into<String>) -> Self {
        self.source_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TranscriptToolResult`].
    pub fn build(self) -> Result<TranscriptToolResult, BuildError> {
        Ok(TranscriptToolResult {
            content: self.content,
            source_call_id: self.source_call_id,
        })
    }
}
