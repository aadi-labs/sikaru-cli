pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompleteAuthorization {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<HashMap<String, String>>,
    #[serde(default)]
    pub state: String,
}

impl CompleteAuthorization {
    pub fn builder() -> CompleteAuthorizationBuilder {
        <CompleteAuthorizationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompleteAuthorizationBuilder {
    query: Option<HashMap<String, String>>,
    state: Option<String>,
}

impl CompleteAuthorizationBuilder {
    pub fn query(mut self, value: HashMap<String, String>) -> Self {
        self.query = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompleteAuthorization`].
    /// This method will fail if any of the following fields are not set:
    /// - [`state`](CompleteAuthorizationBuilder::state)
    pub fn build(self) -> Result<CompleteAuthorization, BuildError> {
        Ok(CompleteAuthorization {
            query: self.query,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}

