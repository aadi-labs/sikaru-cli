//! Authored command registration, preserved by both Fern and the local publisher.
use fern_cli_sdk::app::CliApp;

#[path = "compute.rs"]
mod compute;

pub fn register(app: CliApp) -> CliApp {
    compute::install(app)
}
