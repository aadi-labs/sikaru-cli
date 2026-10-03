pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ChannelIdentityAppId {
    #[serde(default)]
    pub id: String,
}

impl ChannelIdentityAppId {
    pub fn builder() -> ChannelIdentityAppIdBuilder {
        <ChannelIdentityAppIdBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChannelIdentityAppIdBuilder {
    id: Option<String>,
}

impl ChannelIdentityAppIdBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChannelIdentityAppId`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChannelIdentityAppIdBuilder::id)
    pub fn build(self) -> Result<ChannelIdentityAppId, BuildError> {
        Ok(ChannelIdentityAppId {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
        })
    }
}
