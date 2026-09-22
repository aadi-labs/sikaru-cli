pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateTraceImportRequest {
    #[serde(rename = "connectionId")]
    #[serde(default)]
    pub connection_id: String,
    #[serde(rename = "converterVersion")]
    #[serde(default)]
    pub converter_version: String,
    #[serde(default)]
    pub dataset: String,
    #[serde(rename = "externalProjectId")]
    #[serde(default)]
    pub external_project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grouping: Option<TraceImportGroupingRequest>,
    pub mode: CreateTraceImportRequestMode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<TraceImportOptionsRequest>,
    pub provider: CreateTraceImportRequestProvider,
    #[serde(default)]
    pub scope: TraceImportScopeRequest,
    #[serde(rename = "sourceInstance")]
    #[serde(default)]
    pub source_instance: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl CreateTraceImportRequest {
    pub fn builder() -> CreateTraceImportRequestBuilder {
        <CreateTraceImportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateTraceImportRequestBuilder {
    connection_id: Option<String>,
    converter_version: Option<String>,
    dataset: Option<String>,
    external_project_id: Option<String>,
    grouping: Option<TraceImportGroupingRequest>,
    mode: Option<CreateTraceImportRequestMode>,
    options: Option<TraceImportOptionsRequest>,
    provider: Option<CreateTraceImportRequestProvider>,
    scope: Option<TraceImportScopeRequest>,
    source_instance: Option<String>,
    tags: Option<Vec<String>>,
}

impl CreateTraceImportRequestBuilder {
    pub fn connection_id(mut self, value: impl Into<String>) -> Self {
        self.connection_id = Some(value.into());
        self
    }

    pub fn converter_version(mut self, value: impl Into<String>) -> Self {
        self.converter_version = Some(value.into());
        self
    }

    pub fn dataset(mut self, value: impl Into<String>) -> Self {
        self.dataset = Some(value.into());
        self
    }

    pub fn external_project_id(mut self, value: impl Into<String>) -> Self {
        self.external_project_id = Some(value.into());
        self
    }

    pub fn grouping(mut self, value: TraceImportGroupingRequest) -> Self {
        self.grouping = Some(value);
        self
    }

    pub fn mode(mut self, value: CreateTraceImportRequestMode) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn options(mut self, value: TraceImportOptionsRequest) -> Self {
        self.options = Some(value);
        self
    }

    pub fn provider(mut self, value: CreateTraceImportRequestProvider) -> Self {
        self.provider = Some(value);
        self
    }

    pub fn scope(mut self, value: TraceImportScopeRequest) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn source_instance(mut self, value: impl Into<String>) -> Self {
        self.source_instance = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateTraceImportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`connection_id`](CreateTraceImportRequestBuilder::connection_id)
    /// - [`converter_version`](CreateTraceImportRequestBuilder::converter_version)
    /// - [`dataset`](CreateTraceImportRequestBuilder::dataset)
    /// - [`external_project_id`](CreateTraceImportRequestBuilder::external_project_id)
    /// - [`mode`](CreateTraceImportRequestBuilder::mode)
    /// - [`provider`](CreateTraceImportRequestBuilder::provider)
    /// - [`scope`](CreateTraceImportRequestBuilder::scope)
    /// - [`source_instance`](CreateTraceImportRequestBuilder::source_instance)
    pub fn build(self) -> Result<CreateTraceImportRequest, BuildError> {
        Ok(CreateTraceImportRequest {
            connection_id: self.connection_id.ok_or_else(|| BuildError::missing_field("connection_id"))?,
            converter_version: self.converter_version.ok_or_else(|| BuildError::missing_field("converter_version"))?,
            dataset: self.dataset.ok_or_else(|| BuildError::missing_field("dataset"))?,
            external_project_id: self.external_project_id.ok_or_else(|| BuildError::missing_field("external_project_id"))?,
            grouping: self.grouping,
            mode: self.mode.ok_or_else(|| BuildError::missing_field("mode"))?,
            options: self.options,
            provider: self.provider.ok_or_else(|| BuildError::missing_field("provider"))?,
            scope: self.scope.ok_or_else(|| BuildError::missing_field("scope"))?,
            source_instance: self.source_instance.ok_or_else(|| BuildError::missing_field("source_instance"))?,
            tags: self.tags,
        })
    }
}

