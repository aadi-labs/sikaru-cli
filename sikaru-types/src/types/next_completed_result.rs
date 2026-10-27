pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ``jobs.next_completed`` receipt payload: the first to finish, if any, and the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NextCompletedResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<CompletionNotice>,
    #[serde(default)]
    pub pending: Vec<String>,
}

impl NextCompletedResult {
    pub fn builder() -> NextCompletedResultBuilder {
        <NextCompletedResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NextCompletedResultBuilder {
    completed: Option<CompletionNotice>,
    pending: Option<Vec<String>>,
}

impl NextCompletedResultBuilder {
    pub fn completed(mut self, value: CompletionNotice) -> Self {
        self.completed = Some(value);
        self
    }

    pub fn pending(mut self, value: Vec<String>) -> Self {
        self.pending = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NextCompletedResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`pending`](NextCompletedResultBuilder::pending)
    pub fn build(self) -> Result<NextCompletedResult, BuildError> {
        Ok(NextCompletedResult {
            completed: self.completed,
            pending: self.pending.ok_or_else(|| BuildError::missing_field("pending"))?,
        })
    }
}
