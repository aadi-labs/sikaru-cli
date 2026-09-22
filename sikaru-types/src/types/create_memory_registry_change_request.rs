pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateMemoryRegistryChangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<CreateMemoryRegistryChangeRequestAction>,
    #[serde(rename = "candidateReleaseId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub candidate_release_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confidence: Option<String>,
    #[serde(rename = "memoryId")]
    #[serde(default)]
    pub memory_id: String,
    #[serde(rename = "memoryType")]
    #[serde(default)]
    pub memory_type: String,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provenance: Option<String>,
    #[serde(default)]
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "sourceTraceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_trace_ids: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

impl CreateMemoryRegistryChangeRequest {
    pub fn builder() -> CreateMemoryRegistryChangeRequestBuilder {
        <CreateMemoryRegistryChangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateMemoryRegistryChangeRequestBuilder {
    action: Option<CreateMemoryRegistryChangeRequestAction>,
    candidate_release_id: Option<String>,
    confidence: Option<String>,
    memory_id: Option<String>,
    memory_type: Option<String>,
    name: Option<String>,
    policy: Option<String>,
    provenance: Option<String>,
    scope: Option<String>,
    source: Option<String>,
    source_trace_ids: Option<Vec<String>>,
    ttl: Option<String>,
}

impl CreateMemoryRegistryChangeRequestBuilder {
    pub fn action(mut self, value: CreateMemoryRegistryChangeRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn candidate_release_id(mut self, value: impl Into<String>) -> Self {
        self.candidate_release_id = Some(value.into());
        self
    }

    pub fn confidence(mut self, value: impl Into<String>) -> Self {
        self.confidence = Some(value.into());
        self
    }

    pub fn memory_id(mut self, value: impl Into<String>) -> Self {
        self.memory_id = Some(value.into());
        self
    }

    pub fn memory_type(mut self, value: impl Into<String>) -> Self {
        self.memory_type = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn policy(mut self, value: impl Into<String>) -> Self {
        self.policy = Some(value.into());
        self
    }

    pub fn provenance(mut self, value: impl Into<String>) -> Self {
        self.provenance = Some(value.into());
        self
    }

    pub fn scope(mut self, value: impl Into<String>) -> Self {
        self.scope = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn source_trace_ids(mut self, value: Vec<String>) -> Self {
        self.source_trace_ids = Some(value);
        self
    }

    pub fn ttl(mut self, value: impl Into<String>) -> Self {
        self.ttl = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateMemoryRegistryChangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`memory_id`](CreateMemoryRegistryChangeRequestBuilder::memory_id)
    /// - [`memory_type`](CreateMemoryRegistryChangeRequestBuilder::memory_type)
    /// - [`name`](CreateMemoryRegistryChangeRequestBuilder::name)
    /// - [`scope`](CreateMemoryRegistryChangeRequestBuilder::scope)
    pub fn build(self) -> Result<CreateMemoryRegistryChangeRequest, BuildError> {
        Ok(CreateMemoryRegistryChangeRequest {
            action: self.action,
            candidate_release_id: self.candidate_release_id,
            confidence: self.confidence,
            memory_id: self.memory_id.ok_or_else(|| BuildError::missing_field("memory_id"))?,
            memory_type: self.memory_type.ok_or_else(|| BuildError::missing_field("memory_type"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            policy: self.policy,
            provenance: self.provenance,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            source: self.source,
            source_trace_ids: self.source_trace_ids,
            ttl: self.ttl,
        })
    }
}

