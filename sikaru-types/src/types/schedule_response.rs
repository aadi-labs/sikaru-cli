pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScheduleResponse {
    pub schedule: ScheduleRecord,
}

impl ScheduleResponse {
    pub fn builder() -> ScheduleResponseBuilder {
        <ScheduleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleResponseBuilder {
    schedule: Option<ScheduleRecord>,
}

impl ScheduleResponseBuilder {
    pub fn schedule(mut self, value: ScheduleRecord) -> Self {
        self.schedule = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedule`](ScheduleResponseBuilder::schedule)
    pub fn build(self) -> Result<ScheduleResponse, BuildError> {
        Ok(ScheduleResponse {
            schedule: self.schedule.ok_or_else(|| BuildError::missing_field("schedule"))?,
        })
    }
}
