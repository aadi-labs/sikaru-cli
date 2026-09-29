pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_apps
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListAppsQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ListAppsQueryRequest {
    pub fn builder() -> ListAppsQueryRequestBuilder {
        <ListAppsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListAppsQueryRequestBuilder {
    search: Option<String>,
    category: Option<String>,
    cursor: Option<String>,
    limit: Option<i64>,
}

impl ListAppsQueryRequestBuilder {
    pub fn search(mut self, value: impl Into<String>) -> Self {
        self.search = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn cursor(mut self, value: impl Into<String>) -> Self {
        self.cursor = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListAppsQueryRequest`].
    pub fn build(self) -> Result<ListAppsQueryRequest, BuildError> {
        Ok(ListAppsQueryRequest {
            search: self.search,
            category: self.category,
            cursor: self.cursor,
            limit: self.limit,
        })
    }
}

