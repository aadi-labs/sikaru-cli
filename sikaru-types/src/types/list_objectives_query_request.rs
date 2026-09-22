pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_objectives
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListObjectivesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListObjectivesQueryRequest {
    pub fn builder() -> ListObjectivesQueryRequestBuilder {
        <ListObjectivesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListObjectivesQueryRequestBuilder {
    session_id: Option<String>,
    status: Option<String>,
    after: Option<String>,
    limit: Option<i64>,
}

impl ListObjectivesQueryRequestBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListObjectivesQueryRequest`].
    pub fn build(self) -> Result<ListObjectivesQueryRequest, BuildError> {
        Ok(ListObjectivesQueryRequest {
            session_id: self.session_id,
            status: self.status,
            after: self.after,
            limit: self.limit,
        })
    }
}

