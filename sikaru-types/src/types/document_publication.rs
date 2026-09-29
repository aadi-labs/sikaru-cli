pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentPublication {
    #[serde(rename = "accessDelta")]
    #[serde(default)]
    pub access_delta: DocumentAccessDelta,
    #[serde(rename = "activatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activated_at: Option<String>,
    #[serde(rename = "activationId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    #[serde(rename = "affectedSchedules")]
    #[serde(default)]
    pub affected_schedules: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub blockers: Vec<DocumentBlocker>,
    #[serde(rename = "ceilingViolations")]
    #[serde(default)]
    pub ceiling_violations: Vec<DocumentIssue>,
    #[serde(rename = "contentOrigin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_origin: Option<String>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "earlierSessions")]
    #[serde(default)]
    pub earlier_sessions: Vec<EarlierDocumentSession>,
    #[serde(rename = "harnessVersionId")]
    #[serde(default)]
    pub harness_version_id: String,
    #[serde(rename = "priorVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prior_version_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unchanged: Option<bool>,
}

impl DocumentPublication {
    pub fn builder() -> DocumentPublicationBuilder {
        <DocumentPublicationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentPublicationBuilder {
    access_delta: Option<DocumentAccessDelta>,
    activated_at: Option<String>,
    activation_id: Option<String>,
    actor: Option<String>,
    affected_schedules: Option<Vec<HashMap<String, serde_json::Value>>>,
    blockers: Option<Vec<DocumentBlocker>>,
    ceiling_violations: Option<Vec<DocumentIssue>>,
    content_origin: Option<String>,
    created_at: Option<String>,
    earlier_sessions: Option<Vec<EarlierDocumentSession>>,
    harness_version_id: Option<String>,
    prior_version_id: Option<String>,
    revision: Option<i64>,
    unchanged: Option<bool>,
}

impl DocumentPublicationBuilder {
    pub fn access_delta(mut self, value: DocumentAccessDelta) -> Self {
        self.access_delta = Some(value);
        self
    }

    pub fn activated_at(mut self, value: impl Into<String>) -> Self {
        self.activated_at = Some(value.into());
        self
    }

    pub fn activation_id(mut self, value: impl Into<String>) -> Self {
        self.activation_id = Some(value.into());
        self
    }

    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn affected_schedules(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.affected_schedules = Some(value);
        self
    }

    pub fn blockers(mut self, value: Vec<DocumentBlocker>) -> Self {
        self.blockers = Some(value);
        self
    }

    pub fn ceiling_violations(mut self, value: Vec<DocumentIssue>) -> Self {
        self.ceiling_violations = Some(value);
        self
    }

    pub fn content_origin(mut self, value: impl Into<String>) -> Self {
        self.content_origin = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn earlier_sessions(mut self, value: Vec<EarlierDocumentSession>) -> Self {
        self.earlier_sessions = Some(value);
        self
    }

    pub fn harness_version_id(mut self, value: impl Into<String>) -> Self {
        self.harness_version_id = Some(value.into());
        self
    }

    pub fn prior_version_id(mut self, value: impl Into<String>) -> Self {
        self.prior_version_id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn unchanged(mut self, value: bool) -> Self {
        self.unchanged = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentPublication`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_delta`](DocumentPublicationBuilder::access_delta)
    /// - [`affected_schedules`](DocumentPublicationBuilder::affected_schedules)
    /// - [`blockers`](DocumentPublicationBuilder::blockers)
    /// - [`ceiling_violations`](DocumentPublicationBuilder::ceiling_violations)
    /// - [`earlier_sessions`](DocumentPublicationBuilder::earlier_sessions)
    /// - [`harness_version_id`](DocumentPublicationBuilder::harness_version_id)
    pub fn build(self) -> Result<DocumentPublication, BuildError> {
        Ok(DocumentPublication {
            access_delta: self.access_delta.ok_or_else(|| BuildError::missing_field("access_delta"))?,
            activated_at: self.activated_at,
            activation_id: self.activation_id,
            actor: self.actor,
            affected_schedules: self.affected_schedules.ok_or_else(|| BuildError::missing_field("affected_schedules"))?,
            blockers: self.blockers.ok_or_else(|| BuildError::missing_field("blockers"))?,
            ceiling_violations: self.ceiling_violations.ok_or_else(|| BuildError::missing_field("ceiling_violations"))?,
            content_origin: self.content_origin,
            created_at: self.created_at,
            earlier_sessions: self.earlier_sessions.ok_or_else(|| BuildError::missing_field("earlier_sessions"))?,
            harness_version_id: self.harness_version_id.ok_or_else(|| BuildError::missing_field("harness_version_id"))?,
            prior_version_id: self.prior_version_id,
            revision: self.revision,
            unchanged: self.unchanged,
        })
    }
}
