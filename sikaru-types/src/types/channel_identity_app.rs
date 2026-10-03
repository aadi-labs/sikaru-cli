pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelIdentityApp {
    #[serde(default)]
    pub audience: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_callback_url: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub issuer: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channel_url: Option<String>,
    #[serde(default)]
    pub status: String,
}

impl ChannelIdentityApp {
    pub fn builder() -> ChannelIdentityAppBuilder {
        <ChannelIdentityAppBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelIdentityAppBuilder {
    audience: Option<String>,
    connection_callback_url: Option<String>,
    id: Option<String>,
    issuer: Option<String>,
    private_channel_url: Option<String>,
    status: Option<String>,
}

impl ChannelIdentityAppBuilder {
    pub fn audience(mut self, value: impl Into<String>) -> Self {
        self.audience = Some(value.into());
        self
    }

    pub fn connection_callback_url(mut self, value: impl Into<String>) -> Self {
        self.connection_callback_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn issuer(mut self, value: impl Into<String>) -> Self {
        self.issuer = Some(value.into());
        self
    }

    pub fn private_channel_url(mut self, value: impl Into<String>) -> Self {
        self.private_channel_url = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelIdentityApp`].
    /// This method will fail if any of the following fields are not set:
    /// - [`audience`](ChannelIdentityAppBuilder::audience)
    /// - [`id`](ChannelIdentityAppBuilder::id)
    /// - [`issuer`](ChannelIdentityAppBuilder::issuer)
    /// - [`status`](ChannelIdentityAppBuilder::status)
    pub fn build(self) -> Result<ChannelIdentityApp, BuildError> {
        Ok(ChannelIdentityApp {
            audience: self.audience.ok_or_else(|| BuildError::missing_field("audience"))?,
            connection_callback_url: self.connection_callback_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            issuer: self.issuer.ok_or_else(|| BuildError::missing_field("issuer"))?,
            private_channel_url: self.private_channel_url,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
