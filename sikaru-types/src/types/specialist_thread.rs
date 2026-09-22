pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SpecialistThread {
    #[serde(rename = "finalText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_text: Option<String>,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(rename = "parentThreadId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_thread_id: Option<String>,
    #[serde(rename = "runId")]
    #[serde(default)]
    pub run_id: String,
    #[serde(default)]
    pub status: String,
}

impl SpecialistThread {
    pub fn builder() -> SpecialistThreadBuilder {
        <SpecialistThreadBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SpecialistThreadBuilder {
    final_text: Option<String>,
    id: Option<String>,
    name: Option<String>,
    parent_thread_id: Option<String>,
    run_id: Option<String>,
    status: Option<String>,
}

impl SpecialistThreadBuilder {
    pub fn final_text(mut self, value: impl Into<String>) -> Self {
        self.final_text = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn parent_thread_id(mut self, value: impl Into<String>) -> Self {
        self.parent_thread_id = Some(value.into());
        self
    }

    pub fn run_id(mut self, value: impl Into<String>) -> Self {
        self.run_id = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SpecialistThread`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](SpecialistThreadBuilder::id)
    /// - [`name`](SpecialistThreadBuilder::name)
    /// - [`run_id`](SpecialistThreadBuilder::run_id)
    /// - [`status`](SpecialistThreadBuilder::status)
    pub fn build(self) -> Result<SpecialistThread, BuildError> {
        Ok(SpecialistThread {
            final_text: self.final_text,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            parent_thread_id: self.parent_thread_id,
            run_id: self.run_id.ok_or_else(|| BuildError::missing_field("run_id"))?,
            status: self.status.ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
