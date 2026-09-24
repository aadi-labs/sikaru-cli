pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResourceBudget {
    /// Spending cap in USD; null means uncapped complimentary usage
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_usd: Option<String>,
    #[serde(default)]
    pub reserved_usd: String,
    #[serde(default)]
    pub used_usd: String,
}

impl ResourceBudget {
    pub fn builder() -> ResourceBudgetBuilder {
        <ResourceBudgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResourceBudgetBuilder {
    limit_usd: Option<String>,
    reserved_usd: Option<String>,
    used_usd: Option<String>,
}

impl ResourceBudgetBuilder {
    pub fn limit_usd(mut self, value: impl Into<String>) -> Self {
        self.limit_usd = Some(value.into());
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

    /// Consumes the builder and constructs a [`ResourceBudget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reserved_usd`](ResourceBudgetBuilder::reserved_usd)
    /// - [`used_usd`](ResourceBudgetBuilder::used_usd)
    pub fn build(self) -> Result<ResourceBudget, BuildError> {
        Ok(ResourceBudget {
            limit_usd: self.limit_usd,
            reserved_usd: self.reserved_usd.ok_or_else(|| BuildError::missing_field("reserved_usd"))?,
            used_usd: self.used_usd.ok_or_else(|| BuildError::missing_field("used_usd"))?,
        })
    }
}
