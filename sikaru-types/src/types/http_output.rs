pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpOutput {
    #[serde(default)]
    pub content: String,
}

impl HttpOutput {
    pub fn builder() -> HttpOutputBuilder {
        <HttpOutputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpOutputBuilder {
    content: Option<String>,
}

impl HttpOutputBuilder {
    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HttpOutput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](HttpOutputBuilder::content)
    pub fn build(self) -> Result<HttpOutput, BuildError> {
        Ok(HttpOutput {
            content: self.content.ok_or_else(|| BuildError::missing_field("content"))?,
        })
    }
}
