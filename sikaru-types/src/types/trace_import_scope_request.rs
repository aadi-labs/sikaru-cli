pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TraceImportScopeRequest {
    #[serde(rename = "externalIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub since: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until: Option<DateTime<FixedOffset>>,
}

impl TraceImportScopeRequest {
    pub fn builder() -> TraceImportScopeRequestBuilder {
        <TraceImportScopeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceImportScopeRequestBuilder {
    external_ids: Option<Vec<String>>,
    since: Option<DateTime<FixedOffset>>,
    until: Option<DateTime<FixedOffset>>,
}

impl TraceImportScopeRequestBuilder {
    pub fn external_ids(mut self, value: Vec<String>) -> Self {
        self.external_ids = Some(value);
        self
    }

    pub fn since(mut self, value: DateTime<FixedOffset>) -> Self {
        self.since = Some(value);
        self
    }

    pub fn until(mut self, value: DateTime<FixedOffset>) -> Self {
        self.until = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TraceImportScopeRequest`].
    pub fn build(self) -> Result<TraceImportScopeRequest, BuildError> {
        Ok(TraceImportScopeRequest {
            external_ids: self.external_ids,
            since: self.since,
            until: self.until,
        })
    }
}
