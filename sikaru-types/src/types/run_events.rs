pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunEvents {
    #[serde(default)]
    pub events: Vec<RunEvent>,
    #[serde(rename = "nextAfter")]
    #[serde(default)]
    pub next_after: i64,
}

impl RunEvents {
    pub fn builder() -> RunEventsBuilder {
        <RunEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunEventsBuilder {
    events: Option<Vec<RunEvent>>,
    next_after: Option<i64>,
}

impl RunEventsBuilder {
    pub fn events(mut self, value: Vec<RunEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn next_after(mut self, value: i64) -> Self {
        self.next_after = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunEvents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RunEventsBuilder::events)
    /// - [`next_after`](RunEventsBuilder::next_after)
    pub fn build(self) -> Result<RunEvents, BuildError> {
        Ok(RunEvents {
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            next_after: self.next_after.ok_or_else(|| BuildError::missing_field("next_after"))?,
        })
    }
}
