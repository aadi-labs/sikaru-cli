pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DraftDocumentInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    #[serde(rename = "runIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_ids: Option<Vec<String>>,
    #[serde(rename = "templateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_id: Option<String>,
    #[serde(rename = "traceIds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_ids: Option<Vec<String>>,
}

impl DraftDocumentInput {
    pub fn builder() -> DraftDocumentInputBuilder {
        <DraftDocumentInputBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DraftDocumentInputBuilder {
    description: Option<String>,
    note: Option<String>,
    run_ids: Option<Vec<String>>,
    template_id: Option<String>,
    trace_ids: Option<Vec<String>>,
}

impl DraftDocumentInputBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    pub fn run_ids(mut self, value: Vec<String>) -> Self {
        self.run_ids = Some(value);
        self
    }

    pub fn template_id(mut self, value: impl Into<String>) -> Self {
        self.template_id = Some(value.into());
        self
    }

    pub fn trace_ids(mut self, value: Vec<String>) -> Self {
        self.trace_ids = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DraftDocumentInput`].
    pub fn build(self) -> Result<DraftDocumentInput, BuildError> {
        Ok(DraftDocumentInput {
            description: self.description,
            note: self.note,
            run_ids: self.run_ids,
            template_id: self.template_id,
            trace_ids: self.trace_ids,
        })
    }
}

