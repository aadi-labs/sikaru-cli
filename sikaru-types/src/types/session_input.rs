pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionInput {
    /// Automatically request evaluated harness improvements after completed turns. Requires harness:write and configured improvement policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_improve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Draft sessions test the pinned agent definition without activation. Creating or appending draft sessions also requires harness:write.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<SessionInputEnvironment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Default model for this session. Use a Sikaru model catalog ID, such as kimi-k3. Omit to inherit the project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<SessionInputReasoningEffort>,
    #[serde(default)]
    pub tenant_id: String,
    #[serde(default)]
    pub user_id: String,
}

impl SessionInput {
    pub fn builder() -> SessionInputBuilder {
        <SessionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionInputBuilder {
    auto_improve: Option<bool>,
    conversation_id: Option<String>,
    environment: Option<SessionInputEnvironment>,
    final_output_schema: Option<HashMap<String, serde_json::Value>>,
    idempotency_key: Option<String>,
    model: Option<String>,
    reasoning_effort: Option<SessionInputReasoningEffort>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl SessionInputBuilder {
    pub fn auto_improve(mut self, value: bool) -> Self {
        self.auto_improve = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn environment(mut self, value: SessionInputEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn final_output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.final_output_schema = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn reasoning_effort(mut self, value: SessionInputReasoningEffort) -> Self {
        self.reasoning_effort = Some(value);
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`tenant_id`](SessionInputBuilder::tenant_id)
    /// - [`user_id`](SessionInputBuilder::user_id)
    pub fn build(self) -> Result<SessionInput, BuildError> {
        Ok(SessionInput {
            auto_improve: self.auto_improve,
            conversation_id: self.conversation_id,
            environment: self.environment,
            final_output_schema: self.final_output_schema,
            idempotency_key: self.idempotency_key,
            model: self.model,
            reasoning_effort: self.reasoning_effort,
            tenant_id: self.tenant_id.ok_or_else(|| BuildError::missing_field("tenant_id"))?,
            user_id: self.user_id.ok_or_else(|| BuildError::missing_field("user_id"))?,
        })
    }
}

