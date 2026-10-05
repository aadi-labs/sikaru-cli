pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetExampleResponse {
    pub example: DatasetExample,
}

impl DatasetExampleResponse {
    pub fn builder() -> DatasetExampleResponseBuilder {
        <DatasetExampleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetExampleResponseBuilder {
    example: Option<DatasetExample>,
}

impl DatasetExampleResponseBuilder {
    pub fn example(mut self, value: DatasetExample) -> Self {
        self.example = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetExampleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`example`](DatasetExampleResponseBuilder::example)
    pub fn build(self) -> Result<DatasetExampleResponse, BuildError> {
        Ok(DatasetExampleResponse {
            example: self.example.ok_or_else(|| BuildError::missing_field("example"))?,
        })
    }
}
