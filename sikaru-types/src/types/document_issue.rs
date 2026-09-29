pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub column: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<i64>,
    #[serde(default)]
    pub message: String,
}

impl DocumentIssue {
    pub fn builder() -> DocumentIssueBuilder {
        <DocumentIssueBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentIssueBuilder {
    column: Option<i64>,
    line: Option<i64>,
    message: Option<String>,
}

impl DocumentIssueBuilder {
    pub fn column(mut self, value: i64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentIssue`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](DocumentIssueBuilder::message)
    pub fn build(self) -> Result<DocumentIssue, BuildError> {
        Ok(DocumentIssue {
            column: self.column,
            line: self.line,
            message: self.message.ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
