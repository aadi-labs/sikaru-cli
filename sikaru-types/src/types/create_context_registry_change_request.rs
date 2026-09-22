pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateContextRegistryChangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<CreateContextRegistryChangeRequestAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(rename = "linkedSignal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_signal: Option<String>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(rename = "productionCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_commit: Option<String>,
    #[serde(rename = "productionTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub production_tag: Option<String>,
    #[serde(rename = "repoId")]
    #[serde(default)]
    pub repo_id: String,
    #[serde(rename = "repoType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repo_type: Option<String>,
    #[serde(rename = "sourceTraceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_trace_ids: Option<Vec<String>>,
    #[serde(rename = "stagingCommit")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging_commit: Option<String>,
    #[serde(rename = "stagingTag")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub staging_tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

impl CreateContextRegistryChangeRequest {
    pub fn builder() -> CreateContextRegistryChangeRequestBuilder {
        <CreateContextRegistryChangeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateContextRegistryChangeRequestBuilder {
    action: Option<CreateContextRegistryChangeRequestAction>,
    files: Option<Vec<String>>,
    kind: Option<String>,
    linked_signal: Option<String>,
    name: Option<String>,
    owner: Option<String>,
    production_commit: Option<String>,
    production_tag: Option<String>,
    repo_id: Option<String>,
    repo_type: Option<String>,
    source_trace_ids: Option<Vec<String>>,
    staging_commit: Option<String>,
    staging_tag: Option<String>,
    tools: Option<Vec<String>>,
    variables: Option<Vec<String>>,
    visibility: Option<String>,
}

impl CreateContextRegistryChangeRequestBuilder {
    pub fn action(mut self, value: CreateContextRegistryChangeRequestAction) -> Self {
        self.action = Some(value);
        self
    }

    pub fn files(mut self, value: Vec<String>) -> Self {
        self.files = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn linked_signal(mut self, value: impl Into<String>) -> Self {
        self.linked_signal = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn owner(mut self, value: impl Into<String>) -> Self {
        self.owner = Some(value.into());
        self
    }

    pub fn production_commit(mut self, value: impl Into<String>) -> Self {
        self.production_commit = Some(value.into());
        self
    }

    pub fn production_tag(mut self, value: impl Into<String>) -> Self {
        self.production_tag = Some(value.into());
        self
    }

    pub fn repo_id(mut self, value: impl Into<String>) -> Self {
        self.repo_id = Some(value.into());
        self
    }

    pub fn repo_type(mut self, value: impl Into<String>) -> Self {
        self.repo_type = Some(value.into());
        self
    }

    pub fn source_trace_ids(mut self, value: Vec<String>) -> Self {
        self.source_trace_ids = Some(value);
        self
    }

    pub fn staging_commit(mut self, value: impl Into<String>) -> Self {
        self.staging_commit = Some(value.into());
        self
    }

    pub fn staging_tag(mut self, value: impl Into<String>) -> Self {
        self.staging_tag = Some(value.into());
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn variables(mut self, value: Vec<String>) -> Self {
        self.variables = Some(value);
        self
    }

    pub fn visibility(mut self, value: impl Into<String>) -> Self {
        self.visibility = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateContextRegistryChangeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](CreateContextRegistryChangeRequestBuilder::name)
    /// - [`repo_id`](CreateContextRegistryChangeRequestBuilder::repo_id)
    pub fn build(self) -> Result<CreateContextRegistryChangeRequest, BuildError> {
        Ok(CreateContextRegistryChangeRequest {
            action: self.action,
            files: self.files,
            kind: self.kind,
            linked_signal: self.linked_signal,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            owner: self.owner,
            production_commit: self.production_commit,
            production_tag: self.production_tag,
            repo_id: self.repo_id.ok_or_else(|| BuildError::missing_field("repo_id"))?,
            repo_type: self.repo_type,
            source_trace_ids: self.source_trace_ids,
            staging_commit: self.staging_commit,
            staging_tag: self.staging_tag,
            tools: self.tools,
            variables: self.variables,
            visibility: self.visibility,
        })
    }
}

