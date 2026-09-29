pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for stream_transcript_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StreamTranscriptEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<i64>,
}

impl StreamTranscriptEventsQueryRequest {
    pub fn builder() -> StreamTranscriptEventsQueryRequestBuilder {
        <StreamTranscriptEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StreamTranscriptEventsQueryRequestBuilder {
    after: Option<i64>,
}

impl StreamTranscriptEventsQueryRequestBuilder {
    pub fn after(mut self, value: i64) -> Self {
        self.after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StreamTranscriptEventsQueryRequest`].
    pub fn build(self) -> Result<StreamTranscriptEventsQueryRequest, BuildError> {
        Ok(StreamTranscriptEventsQueryRequest {
            after: self.after,
        })
    }
}

