pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ProcessObservation {
    #[serde(default)]
    pub evidence: String,
    #[serde(default)]
    pub handle_id: String,
    pub status: ProcessObservationStatus,
}

impl ProcessObservation {
    pub fn builder() -> ProcessObservationBuilder {
        <ProcessObservationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcessObservationBuilder {
    evidence: Option<String>,
    handle_id: Option<String>,
    status: Option<ProcessObservationStatus>,
}

impl ProcessObservationBuilder {
    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    pub fn handle_id(mut self, value: impl Into<String>) -> Self {
        self.handle_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ProcessObservationStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProcessObservation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evidence`](ProcessObservationBuilder::evidence)
    /// - [`handle_id`](ProcessObservationBuilder::handle_id)
    /// - [`status`](ProcessObservationBuilder::status)
    pub fn build(self) -> Result<ProcessObservation, BuildError> {
        Ok(ProcessObservation {
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
            handle_id: self.handle_id.ok_or_else(|| BuildError::missing_field("handle_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
