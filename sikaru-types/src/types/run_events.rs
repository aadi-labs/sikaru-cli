pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunEvents {
    #[serde(rename = "contentVisible")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_visible: Option<bool>,
    #[serde(default)]
    pub events: Vec<RunEvent>,
    #[serde(rename = "nextAfter")]
    #[serde(default)]
    pub next_after: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal: Option<bool>,
}

impl RunEvents {
    pub fn builder() -> RunEventsBuilder {
        <RunEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunEventsBuilder {
    content_visible: Option<bool>,
    events: Option<Vec<RunEvent>>,
    next_after: Option<i64>,
    personal: Option<bool>,
}

impl RunEventsBuilder {
    pub fn content_visible(mut self, value: bool) -> Self {
        self.content_visible = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<RunEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn next_after(mut self, value: i64) -> Self {
        self.next_after = Some(value);
        self
    }

    pub fn personal(mut self, value: bool) -> Self {
        self.personal = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunEvents`].
    /// This method will fail if any of the following fields are not set:
    /// - [`events`](RunEventsBuilder::events)
    /// - [`next_after`](RunEventsBuilder::next_after)
    pub fn build(self) -> Result<RunEvents, BuildError> {
        Ok(RunEvents {
            content_visible: self.content_visible,
            events: self.events.ok_or_else(|| BuildError::missing_field("events"))?,
            next_after: self.next_after.ok_or_else(|| BuildError::missing_field("next_after"))?,
            personal: self.personal,
        })
    }
}
