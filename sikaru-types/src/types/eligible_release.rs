pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EligibleRelease {
    #[serde(rename = "generationId")]
    #[serde(default)]
    pub generation_id: String,
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
}

impl EligibleRelease {
    pub fn builder() -> EligibleReleaseBuilder {
        <EligibleReleaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EligibleReleaseBuilder {
    generation_id: Option<String>,
    harness_version_id: Option<String>,
}

impl EligibleReleaseBuilder {
    pub fn generation_id(mut self, value: impl Into<String>) -> Self {
        self.generation_id = Some(value.into());
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EligibleRelease`].
    /// This method will fail if any of the following fields are not set:
    /// - [`generation_id`](EligibleReleaseBuilder::generation_id)
    /// - [`harness_version_id`](EligibleReleaseBuilder::harness_version_id)
    pub fn build(self) -> Result<EligibleRelease, BuildError> {
        Ok(EligibleRelease {
            generation_id: self.generation_id.ok_or_else(|| BuildError::missing_field("generation_id"))?,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
        })
    }
}
