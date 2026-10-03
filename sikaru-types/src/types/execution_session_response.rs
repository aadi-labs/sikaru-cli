pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExecutionSessionResponse {
    #[serde(default)]
    pub session: ExecutionSessionRecord,
}

impl ExecutionSessionResponse {
    pub fn builder() -> ExecutionSessionResponseBuilder {
        <ExecutionSessionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionSessionResponseBuilder {
    session: Option<ExecutionSessionRecord>,
}

impl ExecutionSessionResponseBuilder {
    pub fn session(mut self, value: ExecutionSessionRecord) -> Self {
        self.session = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionSessionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`session`](ExecutionSessionResponseBuilder::session)
    pub fn build(self) -> Result<ExecutionSessionResponse, BuildError> {
        Ok(ExecutionSessionResponse {
            session: self.session.ok_or_else(|| BuildError::missing_field("session"))?,
        })
    }
}
