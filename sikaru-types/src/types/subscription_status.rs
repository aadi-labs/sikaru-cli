pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<i64>,
    #[serde(default)]
    pub configured: bool,
    pub mode: SubscriptionStatusMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<SubscriptionStatusPlan>,
    #[serde(default)]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<TrialStatus>,
}

impl SubscriptionStatus {
    pub fn builder() -> SubscriptionStatusBuilder {
        <SubscriptionStatusBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionStatusBuilder {
    cancel_at: Option<i64>,
    configured: Option<bool>,
    mode: Option<SubscriptionStatusMode>,
    plan: Option<SubscriptionStatusPlan>,
    state: Option<String>,
    trial: Option<TrialStatus>,
}

impl SubscriptionStatusBuilder {
    pub fn cancel_at(mut self, value: i64) -> Self {
        self.cancel_at = Some(value);
        self
    }

    pub fn configured(mut self, value: bool) -> Self {
        self.configured = Some(value);
        self
    }

    pub fn mode(mut self, value: SubscriptionStatusMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn plan(mut self, value: SubscriptionStatusPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn trial(mut self, value: TrialStatus) -> Self {
        self.trial = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionStatus`].
    /// This method will fail if any of the following fields are not set:
    /// - [`configured`](SubscriptionStatusBuilder::configured)
    /// - [`mode`](SubscriptionStatusBuilder::mode)
    /// - [`state`](SubscriptionStatusBuilder::state)
    pub fn build(self) -> Result<SubscriptionStatus, BuildError> {
        Ok(SubscriptionStatus {
            cancel_at: self.cancel_at,
            configured: self.configured.ok_or_else(|| BuildError::missing_field("configured"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            plan: self.plan,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            trial: self.trial,
        })
    }
}
