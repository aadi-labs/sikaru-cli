pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TraceImportGroupingRequest {
    #[serde(rename = "contractVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_version: Option<String>,
    #[serde(rename = "joinOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_on: Option<String>,
}

impl TraceImportGroupingRequest {
    pub fn builder() -> TraceImportGroupingRequestBuilder {
        <TraceImportGroupingRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceImportGroupingRequestBuilder {
    contract_version: Option<String>,
    join_on: Option<String>,
}

impl TraceImportGroupingRequestBuilder {
    pub fn contract_version(mut self, value: impl Into<String>) -> Self {
        self.contract_version = Some(value.into());
        self
    }

    pub fn join_on(mut self, value: impl Into<String>) -> Self {
        self.join_on = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TraceImportGroupingRequest`].
    pub fn build(self) -> Result<TraceImportGroupingRequest, BuildError> {
        Ok(TraceImportGroupingRequest {
            contract_version: self.contract_version,
            join_on: self.join_on,
        })
    }
}
