pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public CreateChatCompletionResponse fields at this level (CHAT_USAGE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CompletionUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completion_tokens_details: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_tokens_details: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

impl CompletionUsage {
    pub fn builder() -> CompletionUsageBuilder {
        <CompletionUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CompletionUsageBuilder {
    completion_tokens: Option<i64>,
    completion_tokens_details: Option<HashMap<String, serde_json::Value>>,
    prompt_tokens: Option<i64>,
    prompt_tokens_details: Option<HashMap<String, serde_json::Value>>,
    total_tokens: Option<i64>,
}

impl CompletionUsageBuilder {
    pub fn completion_tokens(mut self, value: i64) -> Self {
        self.completion_tokens = Some(value);
        self
    }

    pub fn completion_tokens_details(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.completion_tokens_details = Some(value);
        self
    }

    pub fn prompt_tokens(mut self, value: i64) -> Self {
        self.prompt_tokens = Some(value);
        self
    }

    pub fn prompt_tokens_details(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.prompt_tokens_details = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CompletionUsage`].
    pub fn build(self) -> Result<CompletionUsage, BuildError> {
        Ok(CompletionUsage {
            completion_tokens: self.completion_tokens,
            completion_tokens_details: self.completion_tokens_details,
            prompt_tokens: self.prompt_tokens,
            prompt_tokens_details: self.prompt_tokens_details,
            total_tokens: self.total_tokens,
        })
    }
}
