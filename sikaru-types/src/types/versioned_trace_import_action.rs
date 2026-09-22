pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VersionedTraceImportAction {
    #[serde(rename = "expectedVersion")]
    #[serde(default)]
    pub expected_version: i64,
}

impl VersionedTraceImportAction {
    pub fn builder() -> VersionedTraceImportActionBuilder {
        <VersionedTraceImportActionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VersionedTraceImportActionBuilder {
    expected_version: Option<i64>,
}

impl VersionedTraceImportActionBuilder {
    pub fn expected_version(mut self, value: i64) -> Self {
        self.expected_version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VersionedTraceImportAction`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expected_version`](VersionedTraceImportActionBuilder::expected_version)
    pub fn build(self) -> Result<VersionedTraceImportAction, BuildError> {
        Ok(VersionedTraceImportAction {
            expected_version: self.expected_version.ok_or_else(|| BuildError::missing_field("expected_version"))?,
        })
    }
}
