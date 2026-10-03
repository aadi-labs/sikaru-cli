pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonalChannelConnection {
    #[serde(default)]
    pub account_status: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
}

impl PersonalChannelConnection {
    pub fn builder() -> PersonalChannelConnectionBuilder {
        <PersonalChannelConnectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalChannelConnectionBuilder {
    account_status: Option<String>,
    display_name: Option<String>,
    id: Option<String>,
    kind: Option<String>,
}

impl PersonalChannelConnectionBuilder {
    pub fn account_status(mut self, value: impl Into<String>) -> Self {
        self.account_status = Some(value.into());
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonalChannelConnection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_status`](PersonalChannelConnectionBuilder::account_status)
    /// - [`display_name`](PersonalChannelConnectionBuilder::display_name)
    /// - [`id`](PersonalChannelConnectionBuilder::id)
    /// - [`kind`](PersonalChannelConnectionBuilder::kind)
    pub fn build(self) -> Result<PersonalChannelConnection, BuildError> {
        Ok(PersonalChannelConnection {
            account_status: self.account_status.ok_or_else(|| BuildError::missing_field("account_status"))?,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
