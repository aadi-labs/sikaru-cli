pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct EvaluationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjudicates: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub context: Option<JudgmentContext>,
    pub environment: EvaluationResultEnvironment,
    #[serde(default)]
    pub evaluator: String,
    #[serde(default)]
    pub id: String,
    #[serde(rename = "publicReason")]
    #[serde(default)]
    pub public_reason: String,
    #[serde(default)]
    pub revision: String,
    pub source: EvaluationResultSource,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<JudgmentTarget>,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
    pub verdict: EvaluationResultVerdict,
}

impl EvaluationResult {
    pub fn builder() -> EvaluationResultBuilder {
        <EvaluationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EvaluationResultBuilder {
    adjudicates: Option<Vec<String>>,
    context: Option<JudgmentContext>,
    environment: Option<EvaluationResultEnvironment>,
    evaluator: Option<String>,
    id: Option<String>,
    public_reason: Option<String>,
    revision: Option<String>,
    source: Option<EvaluationResultSource>,
    target: Option<JudgmentTarget>,
    trace_id: Option<String>,
    verdict: Option<EvaluationResultVerdict>,
}

impl EvaluationResultBuilder {
    pub fn adjudicates(mut self, value: Vec<String>) -> Self {
        self.adjudicates = Some(value);
        self
    }

    pub fn context(mut self, value: JudgmentContext) -> Self {
        self.context = Some(value);
        self
    }

    pub fn environment(mut self, value: EvaluationResultEnvironment) -> Self {
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

    pub fn public_reason(mut self, value: impl Into<String>) -> Self {
        self.public_reason = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn source(mut self, value: EvaluationResultSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn target(mut self, value: JudgmentTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn verdict(mut self, value: EvaluationResultVerdict) -> Self {
        self.verdict = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EvaluationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](EvaluationResultBuilder::environment)
    /// - [`evaluator`](EvaluationResultBuilder::evaluator)
    /// - [`id`](EvaluationResultBuilder::id)
    /// - [`public_reason`](EvaluationResultBuilder::public_reason)
    /// - [`revision`](EvaluationResultBuilder::revision)
    /// - [`source`](EvaluationResultBuilder::source)
    /// - [`trace_id`](EvaluationResultBuilder::trace_id)
    /// - [`verdict`](EvaluationResultBuilder::verdict)
    pub fn build(self) -> Result<EvaluationResult, BuildError> {
        Ok(EvaluationResult {
            adjudicates: self.adjudicates,
            context: self.context,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            evaluator: self.evaluator.ok_or_else(|| BuildError::missing_field("evaluator"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            public_reason: self.public_reason.ok_or_else(|| BuildError::missing_field("public_reason"))?,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            target: self.target,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
            verdict: self.verdict.ok_or_else(|| BuildError::missing_field("verdict"))?,
        })
    }
}

