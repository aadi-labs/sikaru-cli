//! Authored native compute commands; generated API commands remain the transport.
use fern_cli_sdk::{app::CliApp, openapi::OpenApiBinding};
#[path = "compute_config.rs"]
pub mod config;
#[cfg(unix)]
#[path = "compute_journal.rs"]
pub mod journal;
#[cfg(unix)]
#[path = "compute_process.rs"]
pub mod process;
#[cfg(unix)]
#[path = "compute_runtime.rs"]
pub mod runtime;
#[cfg(unix)]
#[path = "compute_transport.rs"]
pub mod transport;

#[cfg(unix)]
#[path = "compute_chat.rs"]
pub mod chat;
#[cfg(unix)]
#[path = "compute_diagnostics.rs"]
pub mod diagnostics;
#[cfg(unix)]
#[path = "compute_doctor.rs"]
pub mod doctor;
#[cfg(unix)]
#[path = "compute_input.rs"]
pub mod input;
#[cfg(unix)]
#[path = "compute_state.rs"]
pub mod state;
#[cfg(unix)]
#[path = "compute_workflow.rs"]
pub mod workflow;

pub fn install(app: CliApp) -> CliApp {
    #[cfg(unix)]
    let app = app
        .command(
            inspection_command(),
            OpenApiBinding::handler(|m, ctx| {
                let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(inspect_run(m, ctx))
                })
                .map_err(inspection_error)?;
                println!("{}", result);
                Ok(())
            }),
        )
        .command(
            doctor::command(),
            OpenApiBinding::handler(|m, ctx| {
                let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(doctor::execute(m, ctx))
                });
                emit(Ok(result));
                Ok(())
            }),
        );
    app
}
/// Prints one JSON result and exits with its outcome code.
#[cfg(unix)]
pub fn emit(result: anyhow::Result<serde_json::Value>) {
    let result = result
        .unwrap_or_else(|error| diagnostics::failure(&error, "bootstrap_or_journal_rejected"));
    println!("{}", result);
    use std::io::Write;
    let _ = std::io::stdout().flush();
    let code = exit_code(&result);
    if code != 0 {
        std::process::exit(code);
    }
}
/// Stable native headless outcomes.
pub fn exit_code(result: &serde_json::Value) -> i32 {
    match result["status"].as_str() {
        Some("completed") => 0,
        Some("approval_required") => 2,
        Some("cancelled") => 3,
        Some("recovery_required") => 4,
        Some("paused") => 5,
        _ => 1,
    }
}

#[cfg(unix)]
fn inspection_command() -> clap::Command {
    clap::Command::new("inspect-run")
        .hide(true)
        .arg(
            clap::Arg::new("project-id")
                .long("project-id")
                .required(true),
        )
        .arg(clap::Arg::new("run-id").long("run-id").required(true))
}

#[cfg(unix)]
async fn inspect_run(
    m: &clap::ArgMatches,
    ctx: &fern_cli_sdk::openapi::AppContext,
) -> anyhow::Result<serde_json::Value> {
    let options = sikaru_sdk::RequestOptions::new()
        .additional_header("X-Sikaru-Run-Inspection", "true")
        .max_retries(0);
    let client = crate::sdk::client(ctx);
    let value = state::call(client.runs.get(
        m.get_one::<String>("project-id").unwrap(),
        m.get_one::<String>("run-id").unwrap(),
        Some(options),
    ))
    .await?;
    Ok(serde_json::to_value(value)?)
}

#[cfg(unix)]
fn inspection_error(error: anyhow::Error) -> fern_cli_sdk::error::CliError {
    use fern_cli_sdk::error::CliError;
    match error.downcast_ref::<transport::TransportFailure>() {
        Some(transport::TransportFailure::Rejected(code)) => CliError::api(
            *code,
            "Run inspection was rejected; check the selected credential and project access",
            "requestRejected",
        ),
        Some(transport::TransportFailure::Transient) => CliError::Network(
            "Run inspection is temporarily unavailable; check connectivity and retry".into(),
        ),
        _ => CliError::api(500, "Run inspection failed", "inspectionError"),
    }
}

#[cfg(unix)]
#[path = "compute_checkpoint.rs"]
pub mod checkpoint;
#[cfg(unix)]
#[path = "compute_checkpoint_repo.rs"]
pub mod checkpoint_repo;
