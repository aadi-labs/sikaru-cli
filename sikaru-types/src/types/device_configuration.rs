pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceConfiguration {
    #[serde(rename = "clientId")]
    #[serde(default)]
    pub client_id: String,
}

impl DeviceConfiguration {
    pub fn builder() -> DeviceConfigurationBuilder {
        <DeviceConfigurationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceConfigurationBuilder {
    client_id: Option<String>,
}

impl DeviceConfigurationBuilder {
    pub fn client_id(mut self, value: impl Into<String>) -> Self {
        self.client_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeviceConfiguration`].
    /// This method will fail if any of the following fields are not set:
    /// - [`client_id`](DeviceConfigurationBuilder::client_id)
    pub fn build(self) -> Result<DeviceConfiguration, BuildError> {
        Ok(DeviceConfiguration {
            client_id: self.client_id.ok_or_else(|| BuildError::missing_field("client_id"))?,
        })
    }
}
