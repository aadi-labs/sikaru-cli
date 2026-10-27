//! `sikaru eval`: run an environment's dataset as Checks against an agent's live production
//! version, then read the results. Each started example is one run, so `logs` and `stop` take a
//! run id that `eval run` prints. Refusals (personal agent, unavailable release, evaluation off)
//! are the gateway's own messages.
use super::agent::{self, emit, project, value};
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use serde_json::{json, Value};
use sikaru_sdk::api::*;
use std::time::Duration;

const POLL: Duration = Duration::from_secs(2);

type Handler = fn(&ArgMatches, &AppContext) -> Result<(), CliError>;

pub fn install(app: CliApp) -> CliApp {
    let verbs: [(Command, Handler); 6] = [
        (run_command(), run),
        (list_command(), list),
        (get_command(), get),
        (samples_command(), samples),
        (
            agent::logs_command().about("Print one example run's events"),
            agent::logs,
        ),
        (
            agent::stop_command().about("Cancel one example run"),
            agent::stop,
        ),
    ];
    verbs.into_iter().fold(app, |app, (command, handler)| {
        app.command_under(&["eval"], command, OpenApiBinding::handler(handler))
    })
}

/// A fresh key per start: rerunning `eval run` starts new Checks rather than replaying old ones.
pub(super) fn idempotency_key() -> String {
    format!("{:032x}", rand::random::<u128>())
}

pub(super) fn dataset_command(name: &'static str, about: &'static str) -> Command {
    Command::new(name)
        .about(about)
        .arg(project())
        .arg(Arg::new("dataset").required(true).help("Dataset id"))
}

pub(super) fn dataset_version(command: Command) -> Command {
    command.arg(
        Arg::new("dataset-version")
            .long("dataset-version")
            .value_parser(clap::value_parser!(i64))
            .help("Dataset version; omit for the latest"),
    )
}

pub(super) fn dataset_version_of(m: &ArgMatches) -> Option<i64> {
    m.get_one::<i64>("dataset-version").copied()
}

fn agent_flag(required: bool) -> Arg {
    Arg::new("agent")
        .long("agent")
        .short('a')
        .required(required)
        .help("Agent slug")
}

fn run_command() -> Command {
    dataset_version(dataset_command(
        "run",
        "Run every eligible example against the agent's live production version",
    ))
    .arg(agent_flag(true))
    .arg(
        Arg::new("follow")
            .long("follow")
            .short('f')
            .action(ArgAction::SetTrue)
            .help("Wait until every started check settles, then print the summary"),
    )
}

fn list_command() -> Command {
    Command::new("list")
        .about("List datasets that have been run as Checks")
        .arg(project())
}

fn get_command() -> Command {
    dataset_version(dataset_command(
        "get",
        "Show pass rates per agent version for a dataset",
    ))
    .arg(agent_flag(false))
}

fn samples_command() -> Command {
    dataset_version(dataset_command(
        "samples",
        "List failed examples with their run ids; passing examples are not listed",
    ))
    .arg(agent_flag(false))
}

pub(super) fn check_results(
    client: &ApiClient,
    project: &str,
    dataset: &str,
    version: Option<i64>,
) -> Result<Vec<DatasetCheckGroup>, CliError> {
    let query = ListDatasetCheckResultsQueryRequest { version };
    let listed = crate::sdk::block_on(
        client
            .datasets
            .list_dataset_check_results(project, dataset, &query, None),
    )?;
    Ok(listed.results)
}

