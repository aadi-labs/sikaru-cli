pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunSpend {
    #[serde(default)]
    pub reserved_usd: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub used_usd: String,
}

impl RunSpend {
    pub fn builder() -> RunSpendBuilder {
        <RunSpendBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunSpendBuilder {
    reserved_usd: Option<String>,
    run_id: Option<String>,
    used_usd: Option<String>,
}

impl RunSpendBuilder {
    pub fn reserved_usd(mut self, value: impl Into<String>) -> Self {
        self.reserved_usd = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn used_usd(mut self, value: impl Into<String>) -> Self {
        self.used_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunSpend`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reserved_usd`](RunSpendBuilder::reserved_usd)
    /// - [`run_id`](RunSpendBuilder::run_id)
    /// - [`used_usd`](RunSpendBuilder::used_usd)
    pub fn build(self) -> Result<RunSpend, BuildError> {
        Ok(RunSpend {
            reserved_usd: self.reserved_usd.ok_or_else(|| BuildError::missing_field("reserved_usd"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            used_usd: self.used_usd.ok_or_else(|| BuildError::missing_field("used_usd"))?,
        })
    }
}
