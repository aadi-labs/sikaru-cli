pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreationResume {
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
}

impl CreationResume {
    pub fn builder() -> CreationResumeBuilder {
        <CreationResumeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreationResumeBuilder {
    payload: Option<HashMap<String, serde_json::Value>>,
}

impl CreationResumeBuilder {
    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreationResume`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payload`](CreationResumeBuilder::payload)
    pub fn build(self) -> Result<CreationResume, BuildError> {
        Ok(CreationResume {
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
        })
    }
}
