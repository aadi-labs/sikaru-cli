pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_datasets
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDatasetsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<ListDatasetsDatasetsRequestPurpose>,
}

impl ListDatasetsQueryRequest {
    pub fn builder() -> ListDatasetsQueryRequestBuilder {
        <ListDatasetsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDatasetsQueryRequestBuilder {
    purpose: Option<ListDatasetsDatasetsRequestPurpose>,
}

impl ListDatasetsQueryRequestBuilder {
    pub fn purpose(mut self, value: ListDatasetsDatasetsRequestPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDatasetsQueryRequest`].
    pub fn build(self) -> Result<ListDatasetsQueryRequest, BuildError> {
        Ok(ListDatasetsQueryRequest {
            purpose: self.purpose,
        })
    }
}

