pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for list_eligible_releases
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListEligibleReleasesQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<ListEligibleReleasesExecutionSessionsRequestEnvironment>,
}

impl ListEligibleReleasesQueryRequest {
    pub fn builder() -> ListEligibleReleasesQueryRequestBuilder {
        <ListEligibleReleasesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListEligibleReleasesQueryRequestBuilder {
    environment: Option<ListEligibleReleasesExecutionSessionsRequestEnvironment>,
}

impl ListEligibleReleasesQueryRequestBuilder {
    pub fn environment(mut self, value: ListEligibleReleasesExecutionSessionsRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListEligibleReleasesQueryRequest`].
    pub fn build(self) -> Result<ListEligibleReleasesQueryRequest, BuildError> {
        Ok(ListEligibleReleasesQueryRequest {
            environment: self.environment,
        })
    }
}

