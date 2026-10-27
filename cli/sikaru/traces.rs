//! `sikaru traces`: upload a trace file and read a run's trajectory. The upload's format comes
//! from the file's shape: OTLP `resourceSpans`, ATIF trajectories, or OpenInference spans.
use super::agent::{emit, invalid, project, value};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use serde::de::DeserializeOwned;
use serde_json::Value;
use sikaru_sdk::api::*;

pub fn install(app: CliApp) -> CliApp {
    app.command_under(
        &["traces"],
        upload_command(),
        OpenApiBinding::handler(upload),
    )
    .command_under(&["traces"], get_command(), OpenApiBinding::handler(get))
}

fn upload_command() -> Command {
    Command::new("upload")
        .about("Upload a JSON or JSONL trace file: OTLP, ATIF or OpenInference spans")
        .arg(project())
        .arg(Arg::new("file").required(true).help("Trace file"))
}

fn get_command() -> Command {
    Command::new("get")
        .about("Show a run's trajectory")
        .arg(project())
        .arg(Arg::new("run").required(true).help("Run id"))
}

#[derive(Clone, Copy, PartialEq)]
enum Kind {
    Otlp,
    Atif,
    Spans,
}

fn kind(record: &Value) -> Option<Kind> {
    let has = |key| record.get(key).is_some();
    if has("resourceSpans") {
        Some(Kind::Otlp)
    } else if ["steps", "schema_version", "trajectory", "trajectories"]
        .into_iter()
        .any(has)
    {
        Some(Kind::Atif)
    } else if has("trace_id") && has("span_id") {
        Some(Kind::Spans)
    } else {
        None
    }
}

/// One JSON document (an array is its items), else one JSON value per non-blank line.
fn records(text: &str) -> Result<Vec<Value>, CliError> {
    match serde_json::from_str::<Value>(text) {
        Ok(Value::Array(items)) => Ok(items),
        Ok(value) => Ok(vec![value]),
        Err(_) => text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| serde_json::from_str(line).map_err(invalid))
            .collect(),
    }
}

/// Every record must be the same recognized format.
fn file_kind(records: &[Value]) -> Result<Kind, CliError> {
    let kinds: Vec<Option<Kind>> = records.iter().map(kind).collect();
    match kinds.first() {
        Some(Some(first)) if kinds.iter().all(|k| *k == Some(*first)) => Ok(*first),
        Some(Some(_)) => Err(invalid("The trace file mixes formats; upload one format per file")),
        _ => Err(invalid(
            "Unrecognized trace file: expected OTLP resourceSpans, ATIF trajectories, or spans with trace_id and span_id",
        )),
    }
}

fn convert<T: DeserializeOwned>(
    values: impl IntoIterator<Item = Value>,
) -> Result<Vec<T>, CliError> {
    values
        .into_iter()
        .map(|value| serde_json::from_value(value).map_err(invalid))
        .collect()
}

fn resource_spans(record: Value) -> Vec<Value> {
    match record.get("resourceSpans") {
        Some(Value::Array(spans)) => spans.clone(),
        _ => vec![],
    }
}

/// A record wrapping `trajectory` or `trajectories` contributes those; otherwise it is one.
fn trajectories(record: Value) -> Vec<Value> {
    if let Some(one) = record.get("trajectory") {
        return vec![one.clone()];
    }
    if let Some(Value::Array(many)) = record.get("trajectories") {
        return many.clone();
    }
    vec![record]
}

fn request(project: &str, records: Vec<Value>) -> Result<TraceStreamRequest, CliError> {
    let mut request = TraceStreamRequest {
        dataset: None,
        format: TraceStreamRequestFormat::OpeninferenceV1,
        metadata: TraceMetadata {
            project_id: project.to_owned(),
            account_id: project.to_owned(),
            source: "cli".to_owned(),
            ..Default::default()
        },
        resource_spans: None,
        spans: None,
        trajectories: None,
        trajectory: None,
    };
    match file_kind(&records)? {
        Kind::Otlp => {
            request.format = TraceStreamRequestFormat::OpentelemetryOtlpV1;
            request.resource_spans = Some(convert(records.into_iter().flat_map(resource_spans))?);
        }
        Kind::Atif => {
            request.format = TraceStreamRequestFormat::HarborAtifV1;
            request.trajectories = Some(convert(records.into_iter().flat_map(trajectories))?);
        }
        Kind::Spans => request.spans = Some(convert(records)?),
    }
    Ok(request)
}

fn upload(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let text = std::fs::read_to_string(value(m, "file")).map_err(invalid)?;
    let request = request(value(m, "project"), records(&text)?)?;
    let client = crate::sdk::client(ctx);
    let accepted = crate::sdk::block_on(
        client
            .trace_streams
            .stream_openinference_spans(&request, None),
    )?;
    emit(m, &accepted)
}

fn get(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let trajectory = crate::sdk::block_on(client.runs.get_trajectory(
        value(m, "project"),
        value(m, "run"),
        None,
    ))?;
    emit(m, &trajectory)
}
