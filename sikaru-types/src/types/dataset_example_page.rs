pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetExamplePage {
    #[serde(default)]
    pub examples: Vec<DatasetExample>,
    #[serde(default)]
    pub latest: i64,
    #[serde(default)]
    pub total: i64,
    #[serde(default)]
    pub version: i64,
}

impl DatasetExamplePage {
    pub fn builder() -> DatasetExamplePageBuilder {
        <DatasetExamplePageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetExamplePageBuilder {
    examples: Option<Vec<DatasetExample>>,
    latest: Option<i64>,
    total: Option<i64>,
    version: Option<i64>,
}

impl DatasetExamplePageBuilder {
    pub fn examples(mut self, value: Vec<DatasetExample>) -> Self {
        self.examples = Some(value);
        self
    }

    pub fn latest(mut self, value: i64) -> Self {
        self.latest = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetExamplePage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`examples`](DatasetExamplePageBuilder::examples)
    /// - [`latest`](DatasetExamplePageBuilder::latest)
    /// - [`total`](DatasetExamplePageBuilder::total)
    /// - [`version`](DatasetExamplePageBuilder::version)
    pub fn build(self) -> Result<DatasetExamplePage, BuildError> {
        Ok(DatasetExamplePage {
            examples: self.examples.ok_or_else(|| BuildError::missing_field("examples"))?,
            latest: self.latest.ok_or_else(|| BuildError::missing_field("latest"))?,
            total: self.total.ok_or_else(|| BuildError::missing_field("total"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
