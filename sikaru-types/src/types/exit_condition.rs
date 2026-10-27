pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Fires when the owned process exits.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExitCondition {
    #[serde(default)]
    pub handle_id: String,
}

impl ExitCondition {
    pub fn builder() -> ExitConditionBuilder {
        <ExitConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExitConditionBuilder {
    handle_id: Option<String>,
}

impl ExitConditionBuilder {
    pub fn handle_id(mut self, value: impl Into<String>) -> Self {
        self.handle_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExitCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle_id`](ExitConditionBuilder::handle_id)
    pub fn build(self) -> Result<ExitCondition, BuildError> {
        Ok(ExitCondition {
            handle_id: self.handle_id.ok_or_else(|| BuildError::missing_field("handle_id"))?,
        })
    }
}
