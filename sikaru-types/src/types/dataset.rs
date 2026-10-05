pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Dataset {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: String,
    #[serde(default)]
    pub description: String,
    #[serde(rename = "exampleCount")]
    #[serde(default)]
    pub example_count: i64,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<DatasetPurpose>,
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    #[serde(rename = "usedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<DatasetUsage>,
    #[serde(default)]
    pub version: i64,
}

impl Dataset {
    pub fn builder() -> DatasetBuilder {
        <DatasetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetBuilder {
    created_at: Option<String>,
    created_by: Option<String>,
    description: Option<String>,
    example_count: Option<i64>,
    id: Option<String>,
    name: Option<String>,
    purpose: Option<DatasetPurpose>,
    updated_at: Option<String>,
    used_by: Option<DatasetUsage>,
    version: Option<i64>,
}

impl DatasetBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn example_count(mut self, value: i64) -> Self {
        self.example_count = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn purpose(mut self, value: DatasetPurpose) -> Self {
        self.purpose = Some(value);
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn used_by(mut self, value: DatasetUsage) -> Self {
        self.used_by = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Dataset`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DatasetBuilder::created_at)
    /// - [`created_by`](DatasetBuilder::created_by)
    /// - [`description`](DatasetBuilder::description)
    /// - [`example_count`](DatasetBuilder::example_count)
    /// - [`id`](DatasetBuilder::id)
    /// - [`name`](DatasetBuilder::name)
    /// - [`updated_at`](DatasetBuilder::updated_at)
    /// - [`version`](DatasetBuilder::version)
    pub fn build(self) -> Result<Dataset, BuildError> {
        Ok(Dataset {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
            example_count: self.example_count.ok_or_else(|| BuildError::missing_field("example_count"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            purpose: self.purpose,
            updated_at: self.updated_at.ok_or_else(|| BuildError::missing_field("updated_at"))?,
            used_by: self.used_by,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
