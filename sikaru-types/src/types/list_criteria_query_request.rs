pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_criteria
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListCriteriaQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl ListCriteriaQueryRequest {
    pub fn builder() -> ListCriteriaQueryRequestBuilder {
        <ListCriteriaQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCriteriaQueryRequestBuilder {
    after: Option<String>,
}

impl ListCriteriaQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListCriteriaQueryRequest`].
    pub fn build(self) -> Result<ListCriteriaQueryRequest, BuildError> {
        Ok(ListCriteriaQueryRequest {
            after: self.after,
        })
    }
}

