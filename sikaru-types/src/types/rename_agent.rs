pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenameAgent {
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
}

impl RenameAgent {
    pub fn builder() -> RenameAgentBuilder {
        <RenameAgentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenameAgentBuilder {
    display_name: Option<String>,
}

impl RenameAgentBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RenameAgent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](RenameAgentBuilder::display_name)
    pub fn build(self) -> Result<RenameAgent, BuildError> {
        Ok(RenameAgent {
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
        })
    }
}

