//! Stable advice never includes remote response bodies, tokens, or request URLs.
use super::{input::InvalidInput, state, transport::TransportFailure};
use serde_json::{json, Value};

pub fn failure(error: &anyhow::Error, reason: &str) -> Value {
    if let Some(input) = error.downcast_ref::<InvalidInput>() {
        return json!({"status":"failed","reason":"invalid_input","message":input.0,
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
