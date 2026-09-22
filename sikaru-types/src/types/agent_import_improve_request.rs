pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentImportImproveRequest {
    #[serde(default)]
    pub mode: String,
    #[serde(default)]
    pub objective: String,
    #[serde(rename = "promotionGate")]
    #[serde(default)]
    pub promotion_gate: String,
}

impl AgentImportImproveRequest {
    pub fn builder() -> AgentImportImproveRequestBuilder {
        <AgentImportImproveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentImportImproveRequestBuilder {
    mode: Option<String>,
    objective: Option<String>,
    promotion_gate: Option<String>,
}

impl AgentImportImproveRequestBuilder {
    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    pub fn objective(mut self, value: impl Into<String>) -> Self {
        self.objective = Some(value.into());
        self
    }

    pub fn promotion_gate(mut self, value: impl Into<String>) -> Self {
        self.promotion_gate = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentImportImproveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mode`](AgentImportImproveRequestBuilder::mode)
    /// - [`objective`](AgentImportImproveRequestBuilder::objective)
    /// - [`promotion_gate`](AgentImportImproveRequestBuilder::promotion_gate)
    pub fn build(self) -> Result<AgentImportImproveRequest, BuildError> {
        Ok(AgentImportImproveRequest {
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            objective: self.objective.ok_or_else(|| BuildError::missing_field("objective"))?,
            promotion_gate: self.promotion_gate.ok_or_else(|| BuildError::missing_field("promotion_gate"))?,
        })
    }
}
