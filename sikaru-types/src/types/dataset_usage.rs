pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetUsage {
    #[serde(default)]
    pub agents: Vec<String>,
    #[serde(default)]
    pub checks: i64,
}

impl DatasetUsage {
    pub fn builder() -> DatasetUsageBuilder {
        <DatasetUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetUsageBuilder {
    agents: Option<Vec<String>>,
    checks: Option<i64>,
}

impl DatasetUsageBuilder {
    pub fn agents(mut self, value: Vec<String>) -> Self {
        self.agents = Some(value);
        self
    }

    pub fn checks(mut self, value: i64) -> Self {
        self.checks = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agents`](DatasetUsageBuilder::agents)
    /// - [`checks`](DatasetUsageBuilder::checks)
    pub fn build(self) -> Result<DatasetUsage, BuildError> {
        Ok(DatasetUsage {
            agents: self.agents.ok_or_else(|| BuildError::missing_field("agents"))?,
            checks: self.checks.ok_or_else(|| BuildError::missing_field("checks"))?,
        })
    }
}
