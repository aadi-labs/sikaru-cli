pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// Public ResponseStreamEvent fields at this level (RESPONSE_STREAM_EVENT_FIELDS); the projection omits the rest.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ResponseStreamEvent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotation_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delta: Option<ResponseStreamEventDelta>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<HashMap<String, serde_json::Value>>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub part: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_image_b64: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_image_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<ModelResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence_number: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl ResponseStreamEvent {
    pub fn builder() -> ResponseStreamEventBuilder {
        <ResponseStreamEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseStreamEventBuilder {
    annotation: Option<HashMap<String, serde_json::Value>>,
    annotation_index: Option<i64>,
    arguments: Option<String>,
    code: Option<String>,
    command: Option<String>,
    command_index: Option<i64>,
    content_index: Option<i64>,
    delta: Option<ResponseStreamEventDelta>,
    input: Option<String>,
    item: Option<HashMap<String, serde_json::Value>>,
    item_id: Option<String>,
    logprobs: Option<Vec<HashMap<String, serde_json::Value>>>,
    message: Option<String>,
    output: Option<Vec<HashMap<String, serde_json::Value>>>,
    output_format: Option<String>,
    output_index: Option<i64>,
    param: Option<String>,
    part: Option<HashMap<String, serde_json::Value>>,
    partial_image_b64: Option<String>,
    partial_image_index: Option<i64>,
    quality: Option<String>,
    refusal: Option<String>,
    response: Option<ModelResponse>,
    sequence_number: Option<i64>,
    size: Option<String>,
    status: Option<String>,
    summary_index: Option<i64>,
    text: Option<String>,
    r#type: Option<String>,
}

impl ResponseStreamEventBuilder {
    pub fn annotation(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.annotation = Some(value);
        self
    }

    pub fn annotation_index(mut self, value: i64) -> Self {
        self.annotation_index = Some(value);
        self
    }

    pub fn arguments(mut self, value: impl Into<String>) -> Self {
        self.arguments = Some(value.into());
        self
    }

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn command(mut self, value: impl Into<String>) -> Self {
        self.command = Some(value.into());
        self
    }

    pub fn command_index(mut self, value: i64) -> Self {
        self.command_index = Some(value);
        self
    }

    pub fn content_index(mut self, value: i64) -> Self {
        self.content_index = Some(value);
        self
    }

    pub fn delta(mut self, value: ResponseStreamEventDelta) -> Self {
        self.delta = Some(value);
        self
    }

    pub fn input(mut self, value: impl Into<String>) -> Self {
        self.input = Some(value.into());
        self
    }

    pub fn item(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.item = Some(value);
        self
    }

    pub fn item_id(mut self, value: impl Into<String>) -> Self {
        self.item_id = Some(value.into());
        self
    }

    pub fn logprobs(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.logprobs = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn output(mut self, value: Vec<HashMap<String, serde_json::Value>>) -> Self {
        self.output = Some(value);
        self
    }

    pub fn output_format(mut self, value: impl Into<String>) -> Self {
        self.output_format = Some(value.into());
        self
    }

    pub fn output_index(mut self, value: i64) -> Self {
        self.output_index = Some(value);
        self
    }

    pub fn param(mut self, value: impl Into<String>) -> Self {
        self.param = Some(value.into());
        self
    }

    pub fn part(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.part = Some(value);
        self
    }

    pub fn partial_image_b64(mut self, value: impl Into<String>) -> Self {
        self.partial_image_b64 = Some(value.into());
        self
    }

    pub fn partial_image_index(mut self, value: i64) -> Self {
        self.partial_image_index = Some(value);
        self
    }

    pub fn quality(mut self, value: impl Into<String>) -> Self {
        self.quality = Some(value.into());
        self
    }

    pub fn refusal(mut self, value: impl Into<String>) -> Self {
        self.refusal = Some(value.into());
        self
    }

    pub fn response(mut self, value: ModelResponse) -> Self {
        self.response = Some(value);
        self
    }

    pub fn sequence_number(mut self, value: i64) -> Self {
        self.sequence_number = Some(value);
        self
    }

    pub fn size(mut self, value: impl Into<String>) -> Self {
        self.size = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn summary_index(mut self, value: i64) -> Self {
        self.summary_index = Some(value);
        self
    }

    pub fn text(mut self, value: impl Into<String>) -> Self {
        self.text = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponseStreamEvent`].
    pub fn build(self) -> Result<ResponseStreamEvent, BuildError> {
        Ok(ResponseStreamEvent {
            annotation: self.annotation,
            annotation_index: self.annotation_index,
            arguments: self.arguments,
            code: self.code,
            command: self.command,
            command_index: self.command_index,
            content_index: self.content_index,
            delta: self.delta,
            input: self.input,
            item: self.item,
            item_id: self.item_id,
            logprobs: self.logprobs,
            message: self.message,
            output: self.output,
            output_format: self.output_format,
            output_index: self.output_index,
            param: self.param,
            part: self.part,
            partial_image_b64: self.partial_image_b64,
            partial_image_index: self.partial_image_index,
            quality: self.quality,
            refusal: self.refusal,
            response: self.response,
            sequence_number: self.sequence_number,
            size: self.size,
            status: self.status,
            summary_index: self.summary_index,
            text: self.text,
            r#type: self.r#type,
        })
    }
}
