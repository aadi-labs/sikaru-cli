pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_issue_clusters
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListIssueClustersQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<String>,
}

impl ListIssueClustersQueryRequest {
    pub fn builder() -> ListIssueClustersQueryRequestBuilder {
        <ListIssueClustersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListIssueClustersQueryRequestBuilder {
    status: Option<String>,
    severity: Option<String>,
}

impl ListIssueClustersQueryRequestBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn severity(mut self, value: impl Into<String>) -> Self {
        self.severity = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListIssueClustersQueryRequest`].
    pub fn build(self) -> Result<ListIssueClustersQueryRequest, BuildError> {
        Ok(ListIssueClustersQueryRequest {
            status: self.status,
            severity: self.severity,
        })
    }
}

