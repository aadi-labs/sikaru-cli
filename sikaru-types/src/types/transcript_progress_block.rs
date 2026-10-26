pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptProgressBlock {
    pub availability: TranscriptProgressBlockAvailability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TranscriptProgressBlockFormat>,
    pub kind: TranscriptProgressBlockKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

impl TranscriptProgressBlock {
    pub fn builder() -> TranscriptProgressBlockBuilder {
        <TranscriptProgressBlockBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptProgressBlockBuilder {
    availability: Option<TranscriptProgressBlockAvailability>,
    format: Option<TranscriptProgressBlockFormat>,
    kind: Option<TranscriptProgressBlockKind>,
    text: Option<String>,
    value: Option<serde_json::Value>,
}

impl TranscriptProgressBlockBuilder {
    pub fn availability(mut self, value: TranscriptProgressBlockAvailability) -> Self {
        self.availability = Some(value);
        self
    }

    pub fn format(mut self, value: TranscriptProgressBlockFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn kind(mut self, value: TranscriptProgressBlockKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn value(mut self, value: serde_json::Value) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptProgressBlock`].
    /// This method will fail if any of the following fields are not set:
    /// - [`availability`](TranscriptProgressBlockBuilder::availability)
    /// - [`kind`](TranscriptProgressBlockBuilder::kind)
    pub fn build(self) -> Result<TranscriptProgressBlock, BuildError> {
        Ok(TranscriptProgressBlock {
            availability: self.availability.ok_or_else(|| BuildError::missing_field("availability"))?,
            format: self.format,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            text: self.text,
            value: self.value,
        })
    }
}
