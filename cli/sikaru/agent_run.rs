//! `sikaru agent run`: one verb runs an agent on Sikaru (the default) or with its tools in the
//! local workspace (`--compute local`). With a task it runs once; in a terminal without one it
//! opens a conversation whose messages continue one session. Progress goes to stderr and stdout
//! carries one JSON result; the exit code is the outcome (completed 0, approval required 2,
//! cancelled 3, recovery required 4, paused for funding 5, otherwise 1).
use super::agent::{follow_until, pipeline, EventPrinter};
use super::compute::{self, chat, diagnostics, input, runtime, state, workflow};
use anyhow::{bail, Context, Result};
use clap::{parser::ValueSource, ArgMatches, Command};
use fern_cli_sdk::{error::CliError, openapi::AppContext};
use serde_json::{json, Value};
use sikaru_sdk::api::*;
use std::time::{Duration, Instant};

/// A managed run stops streaming once it settles or waits on a person.
const MANAGED_STOPS: [&str; 6] = [
    "run.completed",
    "run.failed",
    "run.cancelled",
    "session.personal",
    "run.requires_approval",
    "run.paused",
];
/// Options that only shape a run served from the local workspace.
const LOCAL_ONLY: [&str; 5] = [
    "resume",
    "state-dir",
    "workspace",
    "approval-wait",
    "no-workspace-checkpoints",
];

pub fn command() -> Command {
    workflow::command()
}

pub fn run(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let conversation = chat::is_conversation(m);
    let result = if is_local(m) {
        block(local(m, ctx, conversation))
    } else {
        managed(m, ctx, conversation)
    };
    emit(result, chat::is_interactive(m), conversation);
    Ok(())
}

fn is_local(m: &ArgMatches) -> bool {
    m.get_one::<String>("compute").map(String::as_str) == Some("local")
}

fn arg<'a>(m: &'a ArgMatches, name: &str) -> &'a str {
    m.get_one::<String>(name)
        .expect("clap validates required arguments")
}

fn block<T>(future: impl std::future::Future<Output = T>) -> T {
    tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(future))
}

/// The existing provision, attach and serve pipeline.
async fn local(m: &ArgMatches, ctx: &AppContext, conversation: bool) -> Result<Value> {
    if conversation {
        chat::execute(m, ctx).await
    } else {
        workflow::execute(m, ctx).await
    }
}

/// A person at a terminal reads the rendered answer; everything else gets the JSON result.
fn emit(result: Result<Value>, interactive: bool, conversation: bool) {
    use std::io::IsTerminal;
    if !interactive || !std::io::stdout().is_terminal() {
        compute::emit(result);
        return;
    }
    let result =
        result.unwrap_or_else(|error| diagnostics::failure(&error, "controller_or_resume_failed"));
    let code = compute::exit_code(&result);
    // A conversation has already shown each successful answer.
    if !conversation || code != 0 {
        chat::render(&result);
    }
    if code != 0 {
        std::process::exit(code);
    }
}

/// Sikaru runs the harness and its tools: a session, one turn per task, and the run's events.
fn managed(m: &ArgMatches, ctx: &AppContext, conversation: bool) -> Result<Value> {
    if LOCAL_ONLY
        .iter()
        .any(|name| m.value_source(name) == Some(ValueSource::CommandLine))
    {
        bail!(input::InvalidInput(
            "--resume, --state-dir, --workspace and --approval-wait apply to --compute local."
        ));
    }
    let client = crate::sdk::client(ctx);
    if conversation {
        return converse(m, &client);
    }
    let text = input::prompt(m)?.context("a managed run needs a task")?;
    if let Some(validated) = workflow::dry_run(m, None) {
        return Ok(validated);
    }
    let session = open_session(m, &client)?;
    managed_turn(m, &client, &session, &text)
}

