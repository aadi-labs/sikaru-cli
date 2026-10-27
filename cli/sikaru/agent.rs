//! `sikaru agent`: managed agents, their runs, and document authoring over the generated public
//! client. The agent (or run) is positional; `-p/--project` resolves flag, `SIKARU_PROJECT`, then
//! the saved project. Results render through the framework's `--format`.
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    formatter::OutputPipeline,
    openapi::{AppContext, OpenApiBinding},
};
use futures_util::StreamExt;
use serde::Serialize;
use serde_json::Value;
use sikaru_sdk::{api::*, ApiError, SseStream};
use std::pin::Pin;

/// Following stops at these. A personal session the caller can't see ends its stream with
/// `session.personal` on every connection, so it ends the follow too.
const FINAL_EVENTS: [&str; 4] = [
    "run.completed",
    "run.failed",
    "run.cancelled",
    "session.personal",
];
const EVENT_PAGE: i64 = 1000;

type Handler = fn(&ArgMatches, &AppContext) -> Result<(), CliError>;

pub fn install(app: CliApp) -> CliApp {
    let verbs: [(Command, Handler); 11] = [
        (list_command(), list),
        (get_command(), get),
        (create_command(), create),
        (logs_command(), logs),
        (stop_command(), stop),
        (pull_command(), pull),
        (push_command(), push),
        (validate_command(), validate),
        (review_command(), review),
        (publish_command(), publish),
        (snippets_command(), snippets),
    ];
    let app = verbs.into_iter().fold(app, |app, (command, handler)| {
        app.command_under(&["agent"], command, OpenApiBinding::handler(handler))
    });
    // `agent run` serves local tools through the Unix-only native executor.
    #[cfg(unix)]
    let app = app.command_under(
        &["agent"],
        super::agent_run::command(),
        OpenApiBinding::handler(super::agent_run::run),
    );
    app.hide(&["agent", "snippets"])
}

pub(super) fn project() -> Arg {
    Arg::new("project")
        .long("project")
        .short('p')
        .env("SIKARU_PROJECT")
        .required(true)
}

fn agent_command(name: &'static str, about: &'static str) -> Command {
    Command::new(name).about(about).arg(project()).arg(
        Arg::new("agent")
            .env("SIKARU_AGENT")
            .required(true)
            .help("Agent slug"),
    )
}

fn run_command(name: &'static str, about: &'static str) -> Command {
    Command::new(name)
        .about(about)
        .arg(project())
        .arg(Arg::new("run").required(true).help("Run id"))
}

fn file(command: Command) -> Command {
    command.arg(Arg::new("file").long("file").default_value("agent.md"))
}

fn revision(command: Command) -> Command {
    command.arg(
        Arg::new("revision")
            .long("revision")
            .required(true)
            .value_parser(clap::value_parser!(i64)),
    )
}

fn live_version(command: Command) -> Command {
    command.arg(
        Arg::new("live-version")
            .long("live-version")
            .help("Expected live version; omit before the first publication"),
    )
}

fn flag(command: Command, name: &'static str, help: &'static str) -> Command {
    command.arg(
        Arg::new(name)
            .long(name)
            .action(ArgAction::SetTrue)
            .help(help),
    )
}

fn list_command() -> Command {
    Command::new("list")
        .about("List the project's managed agents")
        .arg(project())
}

fn get_command() -> Command {
    agent_command("get", "Show one managed agent")
}

fn create_command() -> Command {
    let command = file(agent_command(
        "create",
        "Create an agent from an agent document",
    ))
    .arg(
        Arg::new("name")
            .long("name")
            .help("Display name; defaults to the slug"),
    );
    flag(
        command,
        "personal",
        "Keep each person's conversations private to them; fixed at creation",
    )
}

pub(super) fn logs_command() -> Command {
    let command = run_command("logs", "Print a run's events");
    command.arg(
        Arg::new("follow")
            .long("follow")
            .short('f')
            .action(ArgAction::SetTrue)
            .help("Keep streaming until the run completes, fails or is cancelled"),
    )
}

pub(super) fn stop_command() -> Command {
    run_command("stop", "Cancel a run")
}

fn pull_command() -> Command {
    file(agent_command(
        "pull",
        "Write the agent's saved draft to a file",
    ))
}

fn push_command() -> Command {
    revision(file(agent_command(
        "push",
        "Save a file as the agent's draft; never creates an agent",
    )))
}

fn validate_command() -> Command {
    file(agent_command("validate", "Validate an agent document file"))
}

fn review_command() -> Command {
    live_version(revision(agent_command(
        "review",
        "Review a draft revision's access before publishing",
    )))
}

fn publish_command() -> Command {
    let command = live_version(revision(agent_command(
        "publish",
        "Publish a reviewed draft revision",
    )))
    .arg(
        Arg::new("access-digest")
            .long("access-digest")
            .required(true)
            .help("accessDigest returned by the access review you approved"),
    );
    let command = flag(
        command,
        "acknowledge-widening",
        "Accept access the revision widens",
    );
    flag(
        command,
        "acknowledge-removals",
        "Accept access the revision removes",
    )
}

