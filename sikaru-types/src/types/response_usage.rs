pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public Response fields at this level (RESPONSE_USAGE_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponseUsage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_tokens_details: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_tokens_details: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tokens: Option<i64>,
}

impl ResponseUsage {
    pub fn builder() -> ResponseUsageBuilder {
        <ResponseUsageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseUsageBuilder {
    input_tokens: Option<i64>,
    input_tokens_details: Option<HashMap<String, serde_json::Value>>,
    output_tokens: Option<i64>,
    output_tokens_details: Option<HashMap<String, serde_json::Value>>,
    total_tokens: Option<i64>,
}

impl ResponseUsageBuilder {
    pub fn input_tokens(mut self, value: i64) -> Self {
        self.input_tokens = Some(value);
        self
    }

    pub fn input_tokens_details(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_tokens_details = Some(value);
        self
    }

    pub fn output_tokens(mut self, value: i64) -> Self {
        self.output_tokens = Some(value);
        self
    }

    pub fn output_tokens_details(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_tokens_details = Some(value);
        self
    }

    pub fn total_tokens(mut self, value: i64) -> Self {
        self.total_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResponseUsage`].
    pub fn build(self) -> Result<ResponseUsage, BuildError> {
        Ok(ResponseUsage {
            input_tokens: self.input_tokens,
            input_tokens_details: self.input_tokens_details,
            output_tokens: self.output_tokens,
            output_tokens_details: self.output_tokens_details,
            total_tokens: self.total_tokens,
        })
    }
}
