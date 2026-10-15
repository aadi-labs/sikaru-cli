pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RunEvent {
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: String,
    #[serde(default)]
    pub id: String,
    /// Fields specific to `eventType`; open by design. Documented shapes: `run.failed` carries `error`, a RunError (`code`, `message`, `recoverable`). `run.tool_call.failed` may carry `error` with `code` and `message`, where `code` is one of `effect_unconfirmed`, `approval_denied`, `access_changed`; the action did not run, or its outcome could not be confirmed and it was not repeated. `run.paused` carries `reason`: `credits_required` (credits or the agent spending cap ran out; the run resumes once funded) or `admission_deferred` (the run is waiting to be admitted and resumes on its own).
    #[serde(default)]
    pub payload: HashMap<String, serde_json::Value>,
    #[serde(default)]
    pub sequence: i64,
}

impl RunEvent {
    pub fn builder() -> RunEventBuilder {
        <RunEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RunEventBuilder {
    created_at: Option<String>,
    event_type: Option<String>,
    id: Option<String>,
    payload: Option<HashMap<String, serde_json::Value>>,
    sequence: Option<i64>,
}

impl RunEventBuilder {
    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn event_type(mut self, value: impl Into<String>) -> Self {
        self.event_type = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn payload(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.payload = Some(value);
        self
    }

    pub fn sequence(mut self, value: i64) -> Self {
        self.sequence = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RunEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`created_at`](RunEventBuilder::created_at)
    /// - [`event_type`](RunEventBuilder::event_type)
    /// - [`id`](RunEventBuilder::id)
    /// - [`payload`](RunEventBuilder::payload)
    /// - [`sequence`](RunEventBuilder::sequence)
    pub fn build(self) -> Result<RunEvent, BuildError> {
        Ok(RunEvent {
            created_at: self.created_at.ok_or_else(|| BuildError::missing_field("created_at"))?,
            event_type: self.event_type.ok_or_else(|| BuildError::missing_field("event_type"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            payload: self.payload.ok_or_else(|| BuildError::missing_field("payload"))?,
            sequence: self.sequence.ok_or_else(|| BuildError::missing_field("sequence"))?,
        })
    }
}
