pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OperationView {
    #[serde(default)]
    pub arguments: HashMap<String, serde_json::Value>,
    pub capability_name: OperationViewCapabilityName,
    pub method: OperationViewMethod,
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(default)]
    pub request_digest: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_provider_id: String,
    #[serde(default)]
    pub workspace_generation: String,
}

impl OperationView {
    pub fn builder() -> OperationViewBuilder {
        <OperationViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OperationViewBuilder {
    arguments: Option<HashMap<String, serde_json::Value>>,
    capability_name: Option<OperationViewCapabilityName>,
    method: Option<OperationViewMethod>,
    owner_epoch: Option<i64>,
    request_digest: Option<String>,
    run_id: Option<String>,
    tool_call_id: Option<String>,
    tool_provider_id: Option<String>,
    workspace_generation: Option<String>,
}

impl OperationViewBuilder {
    pub fn arguments(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(value);
        self
    }

    pub fn capability_name(mut self, value: OperationViewCapabilityName) -> Self {
        self.capability_name = Some(value);
        self
    }

    pub fn method(mut self, value: OperationViewMethod) -> Self {
        self.method = Some(value);
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

    pub fn tool_provider_id(mut self, value: impl Into<String>) -> Self {
        self.tool_provider_id = Some(value.into());
        self
    }

    pub fn workspace_generation(mut self, value: impl Into<String>) -> Self {
        self.workspace_generation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`OperationView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`arguments`](OperationViewBuilder::arguments)
    /// - [`capability_name`](OperationViewBuilder::capability_name)
    /// - [`method`](OperationViewBuilder::method)
    /// - [`owner_epoch`](OperationViewBuilder::owner_epoch)
    /// - [`request_digest`](OperationViewBuilder::request_digest)
    /// - [`run_id`](OperationViewBuilder::run_id)
    /// - [`tool_call_id`](OperationViewBuilder::tool_call_id)
    /// - [`tool_provider_id`](OperationViewBuilder::tool_provider_id)
    /// - [`workspace_generation`](OperationViewBuilder::workspace_generation)
    pub fn build(self) -> Result<OperationView, BuildError> {
        Ok(OperationView {
            arguments: self.arguments.ok_or_else(|| BuildError::missing_field("arguments"))?,
            capability_name: self.capability_name.ok_or_else(|| BuildError::missing_field("capability_name"))?,
            method: self.method.ok_or_else(|| BuildError::missing_field("method"))?,
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            request_digest: self.request_digest.ok_or_else(|| BuildError::missing_field("request_digest"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_provider_id: self.tool_provider_id.ok_or_else(|| BuildError::missing_field("tool_provider_id"))?,
            workspace_generation: self.workspace_generation.ok_or_else(|| BuildError::missing_field("workspace_generation"))?,
        })
    }
}
