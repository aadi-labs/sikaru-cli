pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateCompletionResponse fields at this level (COMPLETION_RESPONSE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct Completion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub choices: Option<Vec<CompletionChoice>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(default)]
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default)]
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<CompletionUsage>,
}

impl Completion {
    pub fn builder() -> CompletionBuilder {
        <CompletionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionBuilder {
    choices: Option<Vec<CompletionChoice>>,
    created: Option<i64>,
    id: Option<String>,
    model: Option<String>,
    object: Option<String>,
    usage: Option<CompletionUsage>,
}

impl CompletionBuilder {
    pub fn choices(mut self, value: Vec<CompletionChoice>) -> Self {
        self.choices = Some(value);
        self
    }

    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn object(mut self, value: impl Into<String>) -> Self {
        self.object = Some(value.into());
        self
    }

    pub fn usage(mut self, value: CompletionUsage) -> Self {
        self.usage = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Completion`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](CompletionBuilder::id)
    /// - [`object`](CompletionBuilder::object)
    pub fn build(self) -> Result<Completion, BuildError> {
        Ok(Completion {
            choices: self.choices,
            created: self.created,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            model: self.model,
            object: self.object.ok_or_else(|| BuildError::missing_field("object"))?,
            usage: self.usage,
        })
    }
}
