pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StartDatasetChecks {
    #[serde(default)]
    pub agent_slug: String,
    /// Omit for the latest dataset version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataset_version: Option<i64>,
    #[serde(default)]
    pub idempotency_key: String,
    /// The agent version to check; Checks run on the agent's live version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl StartDatasetChecks {
    pub fn builder() -> StartDatasetChecksBuilder {
        <StartDatasetChecksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StartDatasetChecksBuilder {
    agent_slug: Option<String>,
    dataset_version: Option<i64>,
    idempotency_key: Option<String>,
    version: Option<String>,
}

impl StartDatasetChecksBuilder {
    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn dataset_version(mut self, value: i64) -> Self {
        self.dataset_version = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StartDatasetChecks`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](StartDatasetChecksBuilder::agent_slug)
    /// - [`idempotency_key`](StartDatasetChecksBuilder::idempotency_key)
    pub fn build(self) -> Result<StartDatasetChecks, BuildError> {
        Ok(StartDatasetChecks {
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            dataset_version: self.dataset_version,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            version: self.version,
        })
    }
}

