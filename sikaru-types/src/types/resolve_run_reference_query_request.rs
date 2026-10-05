pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for resolve_run_reference
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResolveRunReferenceQueryRequest {
    #[serde(rename = "asOf")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub as_of: Option<DateTime<FixedOffset>>,
}

impl ResolveRunReferenceQueryRequest {
    pub fn builder() -> ResolveRunReferenceQueryRequestBuilder {
        <ResolveRunReferenceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResolveRunReferenceQueryRequestBuilder {
    as_of: Option<DateTime<FixedOffset>>,
}

impl ResolveRunReferenceQueryRequestBuilder {
    pub fn as_of(mut self, value: DateTime<FixedOffset>) -> Self {
        self.as_of = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResolveRunReferenceQueryRequest`].
    pub fn build(self) -> Result<ResolveRunReferenceQueryRequest, BuildError> {
        Ok(ResolveRunReferenceQueryRequest {
            as_of: self.as_of,
        })
    }
}

