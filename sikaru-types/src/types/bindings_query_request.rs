pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for bindings
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BindingsQueryRequest {
    #[serde(default)]
    pub agent_id: String,
}

impl BindingsQueryRequest {
    pub fn builder() -> BindingsQueryRequestBuilder {
        <BindingsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BindingsQueryRequestBuilder {
    agent_id: Option<String>,
}

impl BindingsQueryRequestBuilder {
    pub fn agent_id(mut self, value: impl Into<String>) -> Self {
        self.agent_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BindingsQueryRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`agent_id`](BindingsQueryRequestBuilder::agent_id)
    pub fn build(self) -> Result<BindingsQueryRequest, BuildError> {
        Ok(BindingsQueryRequest {
            agent_id: self.agent_id.ok_or_else(|| BuildError::missing_field("agent_id"))?,
        })
    }
}

