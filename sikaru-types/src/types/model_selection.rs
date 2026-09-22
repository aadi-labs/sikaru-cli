pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModelSelection {
    #[serde(rename = "selectedModelId")]
    #[serde(default)]
    pub selected_model_id: String,
}

impl ModelSelection {
    pub fn builder() -> ModelSelectionBuilder {
        <ModelSelectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelSelectionBuilder {
    selected_model_id: Option<String>,
}

impl ModelSelectionBuilder {
    pub fn selected_model_id(mut self, value: impl Into<String>) -> Self {
        self.selected_model_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ModelSelection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`selected_model_id`](ModelSelectionBuilder::selected_model_id)
    pub fn build(self) -> Result<ModelSelection, BuildError> {
        Ok(ModelSelection {
            selected_model_id: self.selected_model_id.ok_or_else(|| BuildError::missing_field("selected_model_id"))?,
        })
    }
}

