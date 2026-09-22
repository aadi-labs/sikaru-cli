pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_managed_session_events
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListManagedSessionEventsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<String>,
}

impl ListManagedSessionEventsQueryRequest {
    pub fn builder() -> ListManagedSessionEventsQueryRequestBuilder {
        <ListManagedSessionEventsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListManagedSessionEventsQueryRequestBuilder {
    after: Option<String>,
    limit: Option<String>,
}

impl ListManagedSessionEventsQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn limit(mut self, value: impl Into<String>) -> Self {
        self.limit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListManagedSessionEventsQueryRequest`].
    pub fn build(self) -> Result<ListManagedSessionEventsQueryRequest, BuildError> {
        Ok(ListManagedSessionEventsQueryRequest {
            after: self.after,
            limit: self.limit,
        })
    }
}

