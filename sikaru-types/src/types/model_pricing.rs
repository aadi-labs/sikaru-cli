pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// US dollars per million tokens.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ModelPricing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_read_usd_per_mtok: Option<f64>,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub input_usd_per_mtok: f64,
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub output_usd_per_mtok: f64,
}

impl ModelPricing {
    pub fn builder() -> ModelPricingBuilder {
        <ModelPricingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelPricingBuilder {
    cache_read_usd_per_mtok: Option<f64>,
    input_usd_per_mtok: Option<f64>,
    output_usd_per_mtok: Option<f64>,
}

impl ModelPricingBuilder {
    pub fn cache_read_usd_per_mtok(mut self, value: f64) -> Self {
        self.cache_read_usd_per_mtok = Some(value);
        self
    }

    pub fn input_usd_per_mtok(mut self, value: f64) -> Self {
        self.input_usd_per_mtok = Some(value);
        self
    }

    pub fn output_usd_per_mtok(mut self, value: f64) -> Self {
        self.output_usd_per_mtok = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelPricing`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input_usd_per_mtok`](ModelPricingBuilder::input_usd_per_mtok)
    /// - [`output_usd_per_mtok`](ModelPricingBuilder::output_usd_per_mtok)
    pub fn build(self) -> Result<ModelPricing, BuildError> {
        Ok(ModelPricing {
            cache_read_usd_per_mtok: self.cache_read_usd_per_mtok,
            input_usd_per_mtok: self.input_usd_per_mtok.ok_or_else(|| BuildError::missing_field("input_usd_per_mtok"))?,
            output_usd_per_mtok: self.output_usd_per_mtok.ok_or_else(|| BuildError::missing_field("output_usd_per_mtok"))?,
        })
    }
}
