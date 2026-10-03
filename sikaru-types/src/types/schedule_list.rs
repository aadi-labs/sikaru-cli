pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleList {
    #[serde(default)]
    pub schedules: Vec<ScheduleRecord>,
}

impl ScheduleList {
    pub fn builder() -> ScheduleListBuilder {
        <ScheduleListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleListBuilder {
    schedules: Option<Vec<ScheduleRecord>>,
}

impl ScheduleListBuilder {
    pub fn schedules(mut self, value: Vec<ScheduleRecord>) -> Self {
        self.schedules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleList`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schedules`](ScheduleListBuilder::schedules)
    pub fn build(self) -> Result<ScheduleList, BuildError> {
        Ok(ScheduleList {
            schedules: self.schedules.ok_or_else(|| BuildError::missing_field("schedules"))?,
        })
    }
}
