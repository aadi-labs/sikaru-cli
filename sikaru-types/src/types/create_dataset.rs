pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateDataset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub idempotency_key: Option<String>,
    #[serde(default)]
    pub name: String,
    pub purpose: CreateDatasetPurpose,
}

impl CreateDataset {
    pub fn builder() -> CreateDatasetBuilder {
        <CreateDatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateDatasetBuilder {
    description: Option<String>,
    idempotency_key: Option<String>,
    name: Option<String>,
    purpose: Option<CreateDatasetPurpose>,
}

impl CreateDatasetBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: CreateDatasetPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateDataset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateDatasetBuilder::name)
    /// - [`purpose`](CreateDatasetBuilder::purpose)
    pub fn build(self) -> Result<CreateDataset, BuildError> {
        Ok(CreateDataset {
            description: self.description,
            idempotency_key: self.idempotency_key,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            purpose: self.purpose.ok_or_else(|| BuildError::missing_field("purpose"))?,
        })
    }
}

