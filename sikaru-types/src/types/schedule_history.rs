pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleHistory {
    #[serde(default)]
    pub items: Vec<ScheduleOccurrence>,
    #[serde(rename = "nextBefore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub next_before: Option<f64>,
}

impl ScheduleHistory {
    pub fn builder() -> ScheduleHistoryBuilder {
        <ScheduleHistoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleHistoryBuilder {
    items: Option<Vec<ScheduleOccurrence>>,
    next_before: Option<f64>,
}

impl ScheduleHistoryBuilder {
    pub fn items(mut self, value: Vec<ScheduleOccurrence>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn next_before(mut self, value: f64) -> Self {
        self.next_before = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleHistory`].
    /// This method will fail if any of the following fields are not set:
    /// - [`items`](ScheduleHistoryBuilder::items)
    pub fn build(self) -> Result<ScheduleHistory, BuildError> {
        Ok(ScheduleHistory {
            items: self.items.ok_or_else(|| BuildError::missing_field("items"))?,
            next_before: self.next_before,
        })
    }
}
