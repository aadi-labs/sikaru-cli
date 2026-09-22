pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TraceMetadata {
    #[serde(default)]
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_harness_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compatibility_profile_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub converter_version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub import_generation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    #[serde(default)]
    pub project_id: String,
    #[serde(default)]
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tenant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Additional properties that are not part of the defined schema.
    #[serde(flatten)]
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl TraceMetadata {
    pub fn builder() -> TraceMetadataBuilder {
        <TraceMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TraceMetadataBuilder {
    account_id: Option<String>,
    base_harness_version_id: Option<String>,
    compatibility_profile_id: Option<String>,
    conversation_id: Option<String>,
    converter_version: Option<String>,
    correlation_id: Option<String>,
    harness_id: Option<String>,
    harness_version_id: Option<String>,
    import_generation_id: Option<String>,
    job_id: Option<String>,
    organization_id: Option<String>,
    product_id: Option<String>,
    project_id: Option<String>,
    source: Option<String>,
    source_instance: Option<String>,
    tenant_id: Option<String>,
    trace_id: Option<String>,
    user_id: Option<String>,
}

impl TraceMetadataBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn base_harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.base_harness_version_id = Some(value.into());
        self
    }

    pub fn compatibility_profile_id(mut self, value: impl Into<String>) -> Self {
        self.compatibility_profile_id = Some(value.into());
        self
    }

    pub fn conversation_id(mut self, value: impl Into<String>) -> Self {
        self.conversation_id = Some(value.into());
        self
    }

    pub fn converter_version(mut self, value: impl Into<String>) -> Self {
        self.converter_version = Some(value.into());
        self
    }

    pub fn correlation_id(mut self, value: impl Into<String>) -> Self {
        self.correlation_id = Some(value.into());
        self
    }

    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn import_generation_id(mut self, value: impl Into<String>) -> Self {
        self.import_generation_id = Some(value.into());
        self
    }

    pub fn job_id(mut self, value: impl Into<String>) -> Self {
        self.job_id = Some(value.into());
        self
    }

    pub fn organization_id(mut self, value: impl Into<String>) -> Self {
        self.organization_id = Some(value.into());
        self
    }

    pub fn product_id(mut self, value: impl Into<String>) -> Self {
        self.product_id = Some(value.into());
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn source_instance(mut self, value: impl Into<String>) -> Self {
        self.source_instance = Some(value.into());
        self
    }

    pub fn tenant_id(mut self, value: impl Into<String>) -> Self {
        self.tenant_id = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TraceMetadata`].
    /// This method will fail if any of the following fields are not set:
    /// - [`account_id`](TraceMetadataBuilder::account_id)
    /// - [`project_id`](TraceMetadataBuilder::project_id)
    /// - [`source`](TraceMetadataBuilder::source)
    pub fn build(self) -> Result<TraceMetadata, BuildError> {
        Ok(TraceMetadata {
            account_id: self.account_id.ok_or_else(|| BuildError::missing_field("account_id"))?,
            base_harness_version_id: self.base_harness_version_id,
            compatibility_profile_id: self.compatibility_profile_id,
            conversation_id: self.conversation_id,
            converter_version: self.converter_version,
            correlation_id: self.correlation_id,
            harness_id: self.harness_id,
            harness_version_id: self.harness_version_id,
            import_generation_id: self.import_generation_id,
            job_id: self.job_id,
            organization_id: self.organization_id,
            product_id: self.product_id,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            source: self.source.ok_or_else(|| BuildError::missing_field("source"))?,
            source_instance: self.source_instance,
            tenant_id: self.tenant_id,
            trace_id: self.trace_id,
            user_id: self.user_id,
            extra: Default::default(),
        })
    }
}
