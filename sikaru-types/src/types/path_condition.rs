pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Fires when a workspace path exists, is missing, or changes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PathCondition {
    #[serde(default)]
    pub path: String,
    pub state: PathConditionState,
}

impl PathCondition {
    pub fn builder() -> PathConditionBuilder {
        <PathConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PathConditionBuilder {
    path: Option<String>,
    state: Option<PathConditionState>,
}

impl PathConditionBuilder {
    pub fn path(mut self, value: impl Into<String>) -> Self {
        self.path = Some(value.into());
        self
    }

    pub fn state(mut self, value: PathConditionState) -> Self {
        self.state = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PathCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`path`](PathConditionBuilder::path)
    /// - [`state`](PathConditionBuilder::state)
    pub fn build(self) -> Result<PathCondition, BuildError> {
        Ok(PathCondition {
            path: self.path.ok_or_else(|| BuildError::missing_field("path"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
