pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for mine_project_issue_clusters
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MineProjectIssueClustersQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<String>,
}

impl MineProjectIssueClustersQueryRequest {
    pub fn builder() -> MineProjectIssueClustersQueryRequestBuilder {
        <MineProjectIssueClustersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MineProjectIssueClustersQueryRequestBuilder {
    since: Option<String>,
    until: Option<String>,
}

impl MineProjectIssueClustersQueryRequestBuilder {
    pub fn since(mut self, value: impl Into<String>) -> Self {
        self.since = Some(value.into());
        self
    }

    pub fn until(mut self, value: impl Into<String>) -> Self {
        self.until = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`MineProjectIssueClustersQueryRequest`].
    pub fn build(self) -> Result<MineProjectIssueClustersQueryRequest, BuildError> {
        Ok(MineProjectIssueClustersQueryRequest {
            since: self.since,
            until: self.until,
        })
    }
}

