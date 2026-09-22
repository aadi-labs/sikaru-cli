pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SessionSpend {
    #[serde(default)]
    pub reserved_usd: String,
    #[serde(default)]
    pub runs: Vec<RunSpend>,
    #[serde(default)]
    pub used_usd: String,
}

impl SessionSpend {
    pub fn builder() -> SessionSpendBuilder {
        <SessionSpendBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SessionSpendBuilder {
    reserved_usd: Option<String>,
    runs: Option<Vec<RunSpend>>,
    used_usd: Option<String>,
}

impl SessionSpendBuilder {
    pub fn reserved_usd(mut self, value: impl Into<String>) -> Self {
        self.reserved_usd = Some(value.into());
        self
    }

    pub fn runs(mut self, value: Vec<RunSpend>) -> Self {
        self.runs = Some(value);
        self
    }

    pub fn used_usd(mut self, value: impl Into<String>) -> Self {
        self.used_usd = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SessionSpend`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reserved_usd`](SessionSpendBuilder::reserved_usd)
    /// - [`runs`](SessionSpendBuilder::runs)
    /// - [`used_usd`](SessionSpendBuilder::used_usd)
    pub fn build(self) -> Result<SessionSpend, BuildError> {
        Ok(SessionSpend {
            reserved_usd: self.reserved_usd.ok_or_else(|| BuildError::missing_field("reserved_usd"))?,
            runs: self.runs.ok_or_else(|| BuildError::missing_field("runs"))?,
            used_usd: self.used_usd.ok_or_else(|| BuildError::missing_field("used_usd"))?,
        })
    }
}