fn snippets_command() -> Command {
    agent_command("snippets", "List the agent's document snippets")
}

pub(super) fn value<'a>(m: &'a ArgMatches, name: &str) -> &'a str {
    m.get_one::<String>(name)
        .expect("clap validates required arguments")
}

fn revision_of(m: &ArgMatches) -> i64 {
    *m.get_one::<i64>("revision")
        .expect("clap validates required arguments")
}

pub(super) fn invalid(error: impl std::fmt::Display) -> CliError {
    CliError::Validation(error.to_string())
}

fn read_file(m: &ArgMatches) -> Result<String, CliError> {
    std::fs::read_to_string(value(m, "file")).map_err(invalid)
}

/// Renders through the framework pipeline, so `--format table|json` behaves as in every group.
pub(super) fn emit(m: &ArgMatches, result: &impl Serialize) -> Result<(), CliError> {
    let value = serde_json::to_value(result).map_err(invalid)?;
    pipeline(m)?
        .emit(&mut std::io::stdout(), &value, false, true)
        .map_err(invalid)
}

pub(super) fn pipeline(m: &ArgMatches) -> Result<OutputPipeline, CliError> {
    OutputPipeline::from_matches(m, "sikaru").map_err(invalid)
}

fn list(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let mut listed = crate::sdk::block_on(
        client
            .managed_agents
            .list_managed_agents(value(m, "project"), None),
    )?;
    emit(
        m,
        &listed
            .remove("managedAgents")
            .unwrap_or(Value::Array(vec![])),
    )
}

fn get(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let read = crate::sdk::block_on(client.managed_agents.get(
        value(m, "project"),
        value(m, "agent"),
        None,
    ))?;
    emit(m, &read.managed_agent)
}

fn create(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let request = CreateManagedAgentRequest {
        agent_slug: value(m, "agent").to_owned(),
        display_name: m.get_one::<String>("name").cloned(),
        personal: m.get_flag("personal").then_some(true),
        source: Some(CreateManagedAgentSourceRequest {
            source_kind: Some(CreateManagedAgentSourceRequestSourceKind::AgentDocument),
            document: Some(read_file(m)?),
            ..Default::default()
        }),
        ..Default::default()
    };
    let client = crate::sdk::client(ctx);
    let created = crate::sdk::block_on(client.managed_agents.create_managed_agent(
        value(m, "project"),
        &request,
        None,
    ))?;
    emit(m, &created)
}

pub(super) fn stop(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let cancelled = crate::sdk::block_on(client.runs.cancel(
        value(m, "project"),
        value(m, "run"),
        None,
    ))?;
    emit(m, &cancelled)
}

/// Events print one row each, the table header once; json prints one compact event per line.
pub(super) struct EventPrinter {
    pipeline: OutputPipeline,
    out: Box<dyn std::io::Write>,
    first: bool,
}

impl EventPrinter {
    pub(super) fn new(pipeline: OutputPipeline, out: Box<dyn std::io::Write>) -> Self {
        Self {
            pipeline,
            out,
            first: true,
        }
    }

    fn print(&mut self, event: &RunEvent) -> Result<(), CliError> {
        let row = Value::Array(vec![serde_json::to_value(event).map_err(invalid)?]);
        self.pipeline
            .emit(&mut self.out, &row, true, self.first)
            .map_err(invalid)?;
        self.first = false;
        Ok(())
    }
}

pub(super) fn logs(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let mut printer = EventPrinter::new(pipeline(m)?, Box::new(std::io::stdout()));
    let (project, run) = (value(m, "project"), value(m, "run"));
    if m.get_flag("follow") {
        follow(&client, project, run, &mut printer)
    } else {
        recorded(&client, project, run, &mut printer)
    }
}

/// The events recorded so far, page by page.
fn recorded(
    client: &ApiClient,
    project: &str,
    run: &str,
    printer: &mut EventPrinter,
) -> Result<(), CliError> {
    let mut after = 0;
    loop {
        let query = RunsEventsQueryRequest {
            after: Some(after.to_string()),
            limit: Some(EVENT_PAGE.to_string()),
        };
        let page = crate::sdk::block_on(client.runs.events(project, run, &query, None))?;
        for event in &page.events {
            printer.print(event)?;
        }
        if (page.events.len() as i64) < EVENT_PAGE {
            return Ok(());
        }
        after = page.next_after;
    }
}

/// The server closes idle streams; reconnect after the last printed event until a final one.
fn follow(
    client: &ApiClient,
    project: &str,
    run: &str,
    printer: &mut EventPrinter,
) -> Result<(), CliError> {
    follow_until(client, project, run, printer, &FINAL_EVENTS, None).map(|_| ())
}

/// Pauses that clear on their own; following continues through them.
const SELF_CLEARING_PAUSES: [&str; 2] = ["billing_unavailable", "admission_deferred"];
/// Consecutive stream failures tolerated before the error is reported.
const STREAM_RETRIES: u32 = 5;

