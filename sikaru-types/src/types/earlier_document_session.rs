pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EarlierDocumentSession {
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(rename = "sessionId")]
    #[serde(default)]
    pub session_id: String,
}

impl EarlierDocumentSession {
    pub fn builder() -> EarlierDocumentSessionBuilder {
        <EarlierDocumentSessionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EarlierDocumentSessionBuilder {
    harness_version_id: Option<String>,
    session_id: Option<String>,
}

impl EarlierDocumentSessionBuilder {
    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EarlierDocumentSession`].
    /// This method will fail if any of the following fields are not set:
    /// - [`harness_version_id`](EarlierDocumentSessionBuilder::harness_version_id)
    /// - [`session_id`](EarlierDocumentSessionBuilder::session_id)
    pub fn build(self) -> Result<EarlierDocumentSession, BuildError> {
        Ok(EarlierDocumentSession {
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}
