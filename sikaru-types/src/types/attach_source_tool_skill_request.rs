pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AttachSourceToolSkillRequest {
    #[serde(default)]
    pub capability_refs: Vec<String>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub source: ToolSkillSourceRequest,
}

impl AttachSourceToolSkillRequest {
    pub fn builder() -> AttachSourceToolSkillRequestBuilder {
        <AttachSourceToolSkillRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachSourceToolSkillRequestBuilder {
    capability_refs: Option<Vec<String>>,
    description: Option<String>,
    source: Option<ToolSkillSourceRequest>,
}

impl AttachSourceToolSkillRequestBuilder {
    pub fn capability_refs(mut self, value: Vec<String>) -> Self {
        self.capability_refs = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn source(mut self, value: ToolSkillSourceRequest) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachSourceToolSkillRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capability_refs`](AttachSourceToolSkillRequestBuilder::capability_refs)
    /// - [`description`](AttachSourceToolSkillRequestBuilder::description)
    /// - [`source`](AttachSourceToolSkillRequestBuilder::source)
    pub fn build(self) -> Result<AttachSourceToolSkillRequest, BuildError> {
        Ok(AttachSourceToolSkillRequest {
            capability_refs: self.capability_refs.ok_or_else(|| BuildError::missing_field("capability_refs"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
        })
    }
}

