pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecoverRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl RecoverRunRequest {
    pub fn builder() -> RecoverRunRequestBuilder {
        <RecoverRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecoverRunRequestBuilder {
    reason: Option<String>,
}

impl RecoverRunRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecoverRunRequest`].
    pub fn build(self) -> Result<RecoverRunRequest, BuildError> {
        Ok(RecoverRunRequest {
            reason: self.reason,
        })
    }
}
