pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_dataset_check_results
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetCheckResultsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl ListDatasetCheckResultsQueryRequest {
    pub fn builder() -> ListDatasetCheckResultsQueryRequestBuilder {
        <ListDatasetCheckResultsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetCheckResultsQueryRequestBuilder {
    version: Option<i64>,
}

impl ListDatasetCheckResultsQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetCheckResultsQueryRequest`].
    pub fn build(self) -> Result<ListDatasetCheckResultsQueryRequest, BuildError> {
        Ok(ListDatasetCheckResultsQueryRequest {
            version: self.version,
        })
    }
}

