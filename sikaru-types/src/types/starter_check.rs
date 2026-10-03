pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct StarterCheck {
    /// A realistic user message to test the agent with.
    #[serde(default)]
    pub instruction: String,
    #[serde(default)]
    pub name: String,
    /// What a good answer must do to pass.
    #[serde(default)]
    pub rubric: String,
}

impl StarterCheck {
    pub fn builder() -> StarterCheckBuilder {
        <StarterCheckBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StarterCheckBuilder {
    instruction: Option<String>,
    name: Option<String>,
    rubric: Option<String>,
}

impl StarterCheckBuilder {
    pub fn instruction(mut self, value: impl Into<String>) -> Self {
        self.instruction = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn rubric(mut self, value: impl Into<String>) -> Self {
        self.rubric = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StarterCheck`].
    /// This method will fail if any of the following fields are not set:
    /// - [`instruction`](StarterCheckBuilder::instruction)
    /// - [`name`](StarterCheckBuilder::name)
    /// - [`rubric`](StarterCheckBuilder::rubric)
    pub fn build(self) -> Result<StarterCheck, BuildError> {
        Ok(StarterCheck {
            instruction: self.instruction.ok_or_else(|| BuildError::missing_field("instruction"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            rubric: self.rubric.ok_or_else(|| BuildError::missing_field("rubric"))?,
        })
    }
}
