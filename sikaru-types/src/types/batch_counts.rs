pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BatchCounts {
    #[serde(default)]
    pub added: i64,
    #[serde(default)]
    pub duplicate: i64,
    #[serde(default)]
    pub skipped: i64,
}

impl BatchCounts {
    pub fn builder() -> BatchCountsBuilder {
        <BatchCountsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchCountsBuilder {
    added: Option<i64>,
    duplicate: Option<i64>,
    skipped: Option<i64>,
}

impl BatchCountsBuilder {
    pub fn added(mut self, value: i64) -> Self {
        self.added = Some(value);
        self
    }

    pub fn duplicate(mut self, value: i64) -> Self {
        self.duplicate = Some(value);
        self
    }

    pub fn skipped(mut self, value: i64) -> Self {
        self.skipped = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchCounts`].
    /// This method will fail if any of the following fields are not set:
    /// - [`added`](BatchCountsBuilder::added)
    /// - [`duplicate`](BatchCountsBuilder::duplicate)
    /// - [`skipped`](BatchCountsBuilder::skipped)
    pub fn build(self) -> Result<BatchCounts, BuildError> {
        Ok(BatchCounts {
            added: self.added.ok_or_else(|| BuildError::missing_field("added"))?,
            duplicate: self.duplicate.ok_or_else(|| BuildError::missing_field("duplicate"))?,
            skipped: self.skipped.ok_or_else(|| BuildError::missing_field("skipped"))?,
        })
    }
}
