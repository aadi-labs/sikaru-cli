pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateWorkflowVersionRequest {
    #[serde(rename = "releaseChannel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_channel: Option<CreateWorkflowVersionRequestReleaseChannel>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CreateWorkflowVersionRequestStatus>,
}

impl CreateWorkflowVersionRequest {
    pub fn builder() -> CreateWorkflowVersionRequestBuilder {
        <CreateWorkflowVersionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateWorkflowVersionRequestBuilder {
    release_channel: Option<CreateWorkflowVersionRequestReleaseChannel>,
    status: Option<CreateWorkflowVersionRequestStatus>,
}

impl CreateWorkflowVersionRequestBuilder {
    pub fn release_channel(mut self, value: CreateWorkflowVersionRequestReleaseChannel) -> Self {
        self.release_channel = Some(value);
        self
    }

    pub fn status(mut self, value: CreateWorkflowVersionRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateWorkflowVersionRequest`].
    pub fn build(self) -> Result<CreateWorkflowVersionRequest, BuildError> {
        Ok(CreateWorkflowVersionRequest {
            release_channel: self.release_channel,
            status: self.status,
        })
    }
}

