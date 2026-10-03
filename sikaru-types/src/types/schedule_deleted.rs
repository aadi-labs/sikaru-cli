pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleDeleted {
    #[serde(default)]
    pub deleted: bool,
}

impl ScheduleDeleted {
    pub fn builder() -> ScheduleDeletedBuilder {
        <ScheduleDeletedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleDeletedBuilder {
    deleted: Option<bool>,
}

impl ScheduleDeletedBuilder {
    pub fn deleted(mut self, value: bool) -> Self {
        self.deleted = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleDeleted`].
    /// This method will fail if any of the following fields are not set:
    /// - [`deleted`](ScheduleDeletedBuilder::deleted)
    pub fn build(self) -> Result<ScheduleDeleted, BuildError> {
        Ok(ScheduleDeleted {
            deleted: self.deleted.ok_or_else(|| BuildError::missing_field("deleted"))?,
        })
    }
}
