pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DatasetCheckResults {
    #[serde(default)]
    pub results: Vec<DatasetCheckGroup>,
}

impl DatasetCheckResults {
    pub fn builder() -> DatasetCheckResultsBuilder {
        <DatasetCheckResultsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetCheckResultsBuilder {
    results: Option<Vec<DatasetCheckGroup>>,
}

impl DatasetCheckResultsBuilder {
    pub fn results(mut self, value: Vec<DatasetCheckGroup>) -> Self {
        self.results = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetCheckResults`].
    /// This method will fail if any of the following fields are not set:
    /// - [`results`](DatasetCheckResultsBuilder::results)
    pub fn build(self) -> Result<DatasetCheckResults, BuildError> {
        Ok(DatasetCheckResults {
            results: self.results.ok_or_else(|| BuildError::missing_field("results"))?,
        })
    }
}
