pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SessionInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acknowledge_widening: Option<bool>,
    /// Automatically request evaluated harness improvements after completed turns. Requires harness:write and configured improvement policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_improve: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    /// Saved document revision to test. Required for document Draft sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub draft_revision: Option<i64>,
    /// Draft sessions test the pinned agent definition without activation. Creating or appending draft sessions also requires harness:write.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<SessionInputEnvironment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_access_digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    /// Default model for this session. Use a Sikaru model catalog ID, such as kimi-k3. Omit to inherit the project default.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning_effort: Option<SessionInputReasoningEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

impl SessionInput {
    pub fn builder() -> SessionInputBuilder {
        <SessionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionInputBuilder {
    acknowledge_widening: Option<bool>,
    auto_improve: Option<bool>,
    conversation_id: Option<String>,
    draft_revision: Option<i64>,
    environment: Option<SessionInputEnvironment>,
    expected_access_digest: Option<String>,
    final_output_schema: Option<HashMap<String, serde_json::Value>>,
    idempotency_key: Option<String>,
    model: Option<String>,
    reasoning_effort: Option<SessionInputReasoningEffort>,
    tenant_id: Option<String>,
    user_id: Option<String>,
}

impl SessionInputBuilder {
    pub fn acknowledge_widening(mut self, value: bool) -> Self {
        self.acknowledge_widening = Some(value);
        self
    }

    pub fn auto_improve(mut self, value: bool) -> Self {
        self.auto_improve = Some(value);
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn draft_revision(mut self, value: i64) -> Self {
        self.draft_revision = Some(value);
        self
    }

    pub fn environment(mut self, value: SessionInputEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn expected_access_digest(mut self, value: impl Into<String>) -> Self {
        self.expected_access_digest = Some(value.into());
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
    pub fn build(self) -> Result<SessionInput, BuildError> {
        Ok(SessionInput {
            acknowledge_widening: self.acknowledge_widening,
            auto_improve: self.auto_improve,
            conversation_id: self.conversation_id,
            draft_revision: self.draft_revision,
            environment: self.environment,
            expected_access_digest: self.expected_access_digest,
            final_output_schema: self.final_output_schema,
            idempotency_key: self.idempotency_key,
            model: self.model,
            reasoning_effort: self.reasoning_effort,
            tenant_id: self.tenant_id,
            user_id: self.user_id,
        })
    }
}

