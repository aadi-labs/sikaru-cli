pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ToolSkillSourceRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    #[serde(default)]
    pub kind: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

impl ToolSkillSourceRequest {
    pub fn builder() -> ToolSkillSourceRequestBuilder {
        <ToolSkillSourceRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ToolSkillSourceRequestBuilder {
    content: Option<String>,
    kind: Option<String>,
    r#ref: Option<String>,
}

impl ToolSkillSourceRequestBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn r#ref(mut self, value: impl Into<String>) -> Self {
        self.r#ref = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ToolSkillSourceRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`kind`](ToolSkillSourceRequestBuilder::kind)
    pub fn build(self) -> Result<ToolSkillSourceRequest, BuildError> {
        Ok(ToolSkillSourceRequest {
            content: self.content,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            r#ref: self.r#ref,
        })
    }
}
