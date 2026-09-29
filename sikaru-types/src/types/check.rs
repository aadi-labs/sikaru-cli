pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Check {
    #[serde(rename = "agentSlug")]
    #[serde(default)]
    pub agent_slug: String,
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    pub environment: CheckEnvironmentView,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub instruction: String,
    #[serde(rename = "latestResult")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_result: Option<CheckResult>,
    #[serde(default)]
    pub name: String,
    pub verification: CheckVerification,
}

impl Check {
    pub fn builder() -> CheckBuilder {
        <CheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CheckBuilder {
    agent_slug: Option<String>,
    created_at: Option<String>,
    environment: Option<CheckEnvironmentView>,
    id: Option<String>,
    instruction: Option<String>,
    latest_result: Option<CheckResult>,
    name: Option<String>,
    verification: Option<CheckVerification>,
}

impl CheckBuilder {
    pub fn agent_slug(mut self, value: impl Into<String>) -> Self {
        self.agent_slug = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn environment(mut self, value: CheckEnvironmentView) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn instruction(mut self, value: impl Into<String>) -> Self {
        self.instruction = Some(value.into());
        self
    }

    pub fn latest_result(mut self, value: CheckResult) -> Self {
        self.latest_result = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn verification(mut self, value: CheckVerification) -> Self {
        self.verification = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Check`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_slug`](CheckBuilder::agent_slug)
    /// - [`created_at`](CheckBuilder::created_at)
    /// - [`environment`](CheckBuilder::environment)
    /// - [`id`](CheckBuilder::id)
    /// - [`instruction`](CheckBuilder::instruction)
    /// - [`name`](CheckBuilder::name)
    /// - [`verification`](CheckBuilder::verification)
    pub fn build(self) -> Result<Check, BuildError> {
        Ok(Check {
            agent_slug: self.agent_slug.ok_or_else(|| BuildError::missing_field("agent_slug"))?,
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            instruction: self.instruction.ok_or_else(|| BuildError::missing_field("instruction"))?,
            latest_result: self.latest_result,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            verification: self.verification.ok_or_else(|| BuildError::missing_field("verification"))?,
        })
    }
}
