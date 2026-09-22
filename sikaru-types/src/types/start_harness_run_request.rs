pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StartHarnessRunRequest {
    /// Automatically request evaluated harness improvements after completed turns. Requires harness:write and configured improvement policy; active runs keep their pinned release.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_improve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capability_grants: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_environment_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compute_workspace_provenance: Option<WorkspaceProvenance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_delivery: Option<EventDeliveryRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub input: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    /// Catalog model for this run. Omit to use the project default; list choices through model settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub policy: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub product_context: HashMap<String, serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_mode: Option<StartHarnessRunRequestRunMode>,
    #[serde(default)]
    pub tenant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_provider_refs: Option<Vec<ToolProviderRefRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default)]
    pub user_id: String,
}

impl StartHarnessRunRequest {
    pub fn builder() -> StartHarnessRunRequestBuilder {
        <StartHarnessRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartHarnessRunRequestBuilder {
    auto_improve: Option<bool>,
    capability_grants: Option<Vec<String>>,
    compute_environment_id: Option<String>,
    compute_workspace_provenance: Option<WorkspaceProvenance>,
    conversation_id: Option<String>,
    correlation_id: Option<String>,
    event_delivery: Option<EventDeliveryRequest>,
    idempotency_key: Option<String>,
    input: Option<HashMap<String, serde_json::Value>>,
    job_id: Option<String>,
    model: Option<String>,
    policy: Option<HashMap<String, serde_json::Value>>,
    product_context: Option<HashMap<String, serde_json::Value>>,
    run_mode: Option<StartHarnessRunRequestRunMode>,
    tenant_id: Option<String>,
    tool_provider_refs: Option<Vec<ToolProviderRefRequest>>,
    trace_id: Option<String>,
    user_id: Option<String>,
}

impl StartHarnessRunRequestBuilder {
    pub fn auto_improve(mut self, value: bool) -> Self {
        self.auto_improve = Some(value);
        self
    }

    pub fn capability_grants(mut self, value: Vec<String>) -> Self {
        self.capability_grants = Some(value);
        self
    }

    pub fn compute_environment_id(mut self, value: impl Into<String>) -> Self {
        self.compute_environment_id = Some(value.into());
        self
    }

    pub fn compute_workspace_provenance(mut self, value: WorkspaceProvenance) -> Self {
        self.compute_workspace_provenance = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn event_delivery(mut self, value: EventDeliveryRequest) -> Self {
        self.event_delivery = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn input(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn policy(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.policy = Some(value);
        self
    }

    pub fn product_context(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.product_context = Some(value);
        self
    }

    pub fn run_mode(mut self, value: StartHarnessRunRequestRunMode) -> Self {
        self.run_mode = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn tool_provider_refs(mut self, value: Vec<ToolProviderRefRequest>) -> Self {
        self.tool_provider_refs = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartHarnessRunRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](StartHarnessRunRequestBuilder::input)
    /// - [`policy`](StartHarnessRunRequestBuilder::policy)
    /// - [`product_context`](StartHarnessRunRequestBuilder::product_context)
    /// - [`tenant_id`](StartHarnessRunRequestBuilder::tenant_id)
    /// - [`user_id`](StartHarnessRunRequestBuilder::user_id)
    pub fn build(self) -> Result<StartHarnessRunRequest, BuildError> {
        Ok(StartHarnessRunRequest {
            auto_improve: self.auto_improve,
            capability_grants: self.capability_grants,
            compute_environment_id: self.compute_environment_id,
            compute_workspace_provenance: self.compute_workspace_provenance,
            conversation_id: self.conversation_id,
            correlation_id: self.correlation_id,
            event_delivery: self.event_delivery,
            idempotency_key: self.idempotency_key,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            job_id: self.job_id,
            model: self.model,
            policy: self.policy.ok_or_else(|| BuildError::missing_field("policy"))?,
            product_context: self.product_context.ok_or_else(|| BuildError::missing_field("product_context"))?,
            run_mode: self.run_mode,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            tool_provider_refs: self.tool_provider_refs,
            trace_id: self.trace_id,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}

