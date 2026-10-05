pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RunsEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl RunsEventsQueryRequest {
    pub fn builder() -> RunsEventsQueryRequestBuilder {
        <RunsEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunsEventsQueryRequestBuilder {
    after: Option<String>,
    limit: Option<String>,
}

impl RunsEventsQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RunsEventsQueryRequest`].
    pub fn build(self) -> Result<RunsEventsQueryRequest, BuildError> {
        Ok(RunsEventsQueryRequest {
            after: self.after,
            limit: self.limit,
        })
    }
}

