pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartOAuth {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resume_id: Option<String>,
}

impl StartOAuth {
    pub fn builder() -> StartOAuthBuilder {
        <StartOAuthBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartOAuthBuilder {
    installation_id: Option<String>,
    resume_id: Option<String>,
}

impl StartOAuthBuilder {
    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    pub fn resume_id(mut self, value: impl Into<String>) -> Self {
        self.resume_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartOAuth`].
    pub fn build(self) -> Result<StartOAuth, BuildError> {
        Ok(StartOAuth {
            installation_id: self.installation_id,
            resume_id: self.resume_id,
        })
    }
}

