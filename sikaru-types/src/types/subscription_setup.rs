pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct SubscriptionSetup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_at: Option<i64>,
    #[serde(default)]
    pub configured: bool,
    pub mode: SubscriptionSetupMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<SubscriptionSetupPlan>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_change: Option<SubscriptionPlanChange>,
    #[serde(default)]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<TrialStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

impl SubscriptionSetup {
    pub fn builder() -> SubscriptionSetupBuilder {
        <SubscriptionSetupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionSetupBuilder {
    cancel_at: Option<i64>,
    configured: Option<bool>,
    mode: Option<SubscriptionSetupMode>,
    plan: Option<SubscriptionSetupPlan>,
    plan_change: Option<SubscriptionPlanChange>,
    state: Option<String>,
    trial: Option<TrialStatus>,
    url: Option<String>,
}

impl SubscriptionSetupBuilder {
    pub fn cancel_at(mut self, value: i64) -> Self {
        self.cancel_at = Some(value);
        self
    }

    pub fn configured(mut self, value: bool) -> Self {
        self.configured = Some(value);
        self
    }

    pub fn mode(mut self, value: SubscriptionSetupMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn plan(mut self, value: SubscriptionSetupPlan) -> Self {
        self.plan = Some(value);
        self
    }

    pub fn plan_change(mut self, value: SubscriptionPlanChange) -> Self {
        self.plan_change = Some(value);
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

    pub fn url(mut self, value: impl Into<String>) -> Self {
        self.url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionSetup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`configured`](SubscriptionSetupBuilder::configured)
    /// - [`mode`](SubscriptionSetupBuilder::mode)
    /// - [`state`](SubscriptionSetupBuilder::state)
    pub fn build(self) -> Result<SubscriptionSetup, BuildError> {
        Ok(SubscriptionSetup {
            cancel_at: self.cancel_at,
            configured: self.configured.ok_or_else(|| BuildError::missing_field("configured"))?,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            plan: self.plan,
            plan_change: self.plan_change,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            trial: self.trial,
            url: self.url,
        })
    }
}
