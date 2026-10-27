pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateCompletionResponse fields at this level (COMPLETION_CHOICE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompletionChoice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

impl CompletionChoice {
    pub fn builder() -> CompletionChoiceBuilder {
        <CompletionChoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionChoiceBuilder {
    finish_reason: Option<String>,
    index: Option<i64>,
    logprobs: Option<HashMap<String, serde_json::Value>>,
    text: Option<String>,
}

impl CompletionChoiceBuilder {
    pub fn finish_reason(mut self, value: impl Into<String>) -> Self {
        self.finish_reason = Some(value.into());
        self
    }

    pub fn index(mut self, value: i64) -> Self {
        self.index = Some(value);
        self
    }

    pub fn logprobs(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CompletionChoice`].
    pub fn build(self) -> Result<CompletionChoice, BuildError> {
        Ok(CompletionChoice {
            finish_reason: self.finish_reason,
            index: self.index,
            logprobs: self.logprobs,
            text: self.text,
        })
    }
}
