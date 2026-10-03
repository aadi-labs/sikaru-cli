pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CompleteOAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(default)]
    pub state: String,
}

impl CompleteOAuth {
    pub fn builder() -> CompleteOAuthBuilder {
        <CompleteOAuthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteOAuthBuilder {
    code: Option<String>,
    error: Option<String>,
    state: Option<String>,
}

impl CompleteOAuthBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn error(mut self, value: impl Into<String>) -> Self {
        self.error = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteOAuth`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](CompleteOAuthBuilder::state)
    pub fn build(self) -> Result<CompleteOAuth, BuildError> {
        Ok(CompleteOAuth {
            code: self.code,
            error: self.error,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}

