//! Stable advice never includes remote response bodies, tokens, or request URLs.
use super::{input::InvalidInput, state, transport::TransportFailure};
use serde_json::{json, Value};

#[derive(Debug, thiserror::Error)]
#[error("agent_release_unavailable")]
pub struct UnavailableRelease;

/// Only session admission can prove this rejection happened before execution.
pub async fn session_admission<T>(
    future: impl std::future::Future<Output = Result<T, sikaru_sdk::ApiError>>,
) -> anyhow::Result<T> {
    tokio::time::timeout(std::time::Duration::from_secs(10), future)
        .await
        .map_err(|_| anyhow::Error::new(TransportFailure::Transient))?
        .map_err(|error| match &error {
            sikaru_sdk::ApiError::ConflictError {
                detail: Some(detail),
                ..
            } if detail == "agent_release_unavailable" => anyhow::Error::new(UnavailableRelease),
            sikaru_sdk::ApiError::Http {
                status: 409,
                message,
            } if unavailable_detail(message) => anyhow::Error::new(UnavailableRelease),
            _ => super::transport::classify(error),
        })
}

fn unavailable_detail(body: &str) -> bool {
    serde_json::from_str::<Value>(body)
        .ok()
        .is_some_and(|value| value["detail"] == "agent_release_unavailable")
}

pub fn failure(error: &anyhow::Error, reason: &str) -> Value {
    if let Some(input) = error.downcast_ref::<InvalidInput>() {
        return json!({"status":"failed","reason":"invalid_input","message":input.0,
            "execution":null,"cleanup":"not_started","cancel_acknowledged":null,
            "usage":{"available":false}});
    }
    if error.is::<UnavailableRelease>() {
        return json!({"status":"failed","reason":"agent_release_unavailable",
            "help":"No work started. Ask an agent administrator to upgrade this agent, then retry with the saved state directory. Existing sessions retain their release.",
            "execution":null,"cleanup":"not_started","cancel_acknowledged":null,
            "usage":{"available":false}});
    }
    let mut result = state::failure(reason);
    result["help"] = json!(advice(error));
    result
}

fn advice(error: &anyhow::Error) -> &'static str {
    match error.downcast_ref::<TransportFailure>() {
        Some(TransportFailure::Rejected(401)) => "Authentication failed. Run sikaru auth login, then resume the saved state directory.",
        Some(TransportFailure::Rejected(402)) => "Billing blocked this request. Check the team's subscription and available budget, then resume the saved state directory.",
        Some(TransportFailure::Rejected(403)) => "Access denied. Check that your API key has access to this project and the required scopes.",
        Some(TransportFailure::Rejected(404)) => "Resource not found. Check --project and --agent, or your saved session's availability.",
        Some(TransportFailure::Transient) => "The service could not be reached or is temporarily unavailable. Check connectivity and service status; preserve the state directory for explicit resume.",
        _ => "Preserve the original workspace and state directory. Inspect the saved run before resuming; do not start a replacement run to retry an uncertain action.",
    }
}
