pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateCheck {
    pub environment: CheckEnvironment,
    #[serde(default)]
    pub idempotency_key: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub task: HarborTaskFiles,
}

impl CreateCheck {
    pub fn builder() -> CreateCheckBuilder {
        <CreateCheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCheckBuilder {
    environment: Option<CheckEnvironment>,
    idempotency_key: Option<String>,
    name: Option<String>,
    task: Option<HarborTaskFiles>,
}

impl CreateCheckBuilder {
    pub fn environment(mut self, value: CheckEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    pub fn idempotency_key(mut self, value: impl Into<String>) -> Self {
        self.idempotency_key = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn task(mut self, value: HarborTaskFiles) -> Self {
        self.task = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCheck`].
    /// This method will fail if any of the following fields are not set:
    /// - [`environment`](CreateCheckBuilder::environment)
    /// - [`idempotency_key`](CreateCheckBuilder::idempotency_key)
    /// - [`name`](CreateCheckBuilder::name)
    /// - [`task`](CreateCheckBuilder::task)
    pub fn build(self) -> Result<CreateCheck, BuildError> {
        Ok(CreateCheck {
            environment: self.environment.ok_or_else(|| BuildError::missing_field("environment"))?,
            idempotency_key: self.idempotency_key.ok_or_else(|| BuildError::missing_field("idempotency_key"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            task: self.task.ok_or_else(|| BuildError::missing_field("task"))?,
        })
    }
}

