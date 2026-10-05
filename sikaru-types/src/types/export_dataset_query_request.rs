pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for export_dataset
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExportDatasetQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<i64>,
}

impl ExportDatasetQueryRequest {
    pub fn builder() -> ExportDatasetQueryRequestBuilder {
        <ExportDatasetQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportDatasetQueryRequestBuilder {
    version: Option<i64>,
}

impl ExportDatasetQueryRequestBuilder {
    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportDatasetQueryRequest`].
    pub fn build(self) -> Result<ExportDatasetQueryRequest, BuildError> {
        Ok(ExportDatasetQueryRequest {
            version: self.version,
        })
    }
}

