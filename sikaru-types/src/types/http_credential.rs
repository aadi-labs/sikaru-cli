pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct HttpCredential {
    #[serde(default)]
    pub credential: String,
}

impl HttpCredential {
    pub fn builder() -> HttpCredentialBuilder {
        <HttpCredentialBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct HttpCredentialBuilder {
    credential: Option<String>,
}

impl HttpCredentialBuilder {
    pub fn credential(mut self, value: impl Into<String>) -> Self {
        self.credential = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`HttpCredential`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credential`](HttpCredentialBuilder::credential)
    pub fn build(self) -> Result<HttpCredential, BuildError> {
        Ok(HttpCredential {
            credential: self.credential.ok_or_else(|| BuildError::missing_field("credential"))?,
        })
    }
}
