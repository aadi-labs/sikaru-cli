pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AbandonInput {
    #[serde(default)]
    pub evidence: String,
}

impl AbandonInput {
    pub fn builder() -> AbandonInputBuilder {
        <AbandonInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AbandonInputBuilder {
    evidence: Option<String>,
}

impl AbandonInputBuilder {
    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AbandonInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`evidence`](AbandonInputBuilder::evidence)
    pub fn build(self) -> Result<AbandonInput, BuildError> {
        Ok(AbandonInput {
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
        })
    }
}

