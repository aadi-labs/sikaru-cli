pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NewDataset {
    #[serde(default)]
    pub name: String,
    pub purpose: NewDatasetPurpose,
}

impl NewDataset {
    pub fn builder() -> NewDatasetBuilder {
        <NewDatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NewDatasetBuilder {
    name: Option<String>,
    purpose: Option<NewDatasetPurpose>,
}

impl NewDatasetBuilder {
    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: NewDatasetPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NewDataset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](NewDatasetBuilder::name)
    /// - [`purpose`](NewDatasetBuilder::purpose)
    pub fn build(self) -> Result<NewDataset, BuildError> {
        Ok(NewDataset {
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            purpose: self.purpose.ok_or_else(|| BuildError::missing_field("purpose"))?,
        })
    }
}
