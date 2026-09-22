pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaseInput {
    #[serde(default)]
    pub baseline: AnswerRef,
    #[serde(default)]
    pub candidate: AnswerRef,
    #[serde(rename = "caseId")]
    #[serde(default)]
    pub case_id: String,
}

impl CaseInput {
    pub fn builder() -> CaseInputBuilder {
        <CaseInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaseInputBuilder {
    baseline: Option<AnswerRef>,
    candidate: Option<AnswerRef>,
    case_id: Option<String>,
}

impl CaseInputBuilder {
    pub fn baseline(mut self, value: AnswerRef) -> Self {
        self.baseline = Some(value);
        self
    }

    pub fn candidate(mut self, value: AnswerRef) -> Self {
        self.candidate = Some(value);
        self
    }

    pub fn case_id(mut self, value: impl Into<String>) -> Self {
        self.case_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CaseInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`baseline`](CaseInputBuilder::baseline)
    /// - [`candidate`](CaseInputBuilder::candidate)
    /// - [`case_id`](CaseInputBuilder::case_id)
    pub fn build(self) -> Result<CaseInput, BuildError> {
        Ok(CaseInput {
            baseline: self.baseline.ok_or_else(|| BuildError::missing_field("baseline"))?,
            candidate: self.candidate.ok_or_else(|| BuildError::missing_field("candidate"))?,
            case_id: self.case_id.ok_or_else(|| BuildError::missing_field("case_id"))?,
        })
    }
}
