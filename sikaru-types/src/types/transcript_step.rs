pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TranscriptStep {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub llm_call_count: Option<i64>,
    #[serde(default)]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<HashMap<String, i64>>,
    #[serde(default)]
    pub results: Vec<TranscriptToolResult>,
    pub source: TranscriptStepSource,
    #[serde(default)]
    pub step_id: i64,
    #[serde(default)]
    pub tool_calls: Vec<TranscriptToolCall>,
}

impl TranscriptStep {
    pub fn builder() -> TranscriptStepBuilder {
        <TranscriptStepBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptStepBuilder {
    llm_call_count: Option<i64>,
    message: Option<String>,
    metrics: Option<HashMap<String, i64>>,
    results: Option<Vec<TranscriptToolResult>>,
    source: Option<TranscriptStepSource>,
    step_id: Option<i64>,
    tool_calls: Option<Vec<TranscriptToolCall>>,
}

impl TranscriptStepBuilder {
    pub fn llm_call_count(mut self, value: i64) -> Self {
        self.llm_call_count = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn metrics(mut self, value: HashMap<String, i64>) -> Self {
        self.metrics = Some(value);
        self
    }

    pub fn results(mut self, value: Vec<TranscriptToolResult>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn source(mut self, value: TranscriptStepSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn step_id(mut self, value: i64) -> Self {
        self.step_id = Some(value);
        self
    }

    pub fn tool_calls(mut self, value: Vec<TranscriptToolCall>) -> Self {
        self.tool_calls = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptStep`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](TranscriptStepBuilder::message)
    /// - [`results`](TranscriptStepBuilder::results)
    /// - [`source`](TranscriptStepBuilder::source)
    /// - [`step_id`](TranscriptStepBuilder::step_id)
    /// - [`tool_calls`](TranscriptStepBuilder::tool_calls)
    pub fn build(self) -> Result<TranscriptStep, BuildError> {
        Ok(TranscriptStep {
            llm_call_count: self.llm_call_count,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
            metrics: self.metrics,
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            step_id: self.step_id.ok_or_else(|| BuildError::missing_field("step_id"))?,
            tool_calls: self.tool_calls.ok_or_else(|| BuildError::missing_field("tool_calls"))?,
        })
    }
}
