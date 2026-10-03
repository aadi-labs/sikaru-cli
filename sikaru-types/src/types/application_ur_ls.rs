pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationUrLs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_channel_url: Option<String>,
}

impl ApplicationUrLs {
    pub fn builder() -> ApplicationUrLsBuilder {
        <ApplicationUrLsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationUrLsBuilder {
    connection_callback_url: Option<String>,
    private_channel_url: Option<String>,
}

impl ApplicationUrLsBuilder {
    pub fn connection_callback_url(mut self, value: impl Into<String>) -> Self {
        self.connection_callback_url = Some(value.into());
        self
    }

    pub fn private_channel_url(mut self, value: impl Into<String>) -> Self {
        self.private_channel_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplicationUrLs`].
    pub fn build(self) -> Result<ApplicationUrLs, BuildError> {
        Ok(ApplicationUrLs {
            connection_callback_url: self.connection_callback_url,
            private_channel_url: self.private_channel_url,
        })
    }
}

