pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionInput {
    #[serde(default)]
    pub accepted_recurring_terms: bool,
    #[serde(default)]
    pub idempotency_key: String,
    pub plan: SubscriptionInputPlan,
}

impl SubscriptionInput {
    pub fn builder() -> SubscriptionInputBuilder {
        <SubscriptionInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionInputBuilder {
    accepted_recurring_terms: Option<bool>,
    idempotency_key: Option<String>,
    plan: Option<SubscriptionInputPlan>,
}

impl SubscriptionInputBuilder {
    pub fn accepted_recurring_terms(mut self, value: bool) -> Self {
        self.accepted_recurring_terms = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn plan(mut self, value: SubscriptionInputPlan) -> Self {
        self.plan = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`accepted_recurring_terms`](SubscriptionInputBuilder::accepted_recurring_terms)
    /// - [`idempotency_key`](SubscriptionInputBuilder::idempotency_key)
    /// - [`plan`](SubscriptionInputBuilder::plan)
    pub fn build(self) -> Result<SubscriptionInput, BuildError> {
        Ok(SubscriptionInput {
            accepted_recurring_terms: self.accepted_recurring_terms.ok_or_else(|| BuildError::missing_field("accepted_recurring_terms"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
        })
    }
}

