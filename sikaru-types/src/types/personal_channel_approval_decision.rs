pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PersonalChannelApprovalDecision {
    #[serde(default)]
    pub capability_name: String,
    pub decision: PersonalChannelApprovalDecisionDecision,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub tool_call_id: String,
    #[serde(default)]
    pub tool_provider_id: String,
}

impl PersonalChannelApprovalDecision {
    pub fn builder() -> PersonalChannelApprovalDecisionBuilder {
        <PersonalChannelApprovalDecisionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalChannelApprovalDecisionBuilder {
    capability_name: Option<String>,
    decision: Option<PersonalChannelApprovalDecisionDecision>,
    idempotency_key: Option<String>,
    tool_call_id: Option<String>,
    tool_provider_id: Option<String>,
}

impl PersonalChannelApprovalDecisionBuilder {
    pub fn capability_name(mut self, value: impl Into<String>) -> Self {
        self.capability_name = Some(value.into());
        self
    }

    pub fn decision(mut self, value: PersonalChannelApprovalDecisionDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
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

    /// Consumes the builder and constructs a [`PersonalChannelApprovalDecision`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capability_name`](PersonalChannelApprovalDecisionBuilder::capability_name)
    /// - [`decision`](PersonalChannelApprovalDecisionBuilder::decision)
    /// - [`idempotency_key`](PersonalChannelApprovalDecisionBuilder::idempotency_key)
    /// - [`tool_call_id`](PersonalChannelApprovalDecisionBuilder::tool_call_id)
    /// - [`tool_provider_id`](PersonalChannelApprovalDecisionBuilder::tool_provider_id)
    pub fn build(self) -> Result<PersonalChannelApprovalDecision, BuildError> {
        Ok(PersonalChannelApprovalDecision {
            capability_name: self.capability_name.ok_or_else(|| BuildError::missing_field("capability_name"))?,
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
            tool_provider_id: self.tool_provider_id.ok_or_else(|| BuildError::missing_field("tool_provider_id"))?,
        })
    }
}
