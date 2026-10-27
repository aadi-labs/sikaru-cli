pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ``bash.wait_for`` arguments: canonical conditions and one deadline in seconds.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct WaitForArguments {
    #[serde(default)]
    pub conditions: Vec<WaitForArgumentsConditionsItem>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub timeout: f64,
}

impl WaitForArguments {
    pub fn builder() -> WaitForArgumentsBuilder {
        <WaitForArgumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitForArgumentsBuilder {
    conditions: Option<Vec<WaitForArgumentsConditionsItem>>,
    timeout: Option<f64>,
}

impl WaitForArgumentsBuilder {
    pub fn conditions(mut self, value: Vec<WaitForArgumentsConditionsItem>) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaitForArguments`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conditions`](WaitForArgumentsBuilder::conditions)
    /// - [`timeout`](WaitForArgumentsBuilder::timeout)
    pub fn build(self) -> Result<WaitForArguments, BuildError> {
        Ok(WaitForArguments {
            conditions: self.conditions.ok_or_else(|| BuildError::missing_field("conditions"))?,
            timeout: self.timeout.ok_or_else(|| BuildError::missing_field("timeout"))?,
        })
    }
}
