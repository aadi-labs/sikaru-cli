pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetCheckFailure {
    #[serde(rename = "checkId")]
    #[serde(default)]
    pub check_id: String,
    #[serde(rename = "checkName")]
    #[serde(default)]
    pub check_name: String,
    #[serde(rename = "exampleId")]
    #[serde(default)]
    pub example_id: String,
    #[serde(default)]
    pub reason: String,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
}

impl DatasetCheckFailure {
    pub fn builder() -> DatasetCheckFailureBuilder {
        <DatasetCheckFailureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetCheckFailureBuilder {
    check_id: Option<String>,
    check_name: Option<String>,
    example_id: Option<String>,
    reason: Option<String>,
    run_id: Option<String>,
}

impl DatasetCheckFailureBuilder {
    pub fn check_id(mut self, value: impl Into<String>) -> Self {
        self.check_id = Some(value.into());
        self
    }

    pub fn check_name(mut self, value: impl Into<String>) -> Self {
        self.check_name = Some(value.into());
        self
    }

    pub fn example_id(mut self, value: impl Into<String>) -> Self {
        self.example_id = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DatasetCheckFailure`].
    /// This method will fail if any of the following fields are not set:
    /// - [`check_id`](DatasetCheckFailureBuilder::check_id)
    /// - [`check_name`](DatasetCheckFailureBuilder::check_name)
    /// - [`example_id`](DatasetCheckFailureBuilder::example_id)
    /// - [`reason`](DatasetCheckFailureBuilder::reason)
    /// - [`run_id`](DatasetCheckFailureBuilder::run_id)
    pub fn build(self) -> Result<DatasetCheckFailure, BuildError> {
        Ok(DatasetCheckFailure {
            check_id: self.check_id.ok_or_else(|| BuildError::missing_field("check_id"))?,
            check_name: self.check_name.ok_or_else(|| BuildError::missing_field("check_name"))?,
            example_id: self.example_id.ok_or_else(|| BuildError::missing_field("example_id"))?,
            reason: self.reason.ok_or_else(|| BuildError::missing_field("reason"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
        })
    }
}
