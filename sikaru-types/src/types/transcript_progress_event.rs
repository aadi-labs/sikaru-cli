pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptProgressEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocks: Option<Vec<TranscriptProgressBlock>>,
    #[serde(default)]
    pub id: String,
    pub kind: TranscriptProgressEventKind,
    #[serde(default)]
    pub label: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub measurements: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "observedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<String>,
    pub ordering: TranscriptProgressEventOrdering,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub waiting: Option<FundingWait>,
}

impl TranscriptProgressEvent {
    pub fn builder() -> TranscriptProgressEventBuilder {
        <TranscriptProgressEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptProgressEventBuilder {
    blocks: Option<Vec<TranscriptProgressBlock>>,
    id: Option<String>,
    kind: Option<TranscriptProgressEventKind>,
    label: Option<String>,
    measurements: Option<HashMap<String, serde_json::Value>>,
    observed_at: Option<String>,
    ordering: Option<TranscriptProgressEventOrdering>,
    role: Option<String>,
    status: Option<String>,
    waiting: Option<FundingWait>,
}

impl TranscriptProgressEventBuilder {
    pub fn blocks(mut self, value: Vec<TranscriptProgressBlock>) -> Self {
        self.blocks = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: TranscriptProgressEventKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn measurements(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.measurements = Some(value);
        self
    }

    pub fn observed_at(mut self, value: impl Into<String>) -> Self {
        self.observed_at = Some(value.into());
        self
    }

    pub fn ordering(mut self, value: TranscriptProgressEventOrdering) -> Self {
        self.ordering = Some(value);
        self
    }

    pub fn role(mut self, value: impl Into<String>) -> Self {
        self.role = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn waiting(mut self, value: FundingWait) -> Self {
        self.waiting = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptProgressEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](TranscriptProgressEventBuilder::id)
    /// - [`kind`](TranscriptProgressEventBuilder::kind)
    /// - [`label`](TranscriptProgressEventBuilder::label)
    /// - [`ordering`](TranscriptProgressEventBuilder::ordering)
    pub fn build(self) -> Result<TranscriptProgressEvent, BuildError> {
        Ok(TranscriptProgressEvent {
            blocks: self.blocks,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            measurements: self.measurements,
            observed_at: self.observed_at,
            ordering: self.ordering.ok_or_else(|| BuildError::missing_field("ordering"))?,
            role: self.role,
            status: self.status,
            waiting: self.waiting,
        })
    }
}
