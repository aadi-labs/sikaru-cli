pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetList {
    #[serde(default)]
    pub datasets: Vec<Dataset>,
}

impl DatasetList {
    pub fn builder() -> DatasetListBuilder {
        <DatasetListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetListBuilder {
    datasets: Option<Vec<Dataset>>,
}

impl DatasetListBuilder {
    pub fn datasets(mut self, value: Vec<Dataset>) -> Self {
        self.datasets = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`datasets`](DatasetListBuilder::datasets)
    pub fn build(self) -> Result<DatasetList, BuildError> {
        Ok(DatasetList {
            datasets: self.datasets.ok_or_else(|| BuildError::missing_field("datasets"))?,
        })
    }
}
