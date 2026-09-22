pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Query parameters for preview_policy_eligibility
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewPolicyEligibilityQueryRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment: Option<PreviewPolicyEligibilityOnlineEvaluationsRequestEnvironment>,
}

impl PreviewPolicyEligibilityQueryRequest {
    pub fn builder() -> PreviewPolicyEligibilityQueryRequestBuilder {
        <PreviewPolicyEligibilityQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewPolicyEligibilityQueryRequestBuilder {
    environment: Option<PreviewPolicyEligibilityOnlineEvaluationsRequestEnvironment>,
}

impl PreviewPolicyEligibilityQueryRequestBuilder {
    pub fn environment(mut self, value: PreviewPolicyEligibilityOnlineEvaluationsRequestEnvironment) -> Self {
        self.environment = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewPolicyEligibilityQueryRequest`].
    pub fn build(self) -> Result<PreviewPolicyEligibilityQueryRequest, BuildError> {
        Ok(PreviewPolicyEligibilityQueryRequest {
            environment: self.environment,
        })
    }
}

