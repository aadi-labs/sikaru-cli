pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReleaseActionRequest {
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
    release_target: Option<String>,
}

impl ReleaseActionRequestBuilder {
    pub fn release_target(mut self, value: impl Into<String>) -> Self {
        self.release_target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReleaseActionRequest`].
    pub fn build(self) -> Result<ReleaseActionRequest, BuildError> {
        Ok(ReleaseActionRequest {
            release_target: self.release_target,
        })
    }
}
