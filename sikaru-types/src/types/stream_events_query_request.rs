pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for stream_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl StreamEventsQueryRequest {
    pub fn builder() -> StreamEventsQueryRequestBuilder {
        <StreamEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamEventsQueryRequestBuilder {
    after: Option<String>,
}

impl StreamEventsQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StreamEventsQueryRequest`].
    pub fn build(self) -> Result<StreamEventsQueryRequest, BuildError> {
        Ok(StreamEventsQueryRequest {
            after: self.after,
        })
    }
}

