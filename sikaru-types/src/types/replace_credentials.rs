pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReplaceCredentials {
    #[serde(default)]
    pub credentials: ConnectionCredentials,
}

impl ReplaceCredentials {
    pub fn builder() -> ReplaceCredentialsBuilder {
        <ReplaceCredentialsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReplaceCredentialsBuilder {
    credentials: Option<ConnectionCredentials>,
}

impl ReplaceCredentialsBuilder {
    pub fn credentials(mut self, value: ConnectionCredentials) -> Self {
        self.credentials = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReplaceCredentials`].
    /// This method will fail if any of the following fields are not set:
    /// - [`credentials`](ReplaceCredentialsBuilder::credentials)
    pub fn build(self) -> Result<ReplaceCredentials, BuildError> {
        Ok(ReplaceCredentials {
            credentials: self.credentials.ok_or_else(|| BuildError::missing_field("credentials"))?,
        })
    }
}

