pub use crate::prelude::*;
#[allow(unused_imports)]
use super::*;

/// A refused or failed request. Branch on `code`; `detail` is for people.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ErrorResponse {
    /// Stable machine code. Specific values: `insufficient_credits` (402): billing does not allow this request now; check your plan, credits and the agent's spending cap; `idempotency_conflict` (409): the idempotency key was already used for a different request, or for a file that was deleted; `concurrent_request` (409): another request is changing the same resource; send the same request again; `agent_disabled` (409): the agent is turned off; turn it on before starting new work; `release_unavailable` (409): the agent has no release that can run; `action_not_pending` (409): the tool call or approval is not waiting for this input: it already has an outcome, its run finished, it is handled by Sikaru, or it does not match a pending request; `not_retryable` (409): the run cannot be retried: it did not fail, its failure is not recoverable, or it failed too long ago; `branch_unavailable` (409): the session cannot be branched at that run; `record_invalid` (409): the retained record cannot be read; `state_conflict` (409): the request conflicts with the resource's current state; read it before trying again; `agent_capacity_full` (429): the agent's turn capacity is full; `runtime_updating` (503): a runtime update briefly paused new work; `agent_unavailable` (503): execution is switched off for this agent; `turn_delivery_pending` (503): the message was recorded but has not started; send it again with the same idempotency key; `execution_storage_unavailable` (503): session, file or workspace storage is unavailable right now; `storage_unavailable` (503): the storage behind this resource is unavailable right now; `trajectory_storage_unavailable` (503): retained run records cannot be read right now. Otherwise a status default: `bad_request` (400), `unauthenticated` (401), `payment_required` (402), `forbidden` (403), `not_found` (404), `method_not_allowed` (405), `not_acceptable` (406), `request_timeout` (408), `conflict` (409), `gone` (410), `length_required` (411), `precondition_failed` (412), `payload_too_large` (413), `unsupported_media_type` (415), `range_not_satisfiable` (416), `invalid_request` (422), `locked` (423), `precondition_required` (428), `rate_limited` (429), `internal_error` (500), `not_implemented` (501), `bad_gateway` (502), `service_unavailable` (503), `gateway_timeout` (504). `429` and `503` refusals with a specific code carry `Retry-After` in seconds.
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub detail: String,
}

impl ErrorResponse {
    pub fn builder() -> ErrorResponseBuilder {
        <ErrorResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ErrorResponseBuilder {
    code: Option<String>,
    detail: Option<String>,
}

impl ErrorResponseBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ErrorResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](ErrorResponseBuilder::code)
    /// - [`detail`](ErrorResponseBuilder::detail)
    pub fn build(self) -> Result<ErrorResponse, BuildError> {
        Ok(ErrorResponse {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            detail: self.detail.ok_or_else(|| BuildError::missing_field("detail"))?,
        })
    }
}
