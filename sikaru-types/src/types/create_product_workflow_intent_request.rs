pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateProductWorkflowIntentRequest {
    #[serde(default)]
    pub description: String,
}

impl CreateProductWorkflowIntentRequest {
    pub fn builder() -> CreateProductWorkflowIntentRequestBuilder {
        <CreateProductWorkflowIntentRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateProductWorkflowIntentRequestBuilder {
    description: Option<String>,
}

impl CreateProductWorkflowIntentRequestBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateProductWorkflowIntentRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`description`](CreateProductWorkflowIntentRequestBuilder::description)
    pub fn build(self) -> Result<CreateProductWorkflowIntentRequest, BuildError> {
        Ok(CreateProductWorkflowIntentRequest {
            description: self.description.ok_or_else(|| BuildError::missing_field("description"))?,
        })
    }
}

