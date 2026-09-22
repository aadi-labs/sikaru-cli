pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PolicyState {
    #[serde(default)]
    pub enabled: bool,
}

impl PolicyState {
    pub fn builder() -> PolicyStateBuilder {
        <PolicyStateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PolicyStateBuilder {
    enabled: Option<bool>,
}

impl PolicyStateBuilder {
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PolicyState`].
    /// This method will fail if any of the following fields are not set:
    /// - [`enabled`](PolicyStateBuilder::enabled)
    pub fn build(self) -> Result<PolicyState, BuildError> {
        Ok(PolicyState {
            enabled: self.enabled.ok_or_else(|| BuildError::missing_field("enabled"))?,
        })
    }
}

