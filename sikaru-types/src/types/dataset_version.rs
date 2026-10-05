pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DatasetVersion {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: String,
    #[serde(rename = "exampleCount")]
    #[serde(default)]
    pub example_count: i64,
    #[serde(default)]
    pub note: String,
    #[serde(default)]
    pub number: i64,
}

impl DatasetVersion {
    pub fn builder() -> DatasetVersionBuilder {
        <DatasetVersionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetVersionBuilder {
    created_at: Option<String>,
    created_by: Option<String>,
    example_count: Option<i64>,
    note: Option<String>,
    number: Option<i64>,
}

impl DatasetVersionBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn example_count(mut self, value: i64) -> Self {
        self.example_count = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn number(mut self, value: i64) -> Self {
        self.number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetVersion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DatasetVersionBuilder::created_at)
    /// - [`created_by`](DatasetVersionBuilder::created_by)
    /// - [`example_count`](DatasetVersionBuilder::example_count)
    /// - [`note`](DatasetVersionBuilder::note)
    /// - [`number`](DatasetVersionBuilder::number)
    pub fn build(self) -> Result<DatasetVersion, BuildError> {
        Ok(DatasetVersion {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            example_count: self.example_count.ok_or_else(|| BuildError::missing_field("example_count"))?,
            note: self.note.ok_or_else(|| BuildError::missing_field("note"))?,
            number: self.number.ok_or_else(|| BuildError::missing_field("number"))?,
        })
    }
}
