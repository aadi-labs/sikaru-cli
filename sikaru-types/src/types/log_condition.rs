pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Fires when the process output matches ``pattern``, a regular expression.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LogCondition {
    #[serde(default)]
    pub handle_id: String,
    #[serde(default)]
    pub pattern: String,
}

impl LogCondition {
    pub fn builder() -> LogConditionBuilder {
        <LogConditionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LogConditionBuilder {
    handle_id: Option<String>,
    pattern: Option<String>,
}

impl LogConditionBuilder {
    pub fn handle_id(mut self, value: impl Into<String>) -> Self {
        self.handle_id = Some(value.into());
        self
    }

    pub fn pattern(mut self, value: impl Into<String>) -> Self {
        self.pattern = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LogCondition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle_id`](LogConditionBuilder::handle_id)
    /// - [`pattern`](LogConditionBuilder::pattern)
    pub fn build(self) -> Result<LogCondition, BuildError> {
        Ok(LogCondition {
            handle_id: self.handle_id.ok_or_else(|| BuildError::missing_field("handle_id"))?,
            pattern: self.pattern.ok_or_else(|| BuildError::missing_field("pattern"))?,
        })
    }
}
