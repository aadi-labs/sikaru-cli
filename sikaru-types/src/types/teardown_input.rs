pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TeardownInput {
    #[serde(default)]
    pub children_terminated: bool,
    #[serde(default)]
    pub evidence: String,
    #[serde(default)]
    pub owner_epoch: i64,
    #[serde(default)]
    pub owner_id: String,
    #[serde(default)]
    pub workspace_generation: String,
}

impl TeardownInput {
    pub fn builder() -> TeardownInputBuilder {
        <TeardownInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TeardownInputBuilder {
    children_terminated: Option<bool>,
    evidence: Option<String>,
    owner_epoch: Option<i64>,
    owner_id: Option<String>,
    workspace_generation: Option<String>,
}

impl TeardownInputBuilder {
    pub fn children_terminated(mut self, value: bool) -> Self {
        self.children_terminated = Some(value);
        self
    }

    pub fn evidence(mut self, value: impl Into<String>) -> Self {
        self.evidence = Some(value.into());
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

    pub fn workspace_generation(mut self, value: impl Into<String>) -> Self {
        self.workspace_generation = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TeardownInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`children_terminated`](TeardownInputBuilder::children_terminated)
    /// - [`evidence`](TeardownInputBuilder::evidence)
    /// - [`owner_epoch`](TeardownInputBuilder::owner_epoch)
    /// - [`owner_id`](TeardownInputBuilder::owner_id)
    /// - [`workspace_generation`](TeardownInputBuilder::workspace_generation)
    pub fn build(self) -> Result<TeardownInput, BuildError> {
        Ok(TeardownInput {
            children_terminated: self.children_terminated.ok_or_else(|| BuildError::missing_field("children_terminated"))?,
            evidence: self.evidence.ok_or_else(|| BuildError::missing_field("evidence"))?,
            owner_epoch: self.owner_epoch.ok_or_else(|| BuildError::missing_field("owner_epoch"))?,
            owner_id: self.owner_id.ok_or_else(|| BuildError::missing_field("owner_id"))?,
            workspace_generation: self.workspace_generation.ok_or_else(|| BuildError::missing_field("workspace_generation"))?,
        })
    }
}

