pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateIssueClusterStatusRequest {
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
    status: Option<String>,
}

impl UpdateIssueClusterStatusRequestBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UpdateIssueClusterStatusRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`status`](UpdateIssueClusterStatusRequestBuilder::status)
    pub fn build(self) -> Result<UpdateIssueClusterStatusRequest, BuildError> {
        Ok(UpdateIssueClusterStatusRequest {
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}

