pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A model's reasoning declaration plus what each standard effort value runs at.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PublishedReasoning {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_effort: Option<PublishedReasoningDefaultEffort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_history: Option<PublishedReasoningDefaultHistory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub efforts: Option<Vec<PublishedReasoningEffortsItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<PublishedReasoningHistoryItem>>,
    #[serde(default)]
    pub normalization: HashMap<String, Option<PublishedReasoningNormalizationValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<bool>,
}

impl PublishedReasoning {
    pub fn builder() -> PublishedReasoningBuilder {
        <PublishedReasoningBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublishedReasoningBuilder {
    budget: Option<bool>,
    default_effort: Option<PublishedReasoningDefaultEffort>,
    default_history: Option<PublishedReasoningDefaultHistory>,
    efforts: Option<Vec<PublishedReasoningEffortsItem>>,
    history: Option<Vec<PublishedReasoningHistoryItem>>,
    normalization: Option<HashMap<String, Option<PublishedReasoningNormalizationValue>>>,
    summary: Option<bool>,
}

impl PublishedReasoningBuilder {
    pub fn budget(mut self, value: bool) -> Self {
        self.budget = Some(value);
        self
    }

    pub fn default_effort(mut self, value: PublishedReasoningDefaultEffort) -> Self {
        self.default_effort = Some(value);
        self
    }

    pub fn default_history(mut self, value: PublishedReasoningDefaultHistory) -> Self {
        self.default_history = Some(value);
        self
    }

    pub fn efforts(mut self, value: Vec<PublishedReasoningEffortsItem>) -> Self {
        self.efforts = Some(value);
        self
    }

    pub fn history(mut self, value: Vec<PublishedReasoningHistoryItem>) -> Self {
        self.history = Some(value);
        self
    }

    pub fn normalization(mut self, value: HashMap<String, Option<PublishedReasoningNormalizationValue>>) -> Self {
        self.normalization = Some(value);
        self
    }

    pub fn summary(mut self, value: bool) -> Self {
        self.summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublishedReasoning`].
    /// This method will fail if any of the following fields are not set:
    /// - [`normalization`](PublishedReasoningBuilder::normalization)
    pub fn build(self) -> Result<PublishedReasoning, BuildError> {
        Ok(PublishedReasoning {
            budget: self.budget,
            default_effort: self.default_effort,
            default_history: self.default_history,
            efforts: self.efforts,
            history: self.history,
            normalization: self.normalization.ok_or_else(|| BuildError::missing_field("normalization"))?,
            summary: self.summary,
        })
    }
}
