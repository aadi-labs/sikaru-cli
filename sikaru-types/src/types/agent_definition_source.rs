pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AgentDefinitionSource {
    #[serde(default)]
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub digest: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<AgentDefinitionSourceEncoding>,
    pub kind: AgentDefinitionSourceKind,
    #[serde(default)]
    pub path: String,
}

impl AgentDefinitionSource {
    pub fn builder() -> AgentDefinitionSourceBuilder {
        <AgentDefinitionSourceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDefinitionSourceBuilder {
    content: Option<String>,
    digest: Option<String>,
    encoding: Option<AgentDefinitionSourceEncoding>,
    kind: Option<AgentDefinitionSourceKind>,
    path: Option<String>,
}

impl AgentDefinitionSourceBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn digest(mut self, value: impl Into<String>) -> Self {
        self.digest = Some(value.into());
        self
    }

    pub fn encoding(mut self, value: AgentDefinitionSourceEncoding) -> Self {
        self.encoding = Some(value);
        self
    }

    pub fn kind(mut self, value: AgentDefinitionSourceKind) -> Self {
        self.kind = Some(value);
        self
    }

    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentDefinitionSource`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](AgentDefinitionSourceBuilder::content)
    /// - [`kind`](AgentDefinitionSourceBuilder::kind)
    /// - [`path`](AgentDefinitionSourceBuilder::path)
    pub fn build(self) -> Result<AgentDefinitionSource, BuildError> {
        Ok(AgentDefinitionSource {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            digest: self.digest,
            encoding: self.encoding,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
        })
    }
}
