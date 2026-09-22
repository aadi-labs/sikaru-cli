pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JobInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<JobInputEnvironment>,
    #[serde(default)]
    pub evaluator: String,
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    #[serde(default)]
    pub revision: String,
    #[serde(default)]
    pub rubric: String,
    #[serde(default)]
    pub targets: Vec<TargetInput>,
}

impl JobInput {
    pub fn builder() -> JobInputBuilder {
        <JobInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JobInputBuilder {
    environment: Option<JobInputEnvironment>,
    evaluator: Option<String>,
    request_id: Option<String>,
    revision: Option<String>,
    rubric: Option<String>,
    targets: Option<Vec<TargetInput>>,
}

impl JobInputBuilder {
    pub fn environment(mut self, value: JobInputEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn evaluator(mut self, value: impl Into<String>) -> Self {
        self.evaluator = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
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

    pub fn targets(mut self, value: Vec<TargetInput>) -> Self {
        self.targets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`JobInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evaluator`](JobInputBuilder::evaluator)
    /// - [`request_id`](JobInputBuilder::request_id)
    /// - [`revision`](JobInputBuilder::revision)
    /// - [`rubric`](JobInputBuilder::rubric)
    /// - [`targets`](JobInputBuilder::targets)
    pub fn build(self) -> Result<JobInput, BuildError> {
        Ok(JobInput {
            environment: self.environment,
            evaluator: self.evaluator.ok_or_else(|| BuildError::missing_field("evaluator"))?,
            request_id: self.request_id.ok_or_else(|| BuildError::missing_field("request_id"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            rubric: self.rubric.ok_or_else(|| BuildError::missing_field("rubric"))?,
            targets: self.targets.ok_or_else(|| BuildError::missing_field("targets"))?,
        })
    }
}