fn stops_at(event: &RunEvent, stops: &[&str]) -> bool {
    let reason = event.payload.get("reason").and_then(|value| value.as_str());
    stops.contains(&event.event_type.as_str())
        && !(event.event_type == "run.paused" && reason.is_some_and(|r| SELF_CLEARING_PAUSES.contains(&r)))
}

/// Follows until one of `stops`, returning it, or until `deadline` passes, returning `None`.
/// The deadline is checked after each event and before each reconnect.
pub(super) fn follow_until(
    client: &ApiClient,
    project: &str,
    run: &str,
    printer: &mut EventPrinter,
    stops: &[&str],
    deadline: Option<std::time::Instant>,
) -> Result<Option<RunEvent>, CliError> {
    let expired = || deadline.is_some_and(|end| std::time::Instant::now() >= end);
    let (mut after, mut failures) = (0, 0);
    while !expired() {
        match follow_once(client, project, run, printer, stops, &mut after, deadline) {
            Ok(Some(stop)) => return Ok(stop),
            Ok(None) => failures = 0,
            Err(error) if failures >= STREAM_RETRIES => return Err(error),
            Err(_) => failures += 1,
        }
        // Back off before reconnecting so an idle or failing stream never spins.
        std::thread::sleep(std::time::Duration::from_millis(250 << failures.min(4)));
    }
    Ok(None)
}

/// One stream connection. `Some(stop)` ends following (`Some(None)` at the deadline).
fn follow_once(
    client: &ApiClient,
    project: &str,
    run: &str,
    printer: &mut EventPrinter,
    stops: &[&str],
    after: &mut i64,
    deadline: Option<std::time::Instant>,
) -> Result<Option<Option<RunEvent>>, CliError> {
    let query = StreamEventsQueryRequest {
        after: Some(after.to_string()),
    };
    let mut stream = Box::pin(crate::sdk::block_on(
        client.runs.stream_events(project, run, &query, None),
    )?);
    while let Some(event) = next_event(&mut stream)? {
        printer.print(&event)?;
        *after = event.sequence;
        if stops_at(&event, stops) {
            return Ok(Some(Some(event)));
        }
        if deadline.is_some_and(|end| std::time::Instant::now() >= end) {
            return Ok(Some(None));
        }
    }
    Ok(None)
}

fn next_event(stream: &mut Pin<Box<SseStream<RunEvent>>>) -> Result<Option<RunEvent>, CliError> {
    crate::sdk::block_on(async {
        match stream.next().await {
            None | Some(Err(ApiError::StreamTimeout)) => Ok(None),
            Some(item) => item.map(Some),
        }
    })
}

fn pull(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let draft = crate::sdk::block_on(client.agent_documents.get(
        value(m, "project"),
        value(m, "agent"),
        None,
    ))?;
    std::fs::write(value(m, "file"), draft.document.as_bytes()).map_err(invalid)?;
    emit(m, &draft)
}

fn push(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let request = SaveDocument {
        document: read_file(m)?,
        expected_revision: revision_of(m),
    };
    let client = crate::sdk::client(ctx);
    let saved = crate::sdk::block_on(client.agent_documents.save(
        value(m, "project"),
        value(m, "agent"),
        &request,
        None,
    ))?;
    emit(m, &saved)
}

fn validate(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let request = DocumentInput {
        document: read_file(m)?,
    };
    let client = crate::sdk::client(ctx);
    let validation = crate::sdk::block_on(client.agent_documents.validate(
        value(m, "project"),
        value(m, "agent"),
        &request,
        None,
    ))?;
    emit(m, &validation)
}

fn review(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let request = ReviewDocument {
        revision: revision_of(m),
        expected_live_version_id: m.get_one::<String>("live-version").cloned(),
        ..Default::default()
    };
    let client = crate::sdk::client(ctx);
    let reviewed = crate::sdk::block_on(client.agent_documents.review(
        value(m, "project"),
        value(m, "agent"),
        &request,
        None,
    ))?;
    emit(m, &reviewed)
}

fn publish(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let request = PublishDocument {
        revision: revision_of(m),
        expected_live_version_id: m.get_one::<String>("live-version").cloned(),
        expected_access_digest: Some(value(m, "access-digest").to_owned()),
        acknowledge_widening: Some(m.get_flag("acknowledge-widening")),
        acknowledge_removals: Some(m.get_flag("acknowledge-removals")),
        ..Default::default()
    };
    let client = crate::sdk::client(ctx);
    let published = crate::sdk::block_on(client.agent_documents.publish(
        value(m, "project"),
        value(m, "agent"),
        &request,
        None,
    ))?;
    emit(m, &published)
}

fn snippets(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let snippets = crate::sdk::block_on(client.agent_documents.snippets(
        value(m, "project"),
        value(m, "agent"),
        None,
    ))?;
    emit(m, &snippets)
}
