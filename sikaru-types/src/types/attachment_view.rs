pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AttachmentView {
    #[serde(default)]
    pub capabilities: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cleanup_at: Option<f64>,
    pub cleanup_status: AttachmentViewCleanupStatus,
    #[serde(default)]
    pub environment_id: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub journal_id: String,
    /// Renewal TTL; anchor only ready/heartbeat acknowledgments to request-start monotonic time. Status and poll responses never renew the lease.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lease_ttl_seconds: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub lease_until: f64,
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<String>,
    #[serde(default)]
    pub processes: Vec<ProcessObservation>,
    #[serde(default)]
    pub project_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol_version: Option<String>,
    #[serde(default)]
    pub provider_id: String,
    #[serde(default)]
    pub session_id: String,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub startup_deadline: f64,
    pub status: AttachmentViewStatus,
    #[serde(default)]
    pub uncertain_operations: Vec<UncertainOperation>,
    #[serde(default)]
    pub workspace_generation: String,
    pub workspace_provenance: WorkspaceProvenance,
}

impl AttachmentView {
    pub fn builder() -> AttachmentViewBuilder {
        <AttachmentViewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AttachmentViewBuilder {
    capabilities: Option<Vec<String>>,
    cleanup_at: Option<f64>,
    cleanup_status: Option<AttachmentViewCleanupStatus>,
    environment_id: Option<String>,
    id: Option<String>,
    journal_id: Option<String>,
    lease_ttl_seconds: Option<i64>,
    lease_until: Option<f64>,
    owner_epoch: Option<i64>,
    owner_id: Option<String>,
    processes: Option<Vec<ProcessObservation>>,
    project_id: Option<String>,
    protocol_version: Option<String>,
    provider_id: Option<String>,
    session_id: Option<String>,
    startup_deadline: Option<f64>,
    status: Option<AttachmentViewStatus>,
    uncertain_operations: Option<Vec<UncertainOperation>>,
    workspace_generation: Option<String>,
    workspace_provenance: Option<WorkspaceProvenance>,
}

impl AttachmentViewBuilder {
    pub fn capabilities(mut self, value: Vec<String>) -> Self {
        self.capabilities = Some(value);
        self
    }

    pub fn cleanup_at(mut self, value: f64) -> Self {
        self.cleanup_at = Some(value);
        self
    }

    pub fn cleanup_status(mut self, value: AttachmentViewCleanupStatus) -> Self {
        self.cleanup_status = Some(value);
        self
    }

    pub fn environment_id(mut self, value: impl Into<String>) -> Self {
        self.environment_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn journal_id(mut self, value: impl Into<String>) -> Self {
        self.journal_id = Some(value.into());
        self
    }

    pub fn lease_ttl_seconds(mut self, value: i64) -> Self {
        self.lease_ttl_seconds = Some(value);
        self
    }

    pub fn lease_until(mut self, value: f64) -> Self {
        self.lease_until = Some(value);
        self
    }

    pub fn owner_epoch(mut self, value: i64) -> Self {
        self.owner_epoch = Some(value);
        self
    }

    pub fn owner_id(mut self, value: impl Into<String>) -> Self {
        self.owner_id = Some(value.into());
        self
    }

    pub fn processes(mut self, value: Vec<ProcessObservation>) -> Self {
        self.processes = Some(value);
        self
    }

    pub fn project_id(mut self, value: impl Into<String>) -> Self {
        self.project_id = Some(value.into());
        self
    }

    pub fn protocol_version(mut self, value: impl Into<String>) -> Self {
        self.protocol_version = Some(value.into());
        self
    }

    pub fn provider_id(mut self, value: impl Into<String>) -> Self {
        self.provider_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    pub fn startup_deadline(mut self, value: f64) -> Self {
        self.startup_deadline = Some(value);
        self
    }

    pub fn status(mut self, value: AttachmentViewStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn uncertain_operations(mut self, value: Vec<UncertainOperation>) -> Self {
        self.uncertain_operations = Some(value);
        self
    }

    pub fn workspace_generation(mut self, value: impl Into<String>) -> Self {
        self.workspace_generation = Some(value.into());
        self
    }

    pub fn workspace_provenance(mut self, value: WorkspaceProvenance) -> Self {
        self.workspace_provenance = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AttachmentView`].
    /// This method will fail if any of the following fields are not set:
    /// - [`capabilities`](AttachmentViewBuilder::capabilities)
    /// - [`cleanup_status`](AttachmentViewBuilder::cleanup_status)
    /// - [`environment_id`](AttachmentViewBuilder::environment_id)
    /// - [`id`](AttachmentViewBuilder::id)
    /// - [`journal_id`](AttachmentViewBuilder::journal_id)
    /// - [`lease_until`](AttachmentViewBuilder::lease_until)
    /// - [`owner_epoch`](AttachmentViewBuilder::owner_epoch)
    /// - [`processes`](AttachmentViewBuilder::processes)
    /// - [`project_id`](AttachmentViewBuilder::project_id)
    /// - [`provider_id`](AttachmentViewBuilder::provider_id)
    /// - [`session_id`](AttachmentViewBuilder::session_id)
    /// - [`startup_deadline`](AttachmentViewBuilder::startup_deadline)
    /// - [`status`](AttachmentViewBuilder::status)
    /// - [`uncertain_operations`](AttachmentViewBuilder::uncertain_operations)
    /// - [`workspace_generation`](AttachmentViewBuilder::workspace_generation)
    /// - [`workspace_provenance`](AttachmentViewBuilder::workspace_provenance)
    pub fn build(self) -> Result<AttachmentView, BuildError> {
        Ok(AttachmentView {
            capabilities: self.capabilities.ok_or_else(|| BuildError::missing_field("capabilities"))?,
            cleanup_at: self.cleanup_at,
            cleanup_status: self.cleanup_status.ok_or_else(|| BuildError::missing_field("cleanup_status"))?,
            environment_id: self.environment_id.ok_or_else(|| BuildError::missing_field("environment_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            journal_id: self.journal_id.ok_or_else(|| BuildError::missing_field("journal_id"))?,
            lease_ttl_seconds: self.lease_ttl_seconds,
            lease_until: self.lease_until.ok_or_else(|| BuildError::missing_field("lease_until"))?,
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            owner_id: self.owner_id,
            processes: self.processes.ok_or_else(|| BuildError::missing_field("processes"))?,
            project_id: self.project_id.ok_or_else(|| BuildError::missing_field("project_id"))?,
            protocol_version: self.protocol_version,
            provider_id: self.provider_id.ok_or_else(|| BuildError::missing_field("provider_id"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
            startup_deadline: self.startup_deadline.ok_or_else(|| BuildError::missing_field("startup_deadline"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
            uncertain_operations: self.uncertain_operations.ok_or_else(|| BuildError::missing_field("uncertain_operations"))?,
            workspace_generation: self.workspace_generation.ok_or_else(|| BuildError::missing_field("workspace_generation"))?,
            workspace_provenance: self.workspace_provenance.ok_or_else(|| BuildError::missing_field("workspace_provenance"))?,
        })
    }
}
