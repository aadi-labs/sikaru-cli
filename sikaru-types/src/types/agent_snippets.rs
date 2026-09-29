pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentSnippets {
    #[serde(default)]
    pub cli: String,
    #[serde(default)]
    pub curl: String,
    #[serde(default)]
    pub python: String,
    #[serde(default)]
    pub typescript: String,
}

impl AgentSnippets {
    pub fn builder() -> AgentSnippetsBuilder {
        <AgentSnippetsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentSnippetsBuilder {
    cli: Option<String>,
    curl: Option<String>,
    python: Option<String>,
    typescript: Option<String>,
}

impl AgentSnippetsBuilder {
    pub fn cli(mut self, value: impl Into<String>) -> Self {
        self.cli = Some(value.into());
        self
    }

    pub fn curl(mut self, value: impl Into<String>) -> Self {
        self.curl = Some(value.into());
        self
    }

    pub fn python(mut self, value: impl Into<String>) -> Self {
        self.python = Some(value.into());
        self
    }

    pub fn typescript(mut self, value: impl Into<String>) -> Self {
        self.typescript = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AgentSnippets`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cli`](AgentSnippetsBuilder::cli)
    /// - [`curl`](AgentSnippetsBuilder::curl)
    /// - [`python`](AgentSnippetsBuilder::python)
    /// - [`typescript`](AgentSnippetsBuilder::typescript)
    pub fn build(self) -> Result<AgentSnippets, BuildError> {
        Ok(AgentSnippets {
            cli: self.cli.ok_or_else(|| BuildError::missing_field("cli"))?,
            curl: self.curl.ok_or_else(|| BuildError::missing_field("curl"))?,
            python: self.python.ok_or_else(|| BuildError::missing_field("python"))?,
            typescript: self.typescript.ok_or_else(|| BuildError::missing_field("typescript"))?,
        })
    }
}
