pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResumeCounts {
    #[serde(default)]
    pub ready: i64,
    #[serde(rename = "readyReasons")]
    #[serde(default)]
    pub ready_reasons: HashMap<String, i64>,
    #[serde(default)]
    pub reasons: HashMap<String, i64>,
    #[serde(default)]
    pub total: i64,
}

impl ResumeCounts {
    pub fn builder() -> ResumeCountsBuilder {
        <ResumeCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeCountsBuilder {
    ready: Option<i64>,
    ready_reasons: Option<HashMap<String, i64>>,
    reasons: Option<HashMap<String, i64>>,
    total: Option<i64>,
}

impl ResumeCountsBuilder {
    pub fn ready(mut self, value: i64) -> Self {
        self.ready = Some(value);
        self
    }

    pub fn ready_reasons(mut self, value: HashMap<String, i64>) -> Self {
        self.ready_reasons = Some(value);
        self
    }

    pub fn reasons(mut self, value: HashMap<String, i64>) -> Self {
        self.reasons = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResumeCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ready`](ResumeCountsBuilder::ready)
    /// - [`ready_reasons`](ResumeCountsBuilder::ready_reasons)
    /// - [`reasons`](ResumeCountsBuilder::reasons)
    /// - [`total`](ResumeCountsBuilder::total)
    pub fn build(self) -> Result<ResumeCounts, BuildError> {
        Ok(ResumeCounts {
            ready: self.ready.ok_or_else(|| BuildError::missing_field("ready"))?,
            ready_reasons: self.ready_reasons.ok_or_else(|| BuildError::missing_field("ready_reasons"))?,
            reasons: self.reasons.ok_or_else(|| BuildError::missing_field("reasons"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
        })
    }
}
