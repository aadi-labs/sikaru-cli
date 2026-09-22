pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct JudgmentContext {
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub sha256: String,
}

impl JudgmentContext {
    pub fn builder() -> JudgmentContextBuilder {
        <JudgmentContextBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct JudgmentContextBuilder {
    content: Option<String>,
    sha256: Option<String>,
}

impl JudgmentContextBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    pub fn sha256(mut self, value: impl Into<String>) -> Self {
        self.sha256 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`JudgmentContext`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](JudgmentContextBuilder::content)
    /// - [`sha256`](JudgmentContextBuilder::sha256)
    pub fn build(self) -> Result<JudgmentContext, BuildError> {
        Ok(JudgmentContext {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
            sha256: self.sha256.ok_or_else(|| BuildError::missing_field("sha256"))?,
        })
    }
}
