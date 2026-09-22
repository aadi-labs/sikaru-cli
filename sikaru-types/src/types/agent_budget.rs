pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentBudget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_reload: Option<ReloadSettings>,
    #[serde(default)]
    pub funded_usd: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<FundingStatus>,
    #[serde(default)]
    pub payment_method_saved: bool,
    #[serde(default)]
    pub remaining_usd: String,
    #[serde(default)]
    pub reserved_usd: String,
    #[serde(default)]
    pub used_usd: String,
}

impl AgentBudget {
    pub fn builder() -> AgentBudgetBuilder {
        <AgentBudgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentBudgetBuilder {
    auto_reload: Option<ReloadSettings>,
    funded_usd: Option<String>,
    funding: Option<FundingStatus>,
    payment_method_saved: Option<bool>,
    remaining_usd: Option<String>,
    reserved_usd: Option<String>,
    used_usd: Option<String>,
}

impl AgentBudgetBuilder {
    pub fn auto_reload(mut self, value: ReloadSettings) -> Self {
        self.auto_reload = Some(value);
        self
    }

    pub fn funded_usd(mut self, value: impl Into<String>) -> Self {
        self.funded_usd = Some(value.into());
        self
    }

    pub fn funding(mut self, value: FundingStatus) -> Self {
        self.funding = Some(value);
        self
    }

    pub fn payment_method_saved(mut self, value: bool) -> Self {
        self.payment_method_saved = Some(value);
        self
    }

    pub fn remaining_usd(mut self, value: impl Into<String>) -> Self {
        self.remaining_usd = Some(value.into());
        self
    }

    pub fn reserved_usd(mut self, value: impl Into<String>) -> Self {
        self.reserved_usd = Some(value.into());
        self
    }

    pub fn used_usd(mut self, value: impl Into<String>) -> Self {
        self.used_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentBudget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`funded_usd`](AgentBudgetBuilder::funded_usd)
    /// - [`payment_method_saved`](AgentBudgetBuilder::payment_method_saved)
    /// - [`remaining_usd`](AgentBudgetBuilder::remaining_usd)
    /// - [`reserved_usd`](AgentBudgetBuilder::reserved_usd)
    /// - [`used_usd`](AgentBudgetBuilder::used_usd)
    pub fn build(self) -> Result<AgentBudget, BuildError> {
        Ok(AgentBudget {
            auto_reload: self.auto_reload,
            funded_usd: self.funded_usd.ok_or_else(|| BuildError::missing_field("funded_usd"))?,
            funding: self.funding,
            payment_method_saved: self.payment_method_saved.ok_or_else(|| BuildError::missing_field("payment_method_saved"))?,
            remaining_usd: self.remaining_usd.ok_or_else(|| BuildError::missing_field("remaining_usd"))?,
            reserved_usd: self.reserved_usd.ok_or_else(|| BuildError::missing_field("reserved_usd"))?,
            used_usd: self.used_usd.ok_or_else(|| BuildError::missing_field("used_usd"))?,
        })
    }
}
