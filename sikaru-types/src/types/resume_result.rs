pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ResumeResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    pub status: ResumeResultStatus,
}

impl ResumeResult {
    pub fn builder() -> ResumeResultBuilder {
        <ResumeResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResumeResultBuilder {
    reason: Option<String>,
    run_id: Option<String>,
    status: Option<ResumeResultStatus>,
}

impl ResumeResultBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: ResumeResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResumeResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](ResumeResultBuilder::run_id)
    /// - [`status`](ResumeResultBuilder::status)
    pub fn build(self) -> Result<ResumeResult, BuildError> {
        Ok(ResumeResult {
            reason: self.reason,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
