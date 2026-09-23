pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionPlanChange {
    #[serde(default)]
    pub effective_at: i64,
    #[serde(default)]
    pub id: String,
    pub plan: SubscriptionPlanChangePlan,
    #[serde(default)]
    pub state: String,
}

impl SubscriptionPlanChange {
    pub fn builder() -> SubscriptionPlanChangeBuilder {
        <SubscriptionPlanChangeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionPlanChangeBuilder {
    effective_at: Option<i64>,
    id: Option<String>,
    plan: Option<SubscriptionPlanChangePlan>,
    state: Option<String>,
}

impl SubscriptionPlanChangeBuilder {
    pub fn effective_at(mut self, value: i64) -> Self {
        self.effective_at = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn plan(mut self, value: SubscriptionPlanChangePlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionPlanChange`].
    /// This method will fail if any of the following fields are not set:
    /// - [`effective_at`](SubscriptionPlanChangeBuilder::effective_at)
    /// - [`id`](SubscriptionPlanChangeBuilder::id)
    /// - [`plan`](SubscriptionPlanChangeBuilder::plan)
    /// - [`state`](SubscriptionPlanChangeBuilder::state)
    pub fn build(self) -> Result<SubscriptionPlanChange, BuildError> {
        Ok(SubscriptionPlanChange {
            effective_at: self.effective_at.ok_or_else(|| BuildError::missing_field("effective_at"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            plan: self.plan.ok_or_else(|| BuildError::missing_field("plan"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
