pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CheckAgent {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: String,
}

impl CheckAgent {
    pub fn builder() -> CheckAgentBuilder {
        <CheckAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckAgentBuilder {
    name: Option<String>,
    slug: Option<String>,
    version_id: Option<String>,
}

impl CheckAgentBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn version_id(mut self, value: impl Into<String>) -> Self {
        self.version_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CheckAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CheckAgentBuilder::name)
    /// - [`slug`](CheckAgentBuilder::slug)
    /// - [`version_id`](CheckAgentBuilder::version_id)
    pub fn build(self) -> Result<CheckAgent, BuildError> {
        Ok(CheckAgent {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            version_id: self.version_id.ok_or_else(|| BuildError::missing_field("version_id"))?,
        })
    }
}
