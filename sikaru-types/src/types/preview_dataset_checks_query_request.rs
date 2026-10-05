pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for preview_dataset_checks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewDatasetChecksQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl PreviewDatasetChecksQueryRequest {
    pub fn builder() -> PreviewDatasetChecksQueryRequestBuilder {
        <PreviewDatasetChecksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewDatasetChecksQueryRequestBuilder {
    agent: Option<String>,
    version: Option<i64>,
}

impl PreviewDatasetChecksQueryRequestBuilder {
    pub fn agent(mut self, value: impl Into<String>) -> Self {
        self.agent = Some(value.into());
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewDatasetChecksQueryRequest`].
    pub fn build(self) -> Result<PreviewDatasetChecksQueryRequest, BuildError> {
        Ok(PreviewDatasetChecksQueryRequest {
            agent: self.agent,
            version: self.version,
        })
    }
}

