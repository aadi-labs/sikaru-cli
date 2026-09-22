pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_improvements
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListImprovementsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListImprovementsQueryRequest {
    pub fn builder() -> ListImprovementsQueryRequestBuilder {
        <ListImprovementsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListImprovementsQueryRequestBuilder {
    after: Option<String>,
    limit: Option<i64>,
}

impl ListImprovementsQueryRequestBuilder {
    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListImprovementsQueryRequest`].
    pub fn build(self) -> Result<ListImprovementsQueryRequest, BuildError> {
        Ok(ListImprovementsQueryRequest {
            after: self.after,
            limit: self.limit,
        })
    }
}

