pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReloadSettings {
    #[serde(default)]
    pub amount_usd: String,
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub threshold_usd: String,
}

impl ReloadSettings {
    pub fn builder() -> ReloadSettingsBuilder {
        <ReloadSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReloadSettingsBuilder {
    amount_usd: Option<String>,
    enabled: Option<bool>,
    threshold_usd: Option<String>,
}

impl ReloadSettingsBuilder {
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

    /// Consumes the builder and constructs a [`ReloadSettings`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount_usd`](ReloadSettingsBuilder::amount_usd)
    /// - [`enabled`](ReloadSettingsBuilder::enabled)
    /// - [`threshold_usd`](ReloadSettingsBuilder::threshold_usd)
    pub fn build(self) -> Result<ReloadSettings, BuildError> {
        Ok(ReloadSettings {
            amount_usd: self.amount_usd.ok_or_else(|| BuildError::missing_field("amount_usd"))?,
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
            threshold_usd: self.threshold_usd.ok_or_else(|| BuildError::missing_field("threshold_usd"))?,
        })
    }
}
