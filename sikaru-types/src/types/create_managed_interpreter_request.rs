pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateManagedInterpreterRequest {
    #[serde(default)]
    pub language: String,
    #[serde(rename = "resourceLimits")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_limits: Option<HashMap<String, serde_json::Value>>,
}

impl CreateManagedInterpreterRequest {
    pub fn builder() -> CreateManagedInterpreterRequestBuilder {
        <CreateManagedInterpreterRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateManagedInterpreterRequestBuilder {
    language: Option<String>,
    resource_limits: Option<HashMap<String, serde_json::Value>>,
}

impl CreateManagedInterpreterRequestBuilder {
    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn resource_limits(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.resource_limits = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateManagedInterpreterRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`language`](CreateManagedInterpreterRequestBuilder::language)
    pub fn build(self) -> Result<CreateManagedInterpreterRequest, BuildError> {
        Ok(CreateManagedInterpreterRequest {
            language: self.language.ok_or_else(|| BuildError::missing_field("language"))?,
            resource_limits: self.resource_limits,
        })
    }
}

