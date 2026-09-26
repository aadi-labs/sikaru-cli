pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// ``jobs.next_completed`` arguments; the host always names the candidate processes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct NextCompletedArguments {
    #[serde(default)]
    pub handle_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tail_bytes: Option<i64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub timeout: f64,
}

impl NextCompletedArguments {
    pub fn builder() -> NextCompletedArgumentsBuilder {
        <NextCompletedArgumentsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NextCompletedArgumentsBuilder {
    handle_ids: Option<Vec<String>>,
    tail_bytes: Option<i64>,
    timeout: Option<f64>,
}

impl NextCompletedArgumentsBuilder {
    pub fn handle_ids(mut self, value: Vec<String>) -> Self {
        self.handle_ids = Some(value);
        self
    }

    pub fn tail_bytes(mut self, value: i64) -> Self {
        self.tail_bytes = Some(value);
        self
    }

    pub fn timeout(mut self, value: f64) -> Self {
        self.timeout = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NextCompletedArguments`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle_ids`](NextCompletedArgumentsBuilder::handle_ids)
    /// - [`timeout`](NextCompletedArgumentsBuilder::timeout)
    pub fn build(self) -> Result<NextCompletedArguments, BuildError> {
        Ok(NextCompletedArguments {
            handle_ids: self.handle_ids.ok_or_else(|| BuildError::missing_field("handle_ids"))?,
            tail_bytes: self.tail_bytes,
            timeout: self.timeout.ok_or_else(|| BuildError::missing_field("timeout"))?,
        })
    }
}
