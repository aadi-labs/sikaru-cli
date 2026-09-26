pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One requested condition, echoed, with what fired or why it did not.
/// 
/// ``reason`` is null when fired, otherwise ``deadline``, ``pending`` (another fired
/// first), ``exited`` (a log pattern can no longer appear), ``error`` or
/// ``unsupported`` (the executor cannot watch it).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ConditionOutcome {
    pub condition: ConditionOutcomeCondition,
    #[serde(default)]
    pub fired: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub observed: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<ConditionOutcomeReason>,
}

impl ConditionOutcome {
    pub fn builder() -> ConditionOutcomeBuilder {
        <ConditionOutcomeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConditionOutcomeBuilder {
    condition: Option<ConditionOutcomeCondition>,
    fired: Option<bool>,
    observed: Option<HashMap<String, serde_json::Value>>,
    reason: Option<ConditionOutcomeReason>,
}

impl ConditionOutcomeBuilder {
    pub fn condition(mut self, value: ConditionOutcomeCondition) -> Self {
        self.condition = Some(value);
        self
    }

    pub fn fired(mut self, value: bool) -> Self {
        self.fired = Some(value);
        self
    }

    pub fn observed(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.observed = Some(value);
        self
    }

    pub fn reason(mut self, value: ConditionOutcomeReason) -> Self {
        self.reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConditionOutcome`].
    /// This method will fail if any of the following fields are not set:
    /// - [`condition`](ConditionOutcomeBuilder::condition)
    /// - [`fired`](ConditionOutcomeBuilder::fired)
    pub fn build(self) -> Result<ConditionOutcome, BuildError> {
        Ok(ConditionOutcome {
            condition: self.condition.ok_or_else(|| BuildError::missing_field("condition"))?,
            fired: self.fired.ok_or_else(|| BuildError::missing_field("fired"))?,
            observed: self.observed,
            reason: self.reason,
        })
    }
}
