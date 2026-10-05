pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StartedCheck {
    #[serde(rename = "checkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_id: Option<String>,
    #[serde(rename = "exampleId")]
    #[serde(default)]
    pub example_id: String,
    pub outcome: StartedCheckOutcome,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(rename = "resultId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_id: Option<String>,
    #[serde(rename = "runId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_id: Option<String>,
}

impl StartedCheck {
    pub fn builder() -> StartedCheckBuilder {
        <StartedCheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartedCheckBuilder {
    check_id: Option<String>,
    example_id: Option<String>,
    outcome: Option<StartedCheckOutcome>,
    reason: Option<String>,
    result_id: Option<String>,
    run_id: Option<String>,
}

impl StartedCheckBuilder {
    pub fn check_id(mut self, value: impl Into<String>) -> Self {
        self.check_id = Some(value.into());
        self
    }

    pub fn example_id(mut self, value: impl Into<String>) -> Self {
        self.example_id = Some(value.into());
        self
    }

    pub fn outcome(mut self, value: StartedCheckOutcome) -> Self {
        self.outcome = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn result_id(mut self, value: impl Into<String>) -> Self {
        self.result_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartedCheck`].
    /// This method will fail if any of the following fields are not set:
    /// - [`example_id`](StartedCheckBuilder::example_id)
    /// - [`outcome`](StartedCheckBuilder::outcome)
    pub fn build(self) -> Result<StartedCheck, BuildError> {
        Ok(StartedCheck {
            check_id: self.check_id,
            example_id: self.example_id.ok_or_else(|| BuildError::missing_field("example_id"))?,
            outcome: self.outcome.ok_or_else(|| BuildError::missing_field("outcome"))?,
            reason: self.reason,
            result_id: self.result_id,
            run_id: self.run_id,
        })
    }
}
