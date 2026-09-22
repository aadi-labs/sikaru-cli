pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_comparisons
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListComparisonsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl ListComparisonsQueryRequest {
    pub fn builder() -> ListComparisonsQueryRequestBuilder {
        <ListComparisonsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListComparisonsQueryRequestBuilder {
    after: Option<String>,
}

impl ListComparisonsQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListComparisonsQueryRequest`].
    pub fn build(self) -> Result<ListComparisonsQueryRequest, BuildError> {
        Ok(ListComparisonsQueryRequest {
            after: self.after,
        })
    }
}

