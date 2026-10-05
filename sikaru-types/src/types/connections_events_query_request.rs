pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConnectionsEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ConnectionsEventsQueryRequest {
    pub fn builder() -> ConnectionsEventsQueryRequestBuilder {
        <ConnectionsEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionsEventsQueryRequestBuilder {
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ConnectionsEventsQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConnectionsEventsQueryRequest`].
    pub fn build(self) -> Result<ConnectionsEventsQueryRequest, BuildError> {
        Ok(ConnectionsEventsQueryRequest {
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

