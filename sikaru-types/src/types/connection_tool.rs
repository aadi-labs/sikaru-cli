pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ConnectionTool {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(default)]
    pub digest: String,
    #[serde(default)]
    pub input_schema: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl ConnectionTool {
    pub fn builder() -> ConnectionToolBuilder {
        <ConnectionToolBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConnectionToolBuilder {
    description: Option<String>,
    digest: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    name: Option<String>,
    version: Option<String>,
}

impl ConnectionToolBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn digest(mut self, value: impl Into<String>) -> Self {
        self.digest = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn version(mut self, value: impl Into<String>) -> Self {
        self.version = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ConnectionTool`].
    /// This method will fail if any of the following fields are not set:
    /// - [`digest`](ConnectionToolBuilder::digest)
    /// - [`input_schema`](ConnectionToolBuilder::input_schema)
    /// - [`name`](ConnectionToolBuilder::name)
    pub fn build(self) -> Result<ConnectionTool, BuildError> {
        Ok(ConnectionTool {
            description: self.description,
            digest: self.digest.ok_or_else(|| BuildError::missing_field("digest"))?,
            input_schema: self.input_schema.ok_or_else(|| BuildError::missing_field("input_schema"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            version: self.version,
        })
    }
}
