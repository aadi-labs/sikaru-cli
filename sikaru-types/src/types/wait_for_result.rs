pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ``bash.wait_for`` receipt payload; the deadline is a result, never an error.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WaitForResult {
    #[serde(default)]
    pub conditions: Vec<ConditionOutcome>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub elapsed_seconds: f64,
    pub status: WaitForResultStatus,
}

impl WaitForResult {
    pub fn builder() -> WaitForResultBuilder {
        <WaitForResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WaitForResultBuilder {
    conditions: Option<Vec<ConditionOutcome>>,
    elapsed_seconds: Option<f64>,
    status: Option<WaitForResultStatus>,
}

impl WaitForResultBuilder {
    pub fn conditions(mut self, value: Vec<ConditionOutcome>) -> Self {
        self.conditions = Some(value);
        self
    }

    pub fn elapsed_seconds(mut self, value: f64) -> Self {
        self.elapsed_seconds = Some(value);
        self
    }

    pub fn status(mut self, value: WaitForResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`WaitForResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`conditions`](WaitForResultBuilder::conditions)
    /// - [`elapsed_seconds`](WaitForResultBuilder::elapsed_seconds)
    /// - [`status`](WaitForResultBuilder::status)
    pub fn build(self) -> Result<WaitForResult, BuildError> {
        Ok(WaitForResult {
            conditions: self.conditions.ok_or_else(|| BuildError::missing_field("conditions"))?,
            elapsed_seconds: self.elapsed_seconds.ok_or_else(|| BuildError::missing_field("elapsed_seconds"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
