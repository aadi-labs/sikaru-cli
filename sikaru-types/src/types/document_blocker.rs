pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentBlocker {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mention: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl DocumentBlocker {
    pub fn builder() -> DocumentBlockerBuilder {
        <DocumentBlockerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentBlockerBuilder {
    mention: Option<String>,
    reason: Option<String>,
    state: Option<String>,
}

impl DocumentBlockerBuilder {
    pub fn mention(mut self, value: impl Into<String>) -> Self {
        self.mention = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentBlocker`].
    pub fn build(self) -> Result<DocumentBlocker, BuildError> {
        Ok(DocumentBlocker {
            mention: self.mention,
            reason: self.reason,
            state: self.state,
        })
    }
}
