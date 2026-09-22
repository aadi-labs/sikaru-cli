pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateCompatibilityProfileRequest {
    #[serde(rename = "approvalMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approval_map: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "displayName")]
    #[serde(default)]
    pub display_name: String,
    #[serde(rename = "evalMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eval_map: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "harnessId")]
    #[serde(default)]
    pub harness_id: String,
    #[serde(rename = "inputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "memoryMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memory_map: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "outputSchema")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_schema: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "runModes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub run_modes: Option<Vec<String>>,
    #[serde(rename = "toolMap")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_map: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "traceCoverageSummary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_coverage_summary: Option<HashMap<String, serde_json::Value>>,
}

impl CreateCompatibilityProfileRequest {
    pub fn builder() -> CreateCompatibilityProfileRequestBuilder {
        <CreateCompatibilityProfileRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateCompatibilityProfileRequestBuilder {
    approval_map: Option<HashMap<String, serde_json::Value>>,
    display_name: Option<String>,
    eval_map: Option<HashMap<String, serde_json::Value>>,
    harness_id: Option<String>,
    input_schema: Option<HashMap<String, serde_json::Value>>,
    memory_map: Option<HashMap<String, serde_json::Value>>,
    output_schema: Option<HashMap<String, serde_json::Value>>,
    run_modes: Option<Vec<String>>,
    tool_map: Option<HashMap<String, serde_json::Value>>,
    trace_coverage_summary: Option<HashMap<String, serde_json::Value>>,
}

impl CreateCompatibilityProfileRequestBuilder {
    pub fn approval_map(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.approval_map = Some(value);
        self
    }

    pub fn display_name(mut self, value: impl Into<String>) -> Self {
        self.display_name = Some(value.into());
        self
    }

    pub fn eval_map(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.eval_map = Some(value);
        self
    }

    pub fn harness_id(mut self, value: impl Into<String>) -> Self {
        self.harness_id = Some(value.into());
        self
    }

    pub fn input_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.input_schema = Some(value);
        self
    }

    pub fn memory_map(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.memory_map = Some(value);
        self
    }

    pub fn output_schema(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.output_schema = Some(value);
        self
    }

    pub fn run_modes(mut self, value: Vec<String>) -> Self {
        self.run_modes = Some(value);
        self
    }

    pub fn tool_map(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.tool_map = Some(value);
        self
    }

    pub fn trace_coverage_summary(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.trace_coverage_summary = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateCompatibilityProfileRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`display_name`](CreateCompatibilityProfileRequestBuilder::display_name)
    /// - [`harness_id`](CreateCompatibilityProfileRequestBuilder::harness_id)
    pub fn build(self) -> Result<CreateCompatibilityProfileRequest, BuildError> {
        Ok(CreateCompatibilityProfileRequest {
            approval_map: self.approval_map,
            display_name: self.display_name.ok_or_else(|| BuildError::missing_field("display_name"))?,
            eval_map: self.eval_map,
            harness_id: self.harness_id.ok_or_else(|| BuildError::missing_field("harness_id"))?,
            input_schema: self.input_schema,
            memory_map: self.memory_map,
            output_schema: self.output_schema,
            run_modes: self.run_modes,
            tool_map: self.tool_map,
            trace_coverage_summary: self.trace_coverage_summary,
        })
    }
}

