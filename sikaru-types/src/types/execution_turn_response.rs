pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ExecutionTurnResponse {
    #[serde(default)]
    pub input: ExecutionInputReceipt,
    #[serde(default)]
    pub run: ExecutionTurnRun,
}

impl ExecutionTurnResponse {
    pub fn builder() -> ExecutionTurnResponseBuilder {
        <ExecutionTurnResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExecutionTurnResponseBuilder {
    input: Option<ExecutionInputReceipt>,
    run: Option<ExecutionTurnRun>,
}

impl ExecutionTurnResponseBuilder {
    pub fn input(mut self, value: ExecutionInputReceipt) -> Self {
        self.input = Some(value);
        self
    }

    pub fn run(mut self, value: ExecutionTurnRun) -> Self {
        self.run = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExecutionTurnResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`input`](ExecutionTurnResponseBuilder::input)
    /// - [`run`](ExecutionTurnResponseBuilder::run)
    pub fn build(self) -> Result<ExecutionTurnResponse, BuildError> {
        Ok(ExecutionTurnResponse {
            input: self.input.ok_or_else(|| BuildError::missing_field("input"))?,
            run: self.run.ok_or_else(|| BuildError::missing_field("run"))?,
        })
    }
}
