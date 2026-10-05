pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureItem {
    /// A flag's labels, kept as tags that stay in Sikaru.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flag_labels: Option<Vec<String>>,
    #[serde(default)]
    pub run_id: String,
    /// The agent step to use as the expected answer; earlier messages become the input. Omit for the whole run and its final answer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seq_to: Option<i64>,
}

impl CaptureItem {
    pub fn builder() -> CaptureItemBuilder {
        <CaptureItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureItemBuilder {
    flag_labels: Option<Vec<String>>,
    run_id: Option<String>,
    seq_to: Option<i64>,
}

impl CaptureItemBuilder {
    pub fn flag_labels(mut self, value: Vec<String>) -> Self {
        self.flag_labels = Some(value);
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn seq_to(mut self, value: i64) -> Self {
        self.seq_to = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`run_id`](CaptureItemBuilder::run_id)
    pub fn build(self) -> Result<CaptureItem, BuildError> {
        Ok(CaptureItem {
            flag_labels: self.flag_labels,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            seq_to: self.seq_to,
        })
    }
}
