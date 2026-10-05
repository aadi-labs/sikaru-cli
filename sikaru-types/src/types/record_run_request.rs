pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RecordRunRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    #[serde(default)]
    pub input: String,
    #[serde(default)]
    pub output: String,
}

impl RecordRunRequest {
    pub fn builder() -> RecordRunRequestBuilder {
        <RecordRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RecordRunRequestBuilder {
    account_id: Option<String>,
    agent_name: Option<String>,
    input: Option<String>,
    output: Option<String>,
}

impl RecordRunRequestBuilder {
    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn agent_name(mut self, value: impl Into<String>) -> Self {
        self.agent_name = Some(value.into());
        self
    }

    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn output(mut self, value: impl Into<String>) -> Self {
        self.output = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RecordRunRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](RecordRunRequestBuilder::input)
    /// - [`output`](RecordRunRequestBuilder::output)
    pub fn build(self) -> Result<RecordRunRequest, BuildError> {
        Ok(RecordRunRequest {
            account_id: self.account_id,
            agent_name: self.agent_name,
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            output: self.output.ok_or_else(|| BuildError::missing_field("output"))?,
        })
    }
}

