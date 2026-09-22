pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for schedule_history
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ScheduleHistoryQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub before: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl ScheduleHistoryQueryRequest {
    pub fn builder() -> ScheduleHistoryQueryRequestBuilder {
        <ScheduleHistoryQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleHistoryQueryRequestBuilder {
    before: Option<f64>,
    limit: Option<i64>,
}

impl ScheduleHistoryQueryRequestBuilder {
    pub fn before(mut self, value: f64) -> Self {
        self.before = Some(value);
        self
    }

    pub fn limit(mut self, value: i64) -> Self {
        self.limit = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ScheduleHistoryQueryRequest`].
    pub fn build(self) -> Result<ScheduleHistoryQueryRequest, BuildError> {
        Ok(ScheduleHistoryQueryRequest {
            before: self.before,
            limit: self.limit,
        })
    }
}

