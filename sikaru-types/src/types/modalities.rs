pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Modalities {
    #[serde(default)]
    pub input: Vec<String>,
    #[serde(default)]
    pub output: Vec<String>,
}

impl Modalities {
    pub fn builder() -> ModalitiesBuilder {
        <ModalitiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModalitiesBuilder {
    input: Option<Vec<String>>,
    output: Option<Vec<String>>,
}

impl ModalitiesBuilder {
    pub fn input(mut self, value: Vec<String>) -> Self {
        self.input = Some(value);
        self
    }

    pub fn output(mut self, value: Vec<String>) -> Self {
        self.output = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Modalities`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ModalitiesBuilder::input)
    /// - [`output`](ModalitiesBuilder::output)
    pub fn build(self) -> Result<Modalities, BuildError> {
        Ok(Modalities {
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            output: self.output.ok_or_else(|| BuildError::missing_field("output"))?,
        })
    }
}
