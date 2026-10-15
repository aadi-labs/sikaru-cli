pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EligibleReleases {
    #[serde(default)]
    pub releases: Vec<EligibleRelease>,
}

impl EligibleReleases {
    pub fn builder() -> EligibleReleasesBuilder {
        <EligibleReleasesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EligibleReleasesBuilder {
    releases: Option<Vec<EligibleRelease>>,
}

impl EligibleReleasesBuilder {
    pub fn releases(mut self, value: Vec<EligibleRelease>) -> Self {
        self.releases = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EligibleReleases`].
    /// This method will fail if any of the following fields are not set:
    /// - [`releases`](EligibleReleasesBuilder::releases)
    pub fn build(self) -> Result<EligibleReleases, BuildError> {
        Ok(EligibleReleases {
            releases: self.releases.ok_or_else(|| BuildError::missing_field("releases"))?,
        })
    }
}
