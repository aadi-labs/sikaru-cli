pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleNotices {
    #[serde(default)]
    pub notices: Vec<ScheduleNotice>,
}

impl ScheduleNotices {
    pub fn builder() -> ScheduleNoticesBuilder {
        <ScheduleNoticesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleNoticesBuilder {
    notices: Option<Vec<ScheduleNotice>>,
}

impl ScheduleNoticesBuilder {
    pub fn notices(mut self, value: Vec<ScheduleNotice>) -> Self {
        self.notices = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleNotices`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notices`](ScheduleNoticesBuilder::notices)
    pub fn build(self) -> Result<ScheduleNotices, BuildError> {
        Ok(ScheduleNotices {
            notices: self.notices.ok_or_else(|| BuildError::missing_field("notices"))?,
        })
    }
}
