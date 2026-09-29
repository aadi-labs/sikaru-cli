pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct MentionResource {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub broken_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<String>,
    #[serde(default)]
    pub resource_id: String,
    #[serde(default)]
    pub slug: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_digests: Option<HashMap<String, String>>,
}

impl MentionResource {
    pub fn builder() -> MentionResourceBuilder {
        <MentionResourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MentionResourceBuilder {
    aliases: Option<Vec<String>>,
    allowed_hosts: Option<Vec<String>>,
    blocked_reason: Option<String>,
    broken_reason: Option<String>,
    connected: Option<bool>,
    kind: Option<String>,
    ownership: Option<String>,
    resource_id: Option<String>,
    slug: Option<String>,
    tool_digests: Option<HashMap<String, String>>,
}

impl MentionResourceBuilder {
    pub fn aliases(mut self, value: Vec<String>) -> Self {
        self.aliases = Some(value);
        self
    }

    pub fn allowed_hosts(mut self, value: Vec<String>) -> Self {
        self.allowed_hosts = Some(value);
        self
    }

    pub fn blocked_reason(mut self, value: impl Into<String>) -> Self {
        self.blocked_reason = Some(value.into());
        self
    }

    pub fn broken_reason(mut self, value: impl Into<String>) -> Self {
        self.broken_reason = Some(value.into());
        self
    }

    pub fn connected(mut self, value: bool) -> Self {
        self.connected = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn ownership(mut self, value: impl Into<String>) -> Self {
        self.ownership = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn tool_digests(mut self, value: HashMap<String, String>) -> Self {
        self.tool_digests = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MentionResource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](MentionResourceBuilder::kind)
    /// - [`resource_id`](MentionResourceBuilder::resource_id)
    /// - [`slug`](MentionResourceBuilder::slug)
    pub fn build(self) -> Result<MentionResource, BuildError> {
        Ok(MentionResource {
            aliases: self.aliases,
            allowed_hosts: self.allowed_hosts,
            blocked_reason: self.blocked_reason,
            broken_reason: self.broken_reason,
            connected: self.connected,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            ownership: self.ownership,
            resource_id: self.resource_id.ok_or_else(|| BuildError::missing_field("resource_id"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            tool_digests: self.tool_digests,
        })
    }
}
