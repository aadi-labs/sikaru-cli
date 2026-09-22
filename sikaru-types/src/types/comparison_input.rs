pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ComparisonInput {
    #[serde(default)]
    pub cases: Vec<CaseInput>,
    #[serde(default)]
    pub evaluator: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub revision: String,
    #[serde(default)]
    pub rubric: String,
}

impl ComparisonInput {
    pub fn builder() -> ComparisonInputBuilder {
        <ComparisonInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ComparisonInputBuilder {
    cases: Option<Vec<CaseInput>>,
    evaluator: Option<String>,
    id: Option<String>,
    revision: Option<String>,
    rubric: Option<String>,
}

impl ComparisonInputBuilder {
    pub fn cases(mut self, value: Vec<CaseInput>) -> Self {
        self.cases = Some(value);
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

    /// Consumes the builder and constructs a [`ComparisonInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cases`](ComparisonInputBuilder::cases)
    /// - [`evaluator`](ComparisonInputBuilder::evaluator)
    /// - [`id`](ComparisonInputBuilder::id)
    /// - [`revision`](ComparisonInputBuilder::revision)
    /// - [`rubric`](ComparisonInputBuilder::rubric)
    pub fn build(self) -> Result<ComparisonInput, BuildError> {
        Ok(ComparisonInput {
            cases: self.cases.ok_or_else(|| BuildError::missing_field("cases"))?,
            evaluator: self.evaluator.ok_or_else(|| BuildError::missing_field("evaluator"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            rubric: self.rubric.ok_or_else(|| BuildError::missing_field("rubric"))?,
        })
    }
}

