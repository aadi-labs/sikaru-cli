pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EventDeliveryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callback_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
}

impl EventDeliveryRequest {
    pub fn builder() -> EventDeliveryRequestBuilder {
        <EventDeliveryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EventDeliveryRequestBuilder {
    callback_url: Option<String>,
    mode: Option<String>,
}

impl EventDeliveryRequestBuilder {
    pub fn callback_url(mut self, value: impl Into<String>) -> Self {
        self.callback_url = Some(value.into());
        self
    }

    pub fn mode(mut self, value: impl Into<String>) -> Self {
        self.mode = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EventDeliveryRequest`].
    pub fn build(self) -> Result<EventDeliveryRequest, BuildError> {
        Ok(EventDeliveryRequest {
            callback_url: self.callback_url,
            mode: self.mode,
        })
    }
}
