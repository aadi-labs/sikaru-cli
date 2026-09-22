pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct LiveHandle {
    #[serde(default)]
    pub handle_id: String,
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub tool_call_id: String,
}

impl LiveHandle {
    pub fn builder() -> LiveHandleBuilder {
        <LiveHandleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LiveHandleBuilder {
    handle_id: Option<String>,
    run_id: Option<String>,
    tool_call_id: Option<String>,
}

impl LiveHandleBuilder {
    pub fn handle_id(mut self, value: impl Into<String>) -> Self {
        self.handle_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn tool_call_id(mut self, value: impl Into<String>) -> Self {
        self.tool_call_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`LiveHandle`].
    /// This method will fail if any of the following fields are not set:
    /// - [`handle_id`](LiveHandleBuilder::handle_id)
    /// - [`run_id`](LiveHandleBuilder::run_id)
    /// - [`tool_call_id`](LiveHandleBuilder::tool_call_id)
    pub fn build(self) -> Result<LiveHandle, BuildError> {
        Ok(LiveHandle {
            handle_id: self.handle_id.ok_or_else(|| BuildError::missing_field("handle_id"))?,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            tool_call_id: self.tool_call_id.ok_or_else(|| BuildError::missing_field("tool_call_id"))?,
        })
    }
}
