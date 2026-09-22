pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CleanupInput {
    #[serde(default)]
    pub children_terminated: bool,
    #[serde(default)]
    pub evidence: String,
}

impl CleanupInput {
    pub fn builder() -> CleanupInputBuilder {
        <CleanupInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CleanupInputBuilder {
    children_terminated: Option<bool>,
    evidence: Option<String>,
}

impl CleanupInputBuilder {
    pub fn children_terminated(mut self, value: bool) -> Self {
        self.children_terminated = Some(value);
        self
    }

    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CleanupInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children_terminated`](CleanupInputBuilder::children_terminated)
    /// - [`evidence`](CleanupInputBuilder::evidence)
    pub fn build(self) -> Result<CleanupInput, BuildError> {
        Ok(CleanupInput {
            children_terminated: self.children_terminated.ok_or_else(|| BuildError::missing_field("children_terminated"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}

