pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BatchItem {
    #[serde(rename = "exampleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example_id: Option<String>,
    pub outcome: BatchItemOutcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub row: Option<i64>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "seqTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_to: Option<i64>,
}

impl BatchItem {
    pub fn builder() -> BatchItemBuilder {
        <BatchItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchItemBuilder {
    example_id: Option<String>,
    outcome: Option<BatchItemOutcome>,
    reason: Option<String>,
    row: Option<i64>,
    run_id: Option<String>,
    seq_to: Option<i64>,
}

impl BatchItemBuilder {
    pub fn example_id(mut self, value: impl Into<String>) -> Self {
        self.example_id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: BatchItemOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn row(mut self, value: i64) -> Self {
        self.row = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn seq_to(mut self, value: i64) -> Self {
        self.seq_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`outcome`](BatchItemBuilder::outcome)
    pub fn build(self) -> Result<BatchItem, BuildError> {
        Ok(BatchItem {
            example_id: self.example_id,
            outcome: self.outcome.ok_or_else(|| BuildError::missing_field("outcome"))?,
            reason: self.reason,
            row: self.row,
            run_id: self.run_id,
            seq_to: self.seq_to,
        })
    }
}
