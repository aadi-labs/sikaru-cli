pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AgentDocumentBudget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<i64>,
}

impl AgentDocumentBudget {
    pub fn builder() -> AgentDocumentBudgetBuilder {
        <AgentDocumentBudgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AgentDocumentBudgetBuilder {
    max_calls: Option<i64>,
    max_tokens: Option<i64>,
}

impl AgentDocumentBudgetBuilder {
    pub fn max_calls(mut self, value: i64) -> Self {
        self.max_calls = Some(value);
        self
    }

    pub fn max_tokens(mut self, value: i64) -> Self {
        self.max_tokens = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AgentDocumentBudget`].
    pub fn build(self) -> Result<AgentDocumentBudget, BuildError> {
        Ok(AgentDocumentBudget {
            max_calls: self.max_calls,
            max_tokens: self.max_tokens,
        })
    }
}
