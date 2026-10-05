pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateExample {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected: Option<UpdateExampleExpected>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

impl UpdateExample {
    pub fn builder() -> UpdateExampleBuilder {
        <UpdateExampleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateExampleBuilder {
    expected: Option<UpdateExampleExpected>,
    tags: Option<Vec<String>>,
}

impl UpdateExampleBuilder {
    pub fn expected(mut self, value: UpdateExampleExpected) -> Self {
        self.expected = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateExample`].
    pub fn build(self) -> Result<UpdateExample, BuildError> {
        Ok(UpdateExample {
            expected: self.expected,
            tags: self.tags,
        })
    }
}

