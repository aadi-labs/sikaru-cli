pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PauseInput {
    #[serde(default)]
    pub paused: bool,
}

impl PauseInput {
    pub fn builder() -> PauseInputBuilder {
        <PauseInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PauseInputBuilder {
    paused: Option<bool>,
}

impl PauseInputBuilder {
    pub fn paused(mut self, value: bool) -> Self {
        self.paused = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PauseInput`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paused`](PauseInputBuilder::paused)
    pub fn build(self) -> Result<PauseInput, BuildError> {
        Ok(PauseInput {
            paused: self.paused.ok_or_else(|| BuildError::missing_field("paused"))?,
        })
    }
}

