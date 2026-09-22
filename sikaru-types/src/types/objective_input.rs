pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ObjectiveInput {
    #[serde(default)]
    pub criterion: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence_paths: Option<Vec<String>>,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rounds: Option<i64>,
    #[serde(default)]
    pub objective: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub session_id: String,
}

impl ObjectiveInput {
    pub fn builder() -> ObjectiveInputBuilder {
        <ObjectiveInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ObjectiveInputBuilder {
    criterion: Option<String>,
    evidence_paths: Option<Vec<String>>,
    idempotency_key: Option<String>,
    max_rounds: Option<i64>,
    objective: Option<String>,
    run_id: Option<String>,
    session_id: Option<String>,
}

impl ObjectiveInputBuilder {
    pub fn criterion(mut self, value: impl Into<String>) -> Self {
        self.criterion = Some(value.into());
        self
    }

    pub fn evidence_paths(mut self, value: Vec<String>) -> Self {
        self.evidence_paths = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn max_rounds(mut self, value: i64) -> Self {
        self.max_rounds = Some(value);
        self
    }

    pub fn objective(mut self, value: impl Into<String>) -> Self {
        self.objective = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ObjectiveInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`criterion`](ObjectiveInputBuilder::criterion)
    /// - [`idempotency_key`](ObjectiveInputBuilder::idempotency_key)
    /// - [`objective`](ObjectiveInputBuilder::objective)
    /// - [`run_id`](ObjectiveInputBuilder::run_id)
    /// - [`session_id`](ObjectiveInputBuilder::session_id)
    pub fn build(self) -> Result<ObjectiveInput, BuildError> {
        Ok(ObjectiveInput {
            criterion: self.criterion.ok_or_else(|| BuildError::missing_field("criterion"))?,
            evidence_paths: self.evidence_paths,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            max_rounds: self.max_rounds,
            objective: self.objective.ok_or_else(|| BuildError::missing_field("objective"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            session_id: self.session_id.ok_or_else(|| BuildError::missing_field("session_id"))?,
        })
    }
}

