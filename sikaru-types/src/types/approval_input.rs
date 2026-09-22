pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct ApprovalInput {
    pub decision: ApprovalInputDecision,
    #[serde(default)]
    pub idempotency_key: String,
}

impl ApprovalInput {
    pub fn builder() -> ApprovalInputBuilder {
        <ApprovalInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApprovalInputBuilder {
    decision: Option<ApprovalInputDecision>,
    idempotency_key: Option<String>,
}

impl ApprovalInputBuilder {
    pub fn decision(mut self, value: ApprovalInputDecision) -> Self {
        self.decision = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApprovalInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`decision`](ApprovalInputBuilder::decision)
    /// - [`idempotency_key`](ApprovalInputBuilder::idempotency_key)
    pub fn build(self) -> Result<ApprovalInput, BuildError> {
        Ok(ApprovalInput {
            decision: self.decision.ok_or_else(|| BuildError::missing_field("decision"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
        })
    }
}

