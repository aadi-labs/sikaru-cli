pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionSessionPage {
    #[serde(default)]
    pub items: Vec<ExecutionSessionRecord>,
    #[serde(rename = "nextCursor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

impl ExecutionSessionPage {
    pub fn builder() -> ExecutionSessionPageBuilder {
        <ExecutionSessionPageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionSessionPageBuilder {
    items: Option<Vec<ExecutionSessionRecord>>,
    next_cursor: Option<String>,
}

impl ExecutionSessionPageBuilder {
    pub fn items(mut self, value: Vec<ExecutionSessionRecord>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_cursor(mut self, value: impl Into<String>) -> Self {
        self.next_cursor = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExecutionSessionPage`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ExecutionSessionPageBuilder::items)
    pub fn build(self) -> Result<ExecutionSessionPage, BuildError> {
        Ok(ExecutionSessionPage {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_cursor: self.next_cursor,
        })
    }
}
