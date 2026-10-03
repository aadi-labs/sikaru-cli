pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TranscriptUsage {
    #[serde(rename = "cachedTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cached_tokens: Option<i64>,
    #[serde(rename = "inputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<i64>,
    #[serde(rename = "outputTokens")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<i64>,
}

impl TranscriptUsage {
    pub fn builder() -> TranscriptUsageBuilder {
        <TranscriptUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TranscriptUsageBuilder {
    cached_tokens: Option<i64>,
    input_tokens: Option<i64>,
    output_tokens: Option<i64>,
}

impl TranscriptUsageBuilder {
    pub fn cached_tokens(mut self, value: i64) -> Self {
        self.cached_tokens = Some(value);
        self
    }

    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TranscriptUsage`].
    pub fn build(self) -> Result<TranscriptUsage, BuildError> {
        Ok(TranscriptUsage {
            cached_tokens: self.cached_tokens,
            input_tokens: self.input_tokens,
            output_tokens: self.output_tokens,
        })
    }
}
