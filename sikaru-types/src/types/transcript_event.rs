pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TranscriptEvent {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub sequence: i64,
}

impl TranscriptEvent {
    pub fn builder() -> TranscriptEventBuilder {
        <TranscriptEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptEventBuilder {
    created_at: Option<String>,
    id: Option<String>,
    label: Option<String>,
    sequence: Option<i64>,
}

impl TranscriptEventBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](TranscriptEventBuilder::created_at)
    /// - [`id`](TranscriptEventBuilder::id)
    /// - [`label`](TranscriptEventBuilder::label)
    /// - [`sequence`](TranscriptEventBuilder::sequence)
    pub fn build(self) -> Result<TranscriptEvent, BuildError> {
        Ok(TranscriptEvent {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
        })
    }
}
