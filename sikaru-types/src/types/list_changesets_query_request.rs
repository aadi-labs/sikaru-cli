pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_changesets
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListChangesetsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ListChangesetsChangesetsRequestStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub improvement: Option<bool>,
}

impl ListChangesetsQueryRequest {
    pub fn builder() -> ListChangesetsQueryRequestBuilder {
        <ListChangesetsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListChangesetsQueryRequestBuilder {
    status: Option<ListChangesetsChangesetsRequestStatus>,
    improvement: Option<bool>,
}

impl ListChangesetsQueryRequestBuilder {
    pub fn status(mut self, value: ListChangesetsChangesetsRequestStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn improvement(mut self, value: bool) -> Self {
        self.improvement = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListChangesetsQueryRequest`].
    pub fn build(self) -> Result<ListChangesetsQueryRequest, BuildError> {
        Ok(ListChangesetsQueryRequest {
            status: self.status,
            improvement: self.improvement,
        })
    }
}

