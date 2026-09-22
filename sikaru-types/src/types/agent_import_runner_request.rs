pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AgentImportRunnerRequest {
    #[serde(default)]
    pub entrypoint: String,
    #[serde(rename = "environmentRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_ref: Option<String>,
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl AgentImportRunnerRequest {
    pub fn builder() -> AgentImportRunnerRequestBuilder {
        <AgentImportRunnerRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentImportRunnerRequestBuilder {
    entrypoint: Option<String>,
    environment_ref: Option<String>,
    kind: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
}

impl AgentImportRunnerRequestBuilder {
    pub fn entrypoint(mut self, value: impl Into<String>) -> Self {
        self.entrypoint = Some(value.into());
        self
    }

    pub fn environment_ref(mut self, value: impl Into<String>) -> Self {
        self.environment_ref = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentImportRunnerRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entrypoint`](AgentImportRunnerRequestBuilder::entrypoint)
    /// - [`kind`](AgentImportRunnerRequestBuilder::kind)
    pub fn build(self) -> Result<AgentImportRunnerRequest, BuildError> {
        Ok(AgentImportRunnerRequest {
            entrypoint: self.entrypoint.ok_or_else(|| BuildError::missing_field("entrypoint"))?,
            environment_ref: self.environment_ref,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            metadata: self.metadata,
        })
    }
}
