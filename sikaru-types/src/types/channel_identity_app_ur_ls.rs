pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelIdentityAppUrLs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_callback_url: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channel_url: Option<String>,
}

impl ChannelIdentityAppUrLs {
    pub fn builder() -> ChannelIdentityAppUrLsBuilder {
        <ChannelIdentityAppUrLsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelIdentityAppUrLsBuilder {
    connection_callback_url: Option<String>,
    id: Option<String>,
    private_channel_url: Option<String>,
}

impl ChannelIdentityAppUrLsBuilder {
    pub fn connection_callback_url(mut self, value: impl Into<String>) -> Self {
        self.connection_callback_url = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn private_channel_url(mut self, value: impl Into<String>) -> Self {
        self.private_channel_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelIdentityAppUrLs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChannelIdentityAppUrLsBuilder::id)
    pub fn build(self) -> Result<ChannelIdentityAppUrLs, BuildError> {
        Ok(ChannelIdentityAppUrLs {
            connection_callback_url: self.connection_callback_url,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            private_channel_url: self.private_channel_url,
        })
    }
}
