pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// The customer-authored `sikaru.agent.contract.v1` agent definition.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgentDefinition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub budget: Option<AgentDocumentBudget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcomes: Option<Vec<String>>,
    pub schema: AgentDefinitionSchema,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup: Option<AgentSetup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<Vec<AgentDefinitionSource>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<AgentToolCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web: Option<AgentWebCapabilities>,
}

impl AgentDefinition {
    pub fn builder() -> AgentDefinitionBuilder {
        <AgentDefinitionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDefinitionBuilder {
    access: Option<HashMap<String, serde_json::Value>>,
    budget: Option<AgentDocumentBudget>,
    instructions: Option<String>,
    model: Option<String>,
    outcomes: Option<Vec<String>>,
    schema: Option<AgentDefinitionSchema>,
    setup: Option<AgentSetup>,
    sources: Option<Vec<AgentDefinitionSource>>,
    tools: Option<AgentToolCapabilities>,
    web: Option<AgentWebCapabilities>,
}

impl AgentDefinitionBuilder {
    pub fn access(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.access = Some(value);
        self
    }

    pub fn budget(mut self, value: AgentDocumentBudget) -> Self {
        self.budget = Some(value);
        self
    }

    pub fn instructions(mut self, value: impl Into<String>) -> Self {
        self.instructions = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn outcomes(mut self, value: Vec<String>) -> Self {
        self.outcomes = Some(value);
        self
    }

    pub fn schema(mut self, value: AgentDefinitionSchema) -> Self {
        self.schema = Some(value);
        self
    }

    pub fn setup(mut self, value: AgentSetup) -> Self {
        self.setup = Some(value);
        self
    }

    pub fn sources(mut self, value: Vec<AgentDefinitionSource>) -> Self {
        self.sources = Some(value);
        self
    }

    pub fn tools(mut self, value: AgentToolCapabilities) -> Self {
        self.tools = Some(value);
        self
    }

    pub fn web(mut self, value: AgentWebCapabilities) -> Self {
        self.web = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDefinition`].
    /// This method will fail if any of the following fields are not set:
    /// - [`schema`](AgentDefinitionBuilder::schema)
    pub fn build(self) -> Result<AgentDefinition, BuildError> {
        Ok(AgentDefinition {
            access: self.access,
            budget: self.budget,
            instructions: self.instructions,
            model: self.model,
            outcomes: self.outcomes,
            schema: self.schema.ok_or_else(|| BuildError::missing_field("schema"))?,
            setup: self.setup,
            sources: self.sources,
            tools: self.tools,
            web: self.web,
        })
    }
}
