pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResendDelivery {
    #[serde(default)]
    pub acknowledge_possible_duplicate: bool,
}

impl ResendDelivery {
    pub fn builder() -> ResendDeliveryBuilder {
        <ResendDeliveryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResendDeliveryBuilder {
    acknowledge_possible_duplicate: Option<bool>,
}

impl ResendDeliveryBuilder {
    pub fn acknowledge_possible_duplicate(mut self, value: bool) -> Self {
        self.acknowledge_possible_duplicate = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ResendDelivery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`acknowledge_possible_duplicate`](ResendDeliveryBuilder::acknowledge_possible_duplicate)
    pub fn build(self) -> Result<ResendDelivery, BuildError> {
        Ok(ResendDelivery {
            acknowledge_possible_duplicate: self.acknowledge_possible_duplicate.ok_or_else(|| BuildError::missing_field("acknowledge_possible_duplicate"))?,
        })
    }
}

