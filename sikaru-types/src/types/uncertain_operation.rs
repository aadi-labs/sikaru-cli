pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UncertainOperation {
    #[serde(default)]
    pub method: String,
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(default)]
    pub request_digest: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub workspace_generation: String,
}

impl UncertainOperation {
    pub fn builder() -> UncertainOperationBuilder {
        <UncertainOperationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UncertainOperationBuilder {
    method: Option<String>,
    owner_epoch: Option<i64>,
    request_digest: Option<String>,
    run_id: Option<String>,
    tool_call_id: Option<String>,
    workspace_generation: Option<String>,
}

impl UncertainOperationBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn owner_epoch(mut self, value: i64) -> Self {
        self.owner_epoch = Some(value);
        self
    }

    pub fn request_digest(mut self, value: impl Into<String>) -> Self {
        self.request_digest = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    pub fn workspace_generation(mut self, value: impl Into<String>) -> Self {
        self.workspace_generation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UncertainOperation`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](UncertainOperationBuilder::method)
    /// - [`owner_epoch`](UncertainOperationBuilder::owner_epoch)
    /// - [`request_digest`](UncertainOperationBuilder::request_digest)
    /// - [`run_id`](UncertainOperationBuilder::run_id)
    /// - [`tool_call_id`](UncertainOperationBuilder::tool_call_id)
    /// - [`workspace_generation`](UncertainOperationBuilder::workspace_generation)
    pub fn build(self) -> Result<UncertainOperation, BuildError> {
        Ok(UncertainOperation {
            method: self.method.ok_or_else(|| BuildError::missing_field("method"))?,
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            request_digest: self.request_digest.ok_or_else(|| BuildError::missing_field("request_digest"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            workspace_generation: self.workspace_generation.ok_or_else(|| BuildError::missing_field("workspace_generation"))?,
        })
    }
}
