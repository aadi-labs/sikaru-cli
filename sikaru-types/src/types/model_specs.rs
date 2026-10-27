pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// What a model accepts: limits, modalities, endpoints, reasoning controls and supported parameters.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelSpecs {
    #[serde(default)]
    pub context_window: i64,
    #[serde(default)]
    pub endpoints: Vec<ModelSpecsEndpointsItem>,
    #[serde(default)]
    pub max_output_tokens: i64,
    #[serde(default)]
    pub modalities: Modalities,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<PublishedReasoning>,
    #[serde(default)]
    pub supported_parameters: Vec<String>,
    #[serde(default)]
    pub supports_reasoning: bool,
}

impl ModelSpecs {
    pub fn builder() -> ModelSpecsBuilder {
        <ModelSpecsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelSpecsBuilder {
    context_window: Option<i64>,
    endpoints: Option<Vec<ModelSpecsEndpointsItem>>,
    max_output_tokens: Option<i64>,
    modalities: Option<Modalities>,
    reasoning: Option<PublishedReasoning>,
    supported_parameters: Option<Vec<String>>,
    supports_reasoning: Option<bool>,
}

impl ModelSpecsBuilder {
    pub fn context_window(mut self, value: i64) -> Self {
        self.context_window = Some(value);
        self
    }

    pub fn endpoints(mut self, value: Vec<ModelSpecsEndpointsItem>) -> Self {
        self.endpoints = Some(value);
        self
    }

    pub fn max_output_tokens(mut self, value: i64) -> Self {
        self.max_output_tokens = Some(value);
        self
    }

    pub fn modalities(mut self, value: Modalities) -> Self {
        self.modalities = Some(value);
        self
    }

    pub fn reasoning(mut self, value: PublishedReasoning) -> Self {
        self.reasoning = Some(value);
        self
    }

    pub fn supported_parameters(mut self, value: Vec<String>) -> Self {
        self.supported_parameters = Some(value);
        self
    }

    pub fn supports_reasoning(mut self, value: bool) -> Self {
        self.supports_reasoning = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelSpecs`].
    /// This method will fail if any of the following fields are not set:
    /// - [`context_window`](ModelSpecsBuilder::context_window)
    /// - [`endpoints`](ModelSpecsBuilder::endpoints)
    /// - [`max_output_tokens`](ModelSpecsBuilder::max_output_tokens)
    /// - [`modalities`](ModelSpecsBuilder::modalities)
    /// - [`supported_parameters`](ModelSpecsBuilder::supported_parameters)
    /// - [`supports_reasoning`](ModelSpecsBuilder::supports_reasoning)
    pub fn build(self) -> Result<ModelSpecs, BuildError> {
        Ok(ModelSpecs {
            context_window: self.context_window.ok_or_else(|| BuildError::missing_field("context_window"))?,
            endpoints: self.endpoints.ok_or_else(|| BuildError::missing_field("endpoints"))?,
            max_output_tokens: self.max_output_tokens.ok_or_else(|| BuildError::missing_field("max_output_tokens"))?,
            modalities: self.modalities.ok_or_else(|| BuildError::missing_field("modalities"))?,
            reasoning: self.reasoning,
            supported_parameters: self.supported_parameters.ok_or_else(|| BuildError::missing_field("supported_parameters"))?,
            supports_reasoning: self.supports_reasoning.ok_or_else(|| BuildError::missing_field("supports_reasoning"))?,
        })
    }
}
