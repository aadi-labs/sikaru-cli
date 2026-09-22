pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReloadInput {
    #[serde(default)]
    pub amount_usd: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub threshold_usd: String,
}

impl ReloadInput {
    pub fn builder() -> ReloadInputBuilder {
        <ReloadInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReloadInputBuilder {
    amount_usd: Option<String>,
    enabled: Option<bool>,
    threshold_usd: Option<String>,
}

impl ReloadInputBuilder {
    pub fn amount_usd(mut self, value: impl Into<String>) -> Self {
        self.amount_usd = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn threshold_usd(mut self, value: impl Into<String>) -> Self {
        self.threshold_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReloadInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_usd`](ReloadInputBuilder::amount_usd)
    /// - [`enabled`](ReloadInputBuilder::enabled)
    /// - [`threshold_usd`](ReloadInputBuilder::threshold_usd)
    pub fn build(self) -> Result<ReloadInput, BuildError> {
        Ok(ReloadInput {
            amount_usd: self.amount_usd.ok_or_else(|| BuildError::missing_field("amount_usd"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            threshold_usd: self.threshold_usd.ok_or_else(|| BuildError::missing_field("threshold_usd"))?,
        })
    }
}

