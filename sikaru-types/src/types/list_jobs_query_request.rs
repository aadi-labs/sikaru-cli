pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_jobs
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListJobsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl ListJobsQueryRequest {
    pub fn builder() -> ListJobsQueryRequestBuilder {
        <ListJobsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListJobsQueryRequestBuilder {
    cursor: Option<String>,
}

impl ListJobsQueryRequestBuilder {
    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListJobsQueryRequest`].
    pub fn build(self) -> Result<ListJobsQueryRequest, BuildError> {
        Ok(ListJobsQueryRequest {
            cursor: self.cursor,
        })
    }
}

