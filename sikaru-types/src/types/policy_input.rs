pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PolicyInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<PolicyInputEnvironment>,
    #[serde(default)]
    pub evaluator: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub revision: String,
    #[serde(default)]
    pub rubric: String,
    #[serde(rename = "samplePercent")]
    #[serde(default)]
    pub sample_percent: i64,
}

impl PolicyInput {
    pub fn builder() -> PolicyInputBuilder {
        <PolicyInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PolicyInputBuilder {
    environment: Option<PolicyInputEnvironment>,
    evaluator: Option<String>,
    id: Option<String>,
    revision: Option<String>,
    rubric: Option<String>,
    sample_percent: Option<i64>,
}

impl PolicyInputBuilder {
    pub fn environment(mut self, value: PolicyInputEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn evaluator(mut self, value: impl Into<String>) -> Self {
        self.evaluator = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn rubric(mut self, value: impl Into<String>) -> Self {
        self.rubric = Some(value.into());
        self
    }

    pub fn sample_percent(mut self, value: i64) -> Self {
        self.sample_percent = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PolicyInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluator`](PolicyInputBuilder::evaluator)
    /// - [`id`](PolicyInputBuilder::id)
    /// - [`revision`](PolicyInputBuilder::revision)
    /// - [`rubric`](PolicyInputBuilder::rubric)
    /// - [`sample_percent`](PolicyInputBuilder::sample_percent)
    pub fn build(self) -> Result<PolicyInput, BuildError> {
        Ok(PolicyInput {
            environment: self.environment,
            evaluator: self.evaluator.ok_or_else(|| BuildError::missing_field("evaluator"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            rubric: self.rubric.ok_or_else(|| BuildError::missing_field("rubric"))?,
            sample_percent: self.sample_percent.ok_or_else(|| BuildError::missing_field("sample_percent"))?,
        })
    }
}

