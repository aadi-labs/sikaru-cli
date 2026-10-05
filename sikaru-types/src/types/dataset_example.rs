pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DatasetExample {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub input: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub metadata: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub provenance: HashMap<String, DatasetExampleProvenanceValue>,
    pub source: ExampleSource,
    /// Sikaru's own labels appear only in the signed-in dashboard.
    #[serde(default)]
    pub tags: Vec<String>,
    /// How many tags are Sikaru's labels: never exported, and left out of `tags` for API keys.
    #[serde(rename = "tagsKeptInSikaru")]
    #[serde(default)]
    pub tags_kept_in_sikaru: i64,
    #[serde(default)]
    pub version: i64,
}

impl DatasetExample {
    pub fn builder() -> DatasetExampleBuilder {
        <DatasetExampleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DatasetExampleBuilder {
    created_at: Option<String>,
    created_by: Option<String>,
    expected: Option<HashMap<String, serde_json::Value>>,
    id: Option<String>,
    input: Option<Vec<HashMap<String, serde_json::Value>>>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    provenance: Option<HashMap<String, DatasetExampleProvenanceValue>>,
    source: Option<ExampleSource>,
    tags: Option<Vec<String>>,
    tags_kept_in_sikaru: Option<i64>,
    version: Option<i64>,
}

impl DatasetExampleBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn expected(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.expected = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn input(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn provenance(mut self, value: HashMap<String, DatasetExampleProvenanceValue>) -> Self {
        self.provenance = Some(value);
        self
    }

    pub fn source(mut self, value: ExampleSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn tags_kept_in_sikaru(mut self, value: i64) -> Self {
        self.tags_kept_in_sikaru = Some(value);
        self
    }

    pub fn version(mut self, value: i64) -> Self {
        self.version = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DatasetExample`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](DatasetExampleBuilder::created_at)
    /// - [`created_by`](DatasetExampleBuilder::created_by)
    /// - [`id`](DatasetExampleBuilder::id)
    /// - [`input`](DatasetExampleBuilder::input)
    /// - [`metadata`](DatasetExampleBuilder::metadata)
    /// - [`provenance`](DatasetExampleBuilder::provenance)
    /// - [`source`](DatasetExampleBuilder::source)
    /// - [`tags`](DatasetExampleBuilder::tags)
    /// - [`tags_kept_in_sikaru`](DatasetExampleBuilder::tags_kept_in_sikaru)
    /// - [`version`](DatasetExampleBuilder::version)
    pub fn build(self) -> Result<DatasetExample, BuildError> {
        Ok(DatasetExample {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            created_by: self.created_by.ok_or_else(|| BuildError::missing_field("created_by"))?,
            expected: self.expected,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            metadata: self.metadata.ok_or_else(|| BuildError::missing_field("metadata"))?,
            provenance: self.provenance.ok_or_else(|| BuildError::missing_field("provenance"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            tags: self.tags.ok_or_else(|| BuildError::missing_field("tags"))?,
            tags_kept_in_sikaru: self.tags_kept_in_sikaru.ok_or_else(|| BuildError::missing_field("tags_kept_in_sikaru"))?,
            version: self.version.ok_or_else(|| BuildError::missing_field("version"))?,
        })
    }
}
