//! Authored command registration, preserved by both Fern and the local publisher.
use fern_cli_sdk::app::CliApp;

#[path = "compute.rs"]
mod compute;

#[path = "agent.rs"]
mod agent;

#[cfg(unix)]
#[path = "agent_run.rs"]
mod agent_run;

#[path = "eval.rs"]
mod eval;

#[path = "environment.rs"]
mod environment;

#[path = "traces.rs"]
mod traces;

#[path = "user_auth.rs"]
mod user_auth;

#[path = "inference.rs"]
mod inference;

#[path = "account.rs"]
mod account;

const DESCRIPTION: &str = "Sikaru: model inference, managed agents, evals, traces and environments. Run a product with --help to see its commands; every API operation stays callable under its product.";

pub fn register(app: CliApp) -> CliApp {
    account::apply_saved_project();
    let app = [
        agent::install,
        eval::install,
        environment::install,
        traces::install,
    ]
    .into_iter()
    .fold(compute::install(user_auth::install(app)), |app, install| {
        install(app)
    });
    let app = account::install(inference::install(app));
    hide_generated_groups(app.description(DESCRIPTION))
}

/// Generated API groups stay callable under their product but out of default help.
fn hide_generated_groups(app: CliApp) -> CliApp {
    crate::commands::HIDDEN_GROUPS
        .iter()
        .fold(app.hide(&["man"]), |app, path| app.hide(path))
}
