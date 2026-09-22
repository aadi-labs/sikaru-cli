pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_policies
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPoliciesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
}

impl ListPoliciesQueryRequest {
    pub fn builder() -> ListPoliciesQueryRequestBuilder {
        <ListPoliciesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPoliciesQueryRequestBuilder {
    after: Option<String>,
}

impl ListPoliciesQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListPoliciesQueryRequest`].
    pub fn build(self) -> Result<ListPoliciesQueryRequest, BuildError> {
        Ok(ListPoliciesQueryRequest {
            after: self.after,
        })
    }
}