fn converse(m: &ArgMatches, client: &ApiClient) -> Result<Value> {
    let session = open_session(m, client)?;
    let mut last = json!({"status":"completed","session_id":session,"execution":null,
        "usage":{"available":false}});
    eprintln!("Sikaru — enter a task, /help for commands, /exit to leave.");
    while let Some(task) = block(chat::next_task(None, &last))? {
        last = managed_turn(m, client, &session, &task)
            .unwrap_or_else(|error| diagnostics::failure(&error, "managed_run_failed"));
        chat::render(&last);
        if last["status"] != "completed" {
            break;
        }
    }
    Ok(last)
}

fn open_session(m: &ArgMatches, client: &ApiClient) -> Result<String> {
    let request = SessionInput {
        tenant_id: m.get_one::<String>("tenant").cloned(),
        user_id: m.get_one::<String>("user").cloned(),
        idempotency_key: Some(state::identity()),
        model: m.get_one::<String>("model").cloned(),
        ..Default::default()
    };
    let created = block(diagnostics::session_admission(
        client
            .execution_sessions
            .create(arg(m, "project"), arg(m, "agent"), &request, None),
    ))?;
    Ok(created.session.id)
}

/// One task: append it without a compute attachment, stream the run's events to stderr, then
/// read the settled run.
fn managed_turn(m: &ArgMatches, client: &ApiClient, session: &str, text: &str) -> Result<Value> {
    let project = arg(m, "project");
    let run = append_turn(client, project, session, text)?;
    runtime::progress(
        chat::is_interactive(m),
        "Working…",
        json!({"event":"turn_submitted","session_id":session,"run_id":run}),
    );
    let mut printer = EventPrinter::new(pipeline(m)?, Box::new(std::io::stderr()));
    let timeout = Duration::from_secs(*m.get_one::<u64>("timeout").unwrap());
    let deadline = Some(Instant::now() + timeout);
    let stopped = follow_until(
        client,
        project,
        &run,
        &mut printer,
        &MANAGED_STOPS,
        deadline,
    )?;
    let mut result = json!({"session_id":session,"execution":null,"usage":{"available":false}});
    block(workflow::enrich(
        client,
        project,
        Some(run.clone()),
        &mut result,
    ));
    match stopped {
        Some(event) if event.event_type == "run.paused" => paused(project, &run, &event, &mut result),
        Some(_) => result["status"] = json!(outcome(&result["result"]["status"])),
        None => cancel_at_deadline(client, project, &run, &mut result),
    }
    Ok(result)
}

/// A funding pause keeps the run and its work; it resumes once billing allows.
fn paused(project: &str, run: &str, event: &RunEvent, result: &mut Value) {
    let pause = result["result"]["pause"].clone();
    result["status"] = json!("paused");
    result["pause"] = if pause.is_object() {
        pause
    } else {
        json!({"reason": event.payload.get("reason"), "requires_action": true})
    };
    result["next"] = json!(format!(
        "Resolve the billing issue, then run: sikaru agent runs resume --project-id {project} --run-id {run}"
    ));
}

fn append_turn(client: &ApiClient, project: &str, session: &str, text: &str) -> Result<String> {
    let turn = TurnInput {
        idempotency_key: state::identity(),
        input: serde_json::from_value(json!({"goal":text}))?,
        ..Default::default()
    };
    let appended = block(state::call(
        client
            .execution_sessions
            .append_turn(project, session, &turn, None),
    ))?;
    Ok(appended.run.id)
}

/// A managed run's status as the task outcome `exit_code` reads.
fn outcome(status: &Value) -> &'static str {
    match status.as_str() {
        Some("completed") => "completed",
        Some("waiting_for_human") => "approval_required",
        Some("cancelled") => "cancelled",
        _ => "failed",
    }
}

fn cancel_at_deadline(client: &ApiClient, project: &str, run: &str, result: &mut Value) {
    let acknowledged = block(state::call(client.runs.cancel(project, run, None))).is_ok();
    result["status"] = json!("cancelled");
    result["reason"] = json!("deadline");
    result["cancel_acknowledged"] = json!(acknowledged);
}
