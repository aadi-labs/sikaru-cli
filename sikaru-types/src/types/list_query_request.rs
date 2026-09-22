pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_slug: Option<String>,
}

impl ListQueryRequest {
    pub fn builder() -> ListQueryRequestBuilder {
        <ListQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListQueryRequestBuilder {
    harness_id: Option<String>,
    after: Option<String>,
    limit: Option<i64>,
    agent_slug: Option<String>,
}

impl ListQueryRequestBuilder {
    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn after(mut self, value: impl Into<String>) -> Self {
        self.after = Some(value.into());
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListQueryRequest`].
    pub fn build(self) -> Result<ListQueryRequest, BuildError> {
        Ok(ListQueryRequest {
            harness_id: self.harness_id,
            after: self.after,
            limit: self.limit,
            agent_slug: self.agent_slug,
        })
    }
}

