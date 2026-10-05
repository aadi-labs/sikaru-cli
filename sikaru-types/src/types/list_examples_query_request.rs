pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_examples
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListExamplesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
}

impl ListExamplesQueryRequest {
    pub fn builder() -> ListExamplesQueryRequestBuilder {
        <ListExamplesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListExamplesQueryRequestBuilder {
    version: Option<i64>,
    limit: Option<i64>,
    offset: Option<i64>,
}

impl ListExamplesQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn offset(mut self, value: i64) -> Self {
        self.offset = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListExamplesQueryRequest`].
    pub fn build(self) -> Result<ListExamplesQueryRequest, BuildError> {
        Ok(ListExamplesQueryRequest {
            version: self.version,
            limit: self.limit,
            offset: self.offset,
        })
    }
}

