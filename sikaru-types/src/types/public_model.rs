pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// One model in the public catalog.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PublicModel {
    #[serde(default)]
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecation_date: Option<NaiveDate>,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub id: String,
    pub object: PublicModelObject,
    #[serde(default)]
    pub owned_by: String,
    #[serde(default)]
    pub pricing: ModelPricing,
    #[serde(default)]
    pub specs: ModelSpecs,
}

impl PublicModel {
    pub fn builder() -> PublicModelBuilder {
        <PublicModelBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PublicModelBuilder {
    created: Option<i64>,
    deprecation_date: Option<NaiveDate>,
    display_name: Option<String>,
    id: Option<String>,
    object: Option<PublicModelObject>,
    owned_by: Option<String>,
    pricing: Option<ModelPricing>,
    specs: Option<ModelSpecs>,
}

impl PublicModelBuilder {
    pub fn created(mut self, value: i64) -> Self {
        self.created = Some(value);
        self
    }

    pub fn deprecation_date(mut self, value: NaiveDate) -> Self {
        self.deprecation_date = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn object(mut self, value: PublicModelObject) -> Self {
        self.object = Some(value);
        self
    }

    pub fn owned_by(mut self, value: impl Into<String>) -> Self {
        self.owned_by = Some(value.into());
        self
    }

    pub fn pricing(mut self, value: ModelPricing) -> Self {
        self.pricing = Some(value);
        self
    }

    pub fn specs(mut self, value: ModelSpecs) -> Self {
        self.specs = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PublicModel`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created`](PublicModelBuilder::created)
    /// - [`display_name`](PublicModelBuilder::display_name)
    /// - [`id`](PublicModelBuilder::id)
    /// - [`object`](PublicModelBuilder::object)
    /// - [`owned_by`](PublicModelBuilder::owned_by)
    /// - [`pricing`](PublicModelBuilder::pricing)
    /// - [`specs`](PublicModelBuilder::specs)
    pub fn build(self) -> Result<PublicModel, BuildError> {
        Ok(PublicModel {
            created: self.created.ok_or_else(|| BuildError::missing_field("created"))?,
            deprecation_date: self.deprecation_date,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            object: self.object.ok_or_else(|| BuildError::missing_field("object"))?,
            owned_by: self.owned_by.ok_or_else(|| BuildError::missing_field("owned_by"))?,
            pricing: self.pricing.ok_or_else(|| BuildError::missing_field("pricing"))?,
            specs: self.specs.ok_or_else(|| BuildError::missing_field("specs"))?,
        })
    }
}
