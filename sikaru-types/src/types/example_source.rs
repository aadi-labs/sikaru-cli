pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ExampleSource {
    pub kind: ExampleSourceKind,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
    #[serde(rename = "seqFrom")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_from: Option<i64>,
    #[serde(rename = "seqTo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_to: Option<i64>,
}

impl ExampleSource {
    pub fn builder() -> ExampleSourceBuilder {
        <ExampleSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExampleSourceBuilder {
    kind: Option<ExampleSourceKind>,
    run_id: Option<String>,
    seq_from: Option<i64>,
    seq_to: Option<i64>,
}

impl ExampleSourceBuilder {
    pub fn kind(mut self, value: ExampleSourceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn seq_from(mut self, value: i64) -> Self {
        self.seq_from = Some(value);
        self
    }

    pub fn seq_to(mut self, value: i64) -> Self {
        self.seq_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExampleSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ExampleSourceBuilder::kind)
    pub fn build(self) -> Result<ExampleSource, BuildError> {
        Ok(ExampleSource {
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            run_id: self.run_id,
            seq_from: self.seq_from,
            seq_to: self.seq_to,
        })
    }
}
