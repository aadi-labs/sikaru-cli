pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SlackLink {
    #[serde(default)]
    pub installation_id: String,
}

impl SlackLink {
    pub fn builder() -> SlackLinkBuilder {
        <SlackLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SlackLinkBuilder {
    installation_id: Option<String>,
}

impl SlackLinkBuilder {
    pub fn installation_id(mut self, value: impl Into<String>) -> Self {
        self.installation_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SlackLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`installation_id`](SlackLinkBuilder::installation_id)
    pub fn build(self) -> Result<SlackLink, BuildError> {
        Ok(SlackLink {
            installation_id: self.installation_id.ok_or_else(|| BuildError::missing_field("installation_id"))?,
        })
    }
}
