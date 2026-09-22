pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InvoiceBudget {
    #[serde(default)]
    pub learning: ResourceBudget,
    #[serde(default)]
    pub period_end: i64,
    #[serde(default)]
    pub period_start: i64,
    #[serde(default)]
    pub production: ResourceBudget,
    #[serde(default)]
    pub state: String,
}

impl InvoiceBudget {
    pub fn builder() -> InvoiceBudgetBuilder {
        <InvoiceBudgetBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceBudgetBuilder {
    learning: Option<ResourceBudget>,
    period_end: Option<i64>,
    period_start: Option<i64>,
    production: Option<ResourceBudget>,
    state: Option<String>,
}

impl InvoiceBudgetBuilder {
    pub fn learning(mut self, value: ResourceBudget) -> Self {
        self.learning = Some(value);
        self
    }

    pub fn period_end(mut self, value: i64) -> Self {
        self.period_end = Some(value);
        self
    }

    pub fn period_start(mut self, value: i64) -> Self {
        self.period_start = Some(value);
        self
    }

    pub fn production(mut self, value: ResourceBudget) -> Self {
        self.production = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InvoiceBudget`].
    /// This method will fail if any of the following fields are not set:
    /// - [`learning`](InvoiceBudgetBuilder::learning)
    /// - [`period_end`](InvoiceBudgetBuilder::period_end)
    /// - [`period_start`](InvoiceBudgetBuilder::period_start)
    /// - [`production`](InvoiceBudgetBuilder::production)
    /// - [`state`](InvoiceBudgetBuilder::state)
    pub fn build(self) -> Result<InvoiceBudget, BuildError> {
        Ok(InvoiceBudget {
            learning: self.learning.ok_or_else(|| BuildError::missing_field("learning"))?,
            period_end: self.period_end.ok_or_else(|| BuildError::missing_field("period_end"))?,
            period_start: self.period_start.ok_or_else(|| BuildError::missing_field("period_start"))?,
            production: self.production.ok_or_else(|| BuildError::missing_field("production"))?,
            state: self.state.ok_or_else(|| BuildError::missing_field("state"))?,
        })
    }
}
