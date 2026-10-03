pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentDocumentDraft {
    #[serde(default)]
    pub checks: Vec<StarterCheck>,
    #[serde(default)]
    pub document: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub notes: Vec<String>,
    /// Toolkit slugs the agent expects, connected or not.
    #[serde(default)]
    pub tools: Vec<String>,
}

impl AgentDocumentDraft {
    pub fn builder() -> AgentDocumentDraftBuilder {
        <AgentDocumentDraftBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDocumentDraftBuilder {
    checks: Option<Vec<StarterCheck>>,
    document: Option<String>,
    name: Option<String>,
    notes: Option<Vec<String>>,
    tools: Option<Vec<String>>,
}

impl AgentDocumentDraftBuilder {
    pub fn checks(mut self, value: Vec<StarterCheck>) -> Self {
        self.checks = Some(value);
        self
    }

    pub fn document(mut self, value: impl Into<String>) -> Self {
        self.document = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: Vec<String>) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn tools(mut self, value: Vec<String>) -> Self {
        self.tools = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDocumentDraft`].
    /// This method will fail if any of the following fields are not set:
    /// - [`checks`](AgentDocumentDraftBuilder::checks)
    /// - [`document`](AgentDocumentDraftBuilder::document)
    /// - [`name`](AgentDocumentDraftBuilder::name)
    /// - [`notes`](AgentDocumentDraftBuilder::notes)
    /// - [`tools`](AgentDocumentDraftBuilder::tools)
    pub fn build(self) -> Result<AgentDocumentDraft, BuildError> {
        Ok(AgentDocumentDraft {
            checks: self.checks.ok_or_else(|| BuildError::missing_field("checks"))?,
            document: self.document.ok_or_else(|| BuildError::missing_field("document"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            notes: self.notes.ok_or_else(|| BuildError::missing_field("notes"))?,
            tools: self.tools.ok_or_else(|| BuildError::missing_field("tools"))?,
        })
    }
}
