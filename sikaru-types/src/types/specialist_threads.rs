pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpecialistThreads {
    #[serde(default)]
    pub items: Vec<SpecialistThread>,
}

impl SpecialistThreads {
    pub fn builder() -> SpecialistThreadsBuilder {
        <SpecialistThreadsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpecialistThreadsBuilder {
    items: Option<Vec<SpecialistThread>>,
}

impl SpecialistThreadsBuilder {
    pub fn items(mut self, value: Vec<SpecialistThread>) -> Self {
        self.items = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SpecialistThreads`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](SpecialistThreadsBuilder::items)
    pub fn build(self) -> Result<SpecialistThreads, BuildError> {
        Ok(SpecialistThreads {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
        })
    }
}
