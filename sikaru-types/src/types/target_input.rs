pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TargetInput {
    pub target: JudgmentTarget,
    #[serde(rename = "traceId")]
    #[serde(default)]
    pub trace_id: String,
}

impl TargetInput {
    pub fn builder() -> TargetInputBuilder {
        <TargetInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TargetInputBuilder {
    target: Option<JudgmentTarget>,
    trace_id: Option<String>,
}

impl TargetInputBuilder {
    pub fn target(mut self, value: JudgmentTarget) -> Self {
        self.target = Some(value);
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TargetInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`target`](TargetInputBuilder::target)
    /// - [`trace_id`](TargetInputBuilder::trace_id)
    pub fn build(self) -> Result<TargetInput, BuildError> {
        Ok(TargetInput {
            target: self.target.ok_or_else(|| BuildError::missing_field("target"))?,
            trace_id: self.trace_id.ok_or_else(|| BuildError::missing_field("trace_id"))?,
        })
    }
}
