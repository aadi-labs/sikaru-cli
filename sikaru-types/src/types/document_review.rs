pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DocumentReview {
    #[serde(rename = "accessAfter")]
    #[serde(default)]
    pub access_after: HashMap<String, ReviewedAccess>,
    #[serde(rename = "accessBefore")]
    #[serde(default)]
    pub access_before: HashMap<String, ReviewedAccess>,
    #[serde(rename = "accessDelta")]
    #[serde(default)]
    pub access_delta: DocumentAccessDelta,
    #[serde(rename = "accessDigest")]
    #[serde(default)]
    pub access_digest: String,
    #[serde(rename = "affectedSchedules")]
    #[serde(default)]
    pub affected_schedules: Vec<HashMap<String, serde_json::Value>>,
    #[serde(default)]
    pub blockers: Vec<DocumentBlocker>,
    #[serde(rename = "ceilingViolations")]
    #[serde(default)]
    pub ceiling_violations: Vec<DocumentIssue>,
    #[serde(default)]
    pub diff: String,
    #[serde(default)]
    pub document: String,
    #[serde(rename = "earlierSessions")]
    #[serde(default)]
    pub earlier_sessions: Vec<EarlierDocumentSession>,
    #[serde(rename = "harnessVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub harness_version_id: Option<String>,
    #[serde(rename = "liveVersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_version_id: Option<String>,
    #[serde(default)]
    pub revision: i64,
    #[serde(default)]
    pub validation: DocumentValidationView,
}

impl DocumentReview {
    pub fn builder() -> DocumentReviewBuilder {
        <DocumentReviewBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentReviewBuilder {
    access_after: Option<HashMap<String, ReviewedAccess>>,
    access_before: Option<HashMap<String, ReviewedAccess>>,
    access_delta: Option<DocumentAccessDelta>,
    access_digest: Option<String>,
    affected_schedules: Option<Vec<HashMap<String, serde_json::Value>>>,
    blockers: Option<Vec<DocumentBlocker>>,
    ceiling_violations: Option<Vec<DocumentIssue>>,
    diff: Option<String>,
    document: Option<String>,
    earlier_sessions: Option<Vec<EarlierDocumentSession>>,
    harness_version_id: Option<String>,
    live_version_id: Option<String>,
    revision: Option<i64>,
    validation: Option<DocumentValidationView>,
}

impl DocumentReviewBuilder {
    pub fn access_after(mut self, value: HashMap<String, ReviewedAccess>) -> Self {
        self.access_after = Some(value);
        self
    }

    pub fn access_before(mut self, value: HashMap<String, ReviewedAccess>) -> Self {
        self.access_before = Some(value);
        self
    }

    pub fn access_delta(mut self, value: DocumentAccessDelta) -> Self {
        self.access_delta = Some(value);
        self
    }

    pub fn access_digest(mut self, value: impl Into<String>) -> Self {
        self.access_digest = Some(value.into());
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

    pub fn diff(mut self, value: impl Into<String>) -> Self {
        self.diff = Some(value.into());
        self
    }

    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
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

    pub fn live_version_id(mut self, value: impl Into<String>) -> Self {
        self.live_version_id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: i64) -> Self {
        self.revision = Some(value);
        self
    }

    pub fn validation(mut self, value: DocumentValidationView) -> Self {
        self.validation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentReview`].
    /// This method will fail if any of the following fields are not set:
    /// - [`access_after`](DocumentReviewBuilder::access_after)
    /// - [`access_before`](DocumentReviewBuilder::access_before)
    /// - [`access_delta`](DocumentReviewBuilder::access_delta)
    /// - [`access_digest`](DocumentReviewBuilder::access_digest)
    /// - [`affected_schedules`](DocumentReviewBuilder::affected_schedules)
    /// - [`blockers`](DocumentReviewBuilder::blockers)
    /// - [`ceiling_violations`](DocumentReviewBuilder::ceiling_violations)
    /// - [`diff`](DocumentReviewBuilder::diff)
    /// - [`document`](DocumentReviewBuilder::document)
    /// - [`earlier_sessions`](DocumentReviewBuilder::earlier_sessions)
    /// - [`revision`](DocumentReviewBuilder::revision)
    /// - [`validation`](DocumentReviewBuilder::validation)
    pub fn build(self) -> Result<DocumentReview, BuildError> {
        Ok(DocumentReview {
            access_after: self.access_after.ok_or_else(|| BuildError::missing_field("access_after"))?,
            access_before: self.access_before.ok_or_else(|| BuildError::missing_field("access_before"))?,
            access_delta: self.access_delta.ok_or_else(|| BuildError::missing_field("access_delta"))?,
            access_digest: self.access_digest.ok_or_else(|| BuildError::missing_field("access_digest"))?,
            affected_schedules: self.affected_schedules.ok_or_else(|| BuildError::missing_field("affected_schedules"))?,
            blockers: self.blockers.ok_or_else(|| BuildError::missing_field("blockers"))?,
            ceiling_violations: self.ceiling_violations.ok_or_else(|| BuildError::missing_field("ceiling_violations"))?,
            diff: self.diff.ok_or_else(|| BuildError::missing_field("diff"))?,
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            earlier_sessions: self.earlier_sessions.ok_or_else(|| BuildError::missing_field("earlier_sessions"))?,
            harness_version_id: self.harness_version_id,
            live_version_id: self.live_version_id,
            revision: self.revision.ok_or_else(|| BuildError::missing_field("revision"))?,
            validation: self.validation.ok_or_else(|| BuildError::missing_field("validation"))?,
        })
    }
}
