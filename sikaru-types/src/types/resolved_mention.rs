pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResolvedMention {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_hosts: Option<Vec<String>>,
    #[serde(default)]
    pub mention: Mention,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_id: Option<String>,
    #[serde(default)]
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_digests: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<String>,
}

impl ResolvedMention {
    pub fn builder() -> ResolvedMentionBuilder {
        <ResolvedMentionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolvedMentionBuilder {
    allowed_hosts: Option<Vec<String>>,
    mention: Option<Mention>,
    reason: Option<String>,
    resource_id: Option<String>,
    state: Option<String>,
    tool_digests: Option<HashMap<String, String>>,
    warning: Option<String>,
}

impl ResolvedMentionBuilder {
    pub fn allowed_hosts(mut self, value: Vec<String>) -> Self {
        self.allowed_hosts = Some(value);
        self
    }

    pub fn mention(mut self, value: Mention) -> Self {
        self.mention = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn resource_id(mut self, value: impl Into<String>) -> Self {
        self.resource_id = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn tool_digests(mut self, value: HashMap<String, String>) -> Self {
        self.tool_digests = Some(value);
        self
    }

    pub fn warning(mut self, value: impl Into<String>) -> Self {
        self.warning = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResolvedMention`].
    /// This method will fail if any of the following fields are not set:
    /// - [`mention`](ResolvedMentionBuilder::mention)
    /// - [`state`](ResolvedMentionBuilder::state)
    pub fn build(self) -> Result<ResolvedMention, BuildError> {
        Ok(ResolvedMention {
            allowed_hosts: self.allowed_hosts,
            mention: self.mention.ok_or_else(|| BuildError::missing_field("mention"))?,
            reason: self.reason,
            resource_id: self.resource_id,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
            tool_digests: self.tool_digests,
            warning: self.warning,
        })
    }
}
