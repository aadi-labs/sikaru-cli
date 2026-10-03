pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonalConnectionPrompt {
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub kind: String,
}

impl PersonalConnectionPrompt {
    pub fn builder() -> PersonalConnectionPromptBuilder {
        <PersonalConnectionPromptBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonalConnectionPromptBuilder {
    display_name: Option<String>,
    id: Option<String>,
    kind: Option<String>,
}

impl PersonalConnectionPromptBuilder {
    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PersonalConnectionPrompt`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](PersonalConnectionPromptBuilder::display_name)
    /// - [`id`](PersonalConnectionPromptBuilder::id)
    /// - [`kind`](PersonalConnectionPromptBuilder::kind)
    pub fn build(self) -> Result<PersonalConnectionPrompt, BuildError> {
        Ok(PersonalConnectionPrompt {
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
        })
    }
}
