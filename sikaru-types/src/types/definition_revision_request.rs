pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DefinitionRevisionRequest {
    #[serde(rename = "contentDigest")]
    #[serde(default)]
    pub content_digest: String,
    pub definition: AgentDefinition,
}

impl DefinitionRevisionRequest {
    pub fn builder() -> DefinitionRevisionRequestBuilder {
        <DefinitionRevisionRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DefinitionRevisionRequestBuilder {
    content_digest: Option<String>,
    definition: Option<AgentDefinition>,
}

impl DefinitionRevisionRequestBuilder {
    pub fn content_digest(mut self, value: impl Into<String>) -> Self {
        self.content_digest = Some(value.into());
        self
    }

    pub fn definition(mut self, value: AgentDefinition) -> Self {
        self.definition = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DefinitionRevisionRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content_digest`](DefinitionRevisionRequestBuilder::content_digest)
    /// - [`definition`](DefinitionRevisionRequestBuilder::definition)
    pub fn build(self) -> Result<DefinitionRevisionRequest, BuildError> {
        Ok(DefinitionRevisionRequest {
            content_digest: self.content_digest.ok_or_else(|| BuildError::missing_field("content_digest"))?,
            definition: self.definition.ok_or_else(|| BuildError::missing_field("definition"))?,
        })
    }
}

