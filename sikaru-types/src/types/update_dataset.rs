pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<UpdateDatasetPurpose>,
}

impl UpdateDataset {
    pub fn builder() -> UpdateDatasetBuilder {
        <UpdateDatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateDatasetBuilder {
    description: Option<String>,
    name: Option<String>,
    purpose: Option<UpdateDatasetPurpose>,
}

impl UpdateDatasetBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: UpdateDatasetPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateDataset`].
    pub fn build(self) -> Result<UpdateDataset, BuildError> {
        Ok(UpdateDataset {
            description: self.description,
            name: self.name,
            purpose: self.purpose,
        })
    }
}

