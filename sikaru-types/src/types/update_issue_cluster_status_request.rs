pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateIssueClusterStatusRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default)]
    pub status: String,
}

impl UpdateIssueClusterStatusRequest {
    pub fn builder() -> UpdateIssueClusterStatusRequestBuilder {
        <UpdateIssueClusterStatusRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateIssueClusterStatusRequestBuilder {
    reason: Option<String>,
    status: Option<String>,
}

impl UpdateIssueClusterStatusRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateIssueClusterStatusRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdateIssueClusterStatusRequestBuilder::status)
    pub fn build(self) -> Result<UpdateIssueClusterStatusRequest, BuildError> {
        Ok(UpdateIssueClusterStatusRequest {
            reason: self.reason,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

