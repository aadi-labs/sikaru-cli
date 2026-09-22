pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentImportModelCaptureRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub mode: String,
}

impl AgentImportModelCaptureRequest {
    pub fn builder() -> AgentImportModelCaptureRequestBuilder {
        <AgentImportModelCaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentImportModelCaptureRequestBuilder {
    metadata: Option<HashMap<String, serde_json::Value>>,
    mode: Option<String>,
}

impl AgentImportModelCaptureRequestBuilder {
    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentImportModelCaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](AgentImportModelCaptureRequestBuilder::mode)
    pub fn build(self) -> Result<AgentImportModelCaptureRequest, BuildError> {
        Ok(AgentImportModelCaptureRequest {
            metadata: self.metadata,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
        })
    }
}
