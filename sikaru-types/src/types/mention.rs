pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Mention {
    #[serde(default)]
    pub column: i64,
    #[serde(default)]
    pub end: i64,
    #[serde(default)]
    pub kind: String,
    #[serde(default)]
    pub line: i64,
    #[serde(default)]
    pub slug: String,
    #[serde(default)]
    pub start: i64,
    #[serde(default)]
    pub text: String,
}

impl Mention {
    pub fn builder() -> MentionBuilder {
        <MentionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MentionBuilder {
    column: Option<i64>,
    end: Option<i64>,
    kind: Option<String>,
    line: Option<i64>,
    slug: Option<String>,
    start: Option<i64>,
    text: Option<String>,
}

impl MentionBuilder {
    pub fn column(mut self, value: i64) -> Self {
        self.column = Some(value);
        self
    }

    pub fn end(mut self, value: i64) -> Self {
        self.end = Some(value);
        self
    }

    pub fn kind(mut self, value: impl Into<String>) -> Self {
        self.kind = Some(value.into());
        self
    }

    pub fn line(mut self, value: i64) -> Self {
        self.line = Some(value);
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn start(mut self, value: i64) -> Self {
        self.start = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Mention`].
    /// This method will fail if any of the following fields are not set:
    /// - [`column`](MentionBuilder::column)
    /// - [`end`](MentionBuilder::end)
    /// - [`kind`](MentionBuilder::kind)
    /// - [`line`](MentionBuilder::line)
    /// - [`slug`](MentionBuilder::slug)
    /// - [`start`](MentionBuilder::start)
    /// - [`text`](MentionBuilder::text)
    pub fn build(self) -> Result<Mention, BuildError> {
        Ok(Mention {
            column: self.column.ok_or_else(|| BuildError::missing_field("column"))?,
            end: self.end.ok_or_else(|| BuildError::missing_field("end"))?,
            kind: self.kind.ok_or_else(|| BuildError::missing_field("kind"))?,
            line: self.line.ok_or_else(|| BuildError::missing_field("line"))?,
            slug: self.slug.ok_or_else(|| BuildError::missing_field("slug"))?,
            start: self.start.ok_or_else(|| BuildError::missing_field("start"))?,
            text: self.text.ok_or_else(|| BuildError::missing_field("text"))?,
        })
    }
}
