pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OAuthResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installation: Option<Installation>,
    #[serde(default)]
    pub return_path: String,
    pub status: OAuthResultStatus,
}

impl OAuthResult {
    pub fn builder() -> OAuthResultBuilder {
        <OAuthResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OAuthResultBuilder {
    installation: Option<Installation>,
    return_path: Option<String>,
    status: Option<OAuthResultStatus>,
}

impl OAuthResultBuilder {
    pub fn installation(mut self, value: Installation) -> Self {
        self.installation = Some(value);
        self
    }

    pub fn return_path(mut self, value: impl Into<String>) -> Self {
        self.return_path = Some(value.into());
        self
    }

    pub fn status(mut self, value: OAuthResultStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OAuthResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`return_path`](OAuthResultBuilder::return_path)
    /// - [`status`](OAuthResultBuilder::status)
    pub fn build(self) -> Result<OAuthResult, BuildError> {
        Ok(OAuthResult {
            installation: self.installation,
            return_path: self.return_path.ok_or_else(|| BuildError::missing_field("return_path"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
