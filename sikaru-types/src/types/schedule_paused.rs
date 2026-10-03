pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SchedulePaused {
    #[serde(default)]
    pub paused: bool,
}

impl SchedulePaused {
    pub fn builder() -> SchedulePausedBuilder {
        <SchedulePausedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SchedulePausedBuilder {
    paused: Option<bool>,
}

impl SchedulePausedBuilder {
    pub fn paused(mut self, value: bool) -> Self {
        self.paused = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SchedulePaused`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paused`](SchedulePausedBuilder::paused)
    pub fn build(self) -> Result<SchedulePaused, BuildError> {
        Ok(SchedulePaused {
            paused: self.paused.ok_or_else(|| BuildError::missing_field("paused"))?,
        })
    }
}
