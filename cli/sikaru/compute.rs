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
#[path = "compute_launcher.rs"]
pub mod launcher;
#[cfg(unix)]
#[path = "compute_state.rs"]
pub mod state;
#[cfg(unix)]
#[path = "compute_worker.rs"]
pub mod worker;
#[cfg(unix)]
#[path = "compute_workflow.rs"]
pub mod workflow;

pub fn install(app: CliApp) -> CliApp {
    #[cfg(unix)]
    {
        return app
            .command(
                doctor::command(),
                OpenApiBinding::handler(|m, ctx| {
                    let result = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(doctor::execute(m, ctx))
                    });
                    emit(Ok(result));
                    Ok(())
                }),
            )
            .command(
                workflow::command(),
                OpenApiBinding::handler(|m, ctx| {
                    let result = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(workflow::execute(m, ctx))
                    });
                    emit(result);
                    Ok(())
                }),
            )
            .command(
                chat::command(),
                OpenApiBinding::handler(|m, ctx| {
                    let result = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(chat::execute(m, ctx))
                    });
                    emit(result);
                    Ok(())
                }),
            )
            .command(
                clap::Command::new("compute")
                    .about("Run admitted task IO on customer compute")
                    .subcommand(
                        clap::Command::new("serve")
                            .about("Serve one restricted attachment (Unix)")
                            .arg(
                                clap::Arg::new("bootstrap")
                                    .long("bootstrap")
                                    .required(true)
                                    .help("Private bootstrap JSON file, or - for stdin"),
                            ),
                    )
                    .subcommand(worker::command()),
                OpenApiBinding::handler(|m, ctx| {
                    let result = tokio::task::block_in_place(|| {
                        tokio::runtime::Handle::current().block_on(execute(m, ctx))
                    });
                    emit(result);
                    Ok(())
                }),
            );
    }
    #[cfg(not(unix))]
    {
        app.command(
            clap::Command::new("compute").about("Native compute requires macOS or Linux"),
            OpenApiBinding::handler(|_, _| {
                Err(fern_cli_sdk::error::CliError::Validation(
                    "native executor requires Unix".into(),
                ))
            }),
        )
    }
}
#[cfg(unix)]
async fn execute(
    m: &clap::ArgMatches,
    ctx: &fern_cli_sdk::openapi::AppContext,
) -> anyhow::Result<serde_json::Value> {
    match m.subcommand() {
        Some(("serve", options)) => {
            runtime::serve(
                config::Bootstrap::read(options.get_one::<String>("bootstrap").unwrap())?,
                ctx.effective_base_url(),
                ctx.http_config().build_client()?,
            )
            .await
        }
        Some(("worker", options)) => worker::execute(options, ctx).await,
        _ => anyhow::bail!("choose compute serve or worker"),
    }
}
#[cfg(unix)]
fn emit(result: anyhow::Result<serde_json::Value>) {
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
/// Stable native headless outcomes, shared with controller/worker composition.
pub fn exit_code(result: &serde_json::Value) -> i32 {
    match result["status"].as_str() {
        Some("completed") => 0,
        Some("approval_required") => 2,
        Some("cancelled") => 3,
        Some("recovery_required") => 4,
        _ => 1,
    }
}
