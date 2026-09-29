pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionAgentUsage {
    #[serde(default)]
    pub draft: bool,
    #[serde(default)]
    pub draft_valid: bool,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub live: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub slug: String,
}

impl ConnectionAgentUsage {
    pub fn builder() -> ConnectionAgentUsageBuilder {
        <ConnectionAgentUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionAgentUsageBuilder {
    draft: Option<bool>,
    draft_valid: Option<bool>,
    id: Option<String>,
    live: Option<bool>,
    name: Option<String>,
    slug: Option<String>,
}

impl ConnectionAgentUsageBuilder {
    pub fn draft(mut self, value: bool) -> Self {
        self.draft = Some(value);
        self
    }

    pub fn draft_valid(mut self, value: bool) -> Self {
        self.draft_valid = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn live(mut self, value: bool) -> Self {
        self.live = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionAgentUsage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`draft`](ConnectionAgentUsageBuilder::draft)
    /// - [`draft_valid`](ConnectionAgentUsageBuilder::draft_valid)
    /// - [`id`](ConnectionAgentUsageBuilder::id)
    /// - [`live`](ConnectionAgentUsageBuilder::live)
    /// - [`name`](ConnectionAgentUsageBuilder::name)
    /// - [`slug`](ConnectionAgentUsageBuilder::slug)
    pub fn build(self) -> Result<ConnectionAgentUsage, BuildError> {
        Ok(ConnectionAgentUsage {
            draft: self.draft.ok_or_else(|| BuildError::missing_field("draft"))?,
            draft_valid: self.draft_valid.ok_or_else(|| BuildError::missing_field("draft_valid"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            live: self.live.ok_or_else(|| BuildError::missing_field("live"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
        })
    }
}
