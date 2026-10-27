pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ModelList {
    #[serde(default)]
    pub data: Vec<PublicModel>,
    pub object: ModelListObject,
}

impl ModelList {
    pub fn builder() -> ModelListBuilder {
        <ModelListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModelListBuilder {
    data: Option<Vec<PublicModel>>,
    object: Option<ModelListObject>,
}

impl ModelListBuilder {
    pub fn data(mut self, value: Vec<PublicModel>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn object(mut self, value: ModelListObject) -> Self {
        self.object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModelList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](ModelListBuilder::data)
    /// - [`object`](ModelListBuilder::object)
    pub fn build(self) -> Result<ModelList, BuildError> {
        Ok(ModelList {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
            object: self.object.ok_or_else(|| BuildError::missing_field("object"))?,
        })
    }
}
