pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_schedules
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListSchedulesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
}

impl ListSchedulesQueryRequest {
    pub fn builder() -> ListSchedulesQueryRequestBuilder {
        <ListSchedulesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListSchedulesQueryRequestBuilder {
    session_id: Option<String>,
}

impl ListSchedulesQueryRequestBuilder {
    pub fn session_id(mut self, value: impl Into<String>) -> Self {
        self.session_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ListSchedulesQueryRequest`].
    pub fn build(self) -> Result<ListSchedulesQueryRequest, BuildError> {
        Ok(ListSchedulesQueryRequest {
            session_id: self.session_id,
        })
    }
}

