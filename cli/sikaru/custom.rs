//! Authored command registration, preserved by both Fern and the local publisher.
use fern_cli_sdk::app::CliApp;

#[path = "compute.rs"]
mod compute;

#[path = "agents.rs"]
mod agents;

#[path = "user_auth.rs"]
mod user_auth;

pub fn register(app: CliApp) -> CliApp {
    agents::install(compute::install(user_auth::install(app).description("Run a hosted Sikaru agent in your workspace. Start with auth login, doctor, then exec. Use exec --print for scripts; all API commands remain available.")))
}
