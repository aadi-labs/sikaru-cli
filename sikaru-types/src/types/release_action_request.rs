pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseActionRequest {
    /// Promote even though no check evidence is passing yet.
    #[serde(rename = "acknowledgeMissingEvidence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_missing_evidence: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "releaseTarget")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_target: Option<String>,
}

impl ReleaseActionRequest {
    pub fn builder() -> ReleaseActionRequestBuilder {
        <ReleaseActionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReleaseActionRequestBuilder {
    acknowledge_missing_evidence: Option<bool>,
    reason: Option<String>,
    release_target: Option<String>,
}

impl ReleaseActionRequestBuilder {
    pub fn acknowledge_missing_evidence(mut self, value: bool) -> Self {
        self.acknowledge_missing_evidence = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn release_target(mut self, value: impl Into<String>) -> Self {
        self.release_target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseActionRequest`].
    pub fn build(self) -> Result<ReleaseActionRequest, BuildError> {
        Ok(ReleaseActionRequest {
            acknowledge_missing_evidence: self.acknowledge_missing_evidence,
            reason: self.reason,
            release_target: self.release_target,
        })
    }
}
