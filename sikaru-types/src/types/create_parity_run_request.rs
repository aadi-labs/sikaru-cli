pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateParityRunRequest {
    #[serde(rename = "compatibilityProfileId")]
    #[serde(default)]
    pub compatibility_profile_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub samples: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl CreateParityRunRequest {
    pub fn builder() -> CreateParityRunRequestBuilder {
        <CreateParityRunRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateParityRunRequestBuilder {
    compatibility_profile_id: Option<String>,
    samples: Option<Vec<HashMap<String, serde_json::Value>>>,
}

impl CreateParityRunRequestBuilder {
    pub fn compatibility_profile_id(mut self, value: impl Into<String>) -> Self {
        self.compatibility_profile_id = Some(value.into());
        self
    }

    pub fn samples(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.samples = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateParityRunRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`compatibility_profile_id`](CreateParityRunRequestBuilder::compatibility_profile_id)
    pub fn build(self) -> Result<CreateParityRunRequest, BuildError> {
        Ok(CreateParityRunRequest {
            compatibility_profile_id: self.compatibility_profile_id.ok_or_else(|| BuildError::missing_field("compatibility_profile_id"))?,
            samples: self.samples,
        })
    }
}