fn run(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let (project, dataset) = (value(m, "project"), value(m, "dataset"));
    let follow = m.get_flag("follow");
    // Results accumulate per agent and dataset version, so following counts from a baseline.
    let before = match follow {
        true => check_results(&client, project, dataset, None)?,
        false => vec![],
    };
    let request = StartDatasetChecks {
        agent_slug: value(m, "agent").to_owned(),
        dataset_version: dataset_version_of(m),
        idempotency_key: idempotency_key(),
        version: None,
    };
    let started = crate::sdk::block_on(
        client
            .datasets
            .start_dataset_checks(project, dataset, &request, None),
    )?;
    let expected = started.checks.iter().filter(|c| c.run_id.is_some()).count() as i64;
    if !follow || expected == 0 {
        return emit(m, &started);
    }
    let run = StartedRun {
        project,
        dataset,
        agent: value(m, "agent"),
        started: &started,
    };
    announce(&started);
    let baseline = run
        .find(before)
        .map_or(0, |group| group.passed + group.failed);
    emit(m, &run.wait(&client, baseline + expected)?)
}

/// While following, stdout holds only the summary, so each example's run id (for `eval logs`)
/// or the reason it did not start goes to stderr first.
fn announce(started: &DatasetChecksStarted) {
    eprintln!(
        "Checks on {} (dataset version {}); waiting for results",
        started.agent_version, started.dataset_version
    );
    for check in &started.checks {
        let detail = check.run_id.as_deref().or(check.reason.as_deref());
        eprintln!(
            "  {}  {}",
            check.example_id,
            detail.unwrap_or("not started")
        );
    }
}

/// The agent version and dataset version a start ran, which name its results group.
struct StartedRun<'a> {
    project: &'a str,
    dataset: &'a str,
    agent: &'a str,
    started: &'a DatasetChecksStarted,
}

impl StartedRun<'_> {
    fn find(&self, groups: Vec<DatasetCheckGroup>) -> Option<DatasetCheckGroup> {
        groups.into_iter().find(|group| {
            group.agent_slug == self.agent
                && group.agent_version.as_deref() == Some(self.started.agent_version.as_str())
                && group.dataset_version == self.started.dataset_version
        })
    }

    /// Polls until nothing is running and `settled` checks have passed or failed. Checks that
    /// never started are not waited on.
    fn wait(&self, client: &ApiClient, settled: i64) -> Result<DatasetCheckGroup, CliError> {
        loop {
            let groups = check_results(
                client,
                self.project,
                self.dataset,
                Some(self.started.dataset_version),
            )?;
            let done = self
                .find(groups)
                .filter(|g| g.running == 0 && g.passed + g.failed >= settled);
            if let Some(group) = done {
                return Ok(group);
            }
            tokio::task::block_in_place(|| std::thread::sleep(POLL));
        }
    }
}

fn list(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let listed = crate::sdk::block_on(client.datasets.list_datasets(
        value(m, "project"),
        &Default::default(),
        None,
    ))?;
    let checked: Vec<Dataset> = listed
        .datasets
        .into_iter()
        .filter(|d| d.used_by.as_ref().is_some_and(|used| used.checks > 0))
        .collect();
    emit(m, &checked)
}

/// The dataset's result groups, narrowed to `-a` when given.
fn agent_results(m: &ArgMatches, ctx: &AppContext) -> Result<Vec<DatasetCheckGroup>, CliError> {
    let client = crate::sdk::client(ctx);
    let groups = check_results(
        &client,
        value(m, "project"),
        value(m, "dataset"),
        dataset_version_of(m),
    )?;
    let agent = m.get_one::<String>("agent");
    Ok(groups
        .into_iter()
        .filter(|g| agent.is_none_or(|slug| &g.agent_slug == slug))
        .collect())
}

fn get(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    emit(m, &agent_results(m, ctx)?)
}

fn samples(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let rows: Vec<Value> = agent_results(m, ctx)?
        .iter()
        .flat_map(|group| group.failures.iter().map(move |f| failure_row(group, f)))
        .collect();
    emit(m, &rows)
}

fn failure_row(group: &DatasetCheckGroup, failure: &DatasetCheckFailure) -> Value {
    json!({
        "agentSlug": group.agent_slug,
        "agentVersion": group.agent_version,
        "datasetVersion": group.dataset_version,
        "exampleId": failure.example_id,
        "checkName": failure.check_name,
        "runId": failure.run_id,
        "reason": failure.reason,
    })
}
