//! `sikaru environment`: the datasets an agent is evaluated on, with their check results. The
//! verbs keep their names when RL environments replace datasets as the backing resource.
use super::agent::{emit, invalid, project, value};
use super::eval::{
    check_results, dataset_command, dataset_version, dataset_version_of, idempotency_key,
};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use serde_json::json;
use sikaru_sdk::api::*;
use std::path::Path;

/// `info` shows the first page of examples.
const EXAMPLE_PAGE: i64 = 100;

type Handler = fn(&ArgMatches, &AppContext) -> Result<(), CliError>;

pub fn install(app: CliApp) -> CliApp {
    let verbs: [(Command, Handler); 4] = [
        (list_command(), list),
        (info_command(), info),
        (push_command(), push),
        (pull_command(), pull),
    ];
    verbs.into_iter().fold(app, |app, (command, handler)| {
        app.command_under(&["environment"], command, OpenApiBinding::handler(handler))
    })
}

fn list_command() -> Command {
    Command::new("list")
        .about("List the project's datasets")
        .arg(project())
}

fn info_command() -> Command {
    dataset_version(dataset_command(
        "info",
        "Show a dataset with its examples, versions and check results",
    ))
}

fn push_command() -> Command {
    Command::new("push")
        .about("Create a dataset from a .csv or .jsonl file")
        .arg(project())
        .arg(Arg::new("file").required(true).help("Examples file"))
        .arg(
            Arg::new("name")
                .long("name")
                .help("Dataset name; defaults to the file name without its extension"),
        )
        .arg(
            Arg::new("purpose")
                .long("purpose")
                .value_parser(["eval", "training", "experiment"]),
        )
}

fn pull_command() -> Command {
    dataset_version(dataset_command(
        "pull",
        "Write a dataset's examples to a JSONL file",
    ))
    .arg(
        Arg::new("file")
            .long("file")
            .help("Destination; defaults to <dataset>.jsonl"),
    )
}

fn list(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let listed = crate::sdk::block_on(client.datasets.list_datasets(
        value(m, "project"),
        &Default::default(),
        None,
    ))?;
    emit(m, &listed.datasets)
}

fn info(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let client = crate::sdk::client(ctx);
    let (project, dataset, version) = (
        value(m, "project"),
        value(m, "dataset"),
        dataset_version_of(m),
    );
    let read = crate::sdk::block_on(client.datasets.get_dataset(project, dataset, None))?;
    let page = ListExamplesQueryRequest {
        version,
        limit: Some(EXAMPLE_PAGE),
        offset: None,
    };
    let examples =
        crate::sdk::block_on(client.datasets.list_examples(project, dataset, &page, None))?;
    let versions = crate::sdk::block_on(client.datasets.list_versions(project, dataset, None))?;
    let checks = check_results(&client, project, dataset, version)?;
    emit(
        m,
        &json!({
            "dataset": read.dataset,
            "examples": examples,
            "versions": versions.versions,
            "checks": checks,
        }),
    )
}

fn purpose(name: &str) -> CreateDatasetPurpose {
    match name {
        "training" => CreateDatasetPurpose::Training,
        "experiment" => CreateDatasetPurpose::Experiment,
        _ => CreateDatasetPurpose::Eval,
    }
}

/// CSV by extension; anything else is uploaded as JSONL in the export shape.
fn upload_format(file: &Path) -> UploadRequestFormat {
    match file.extension().and_then(|ext| ext.to_str()) {
        Some(ext) if ext.eq_ignore_ascii_case("csv") => UploadRequestFormat::Csv,
        _ => UploadRequestFormat::Jsonl,
    }
}

fn push(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let file = Path::new(value(m, "file"));
    let content = std::fs::read_to_string(file).map_err(invalid)?;
    let name = match m.get_one::<String>("name") {
        Some(name) => name.clone(),
        None => file
            .file_stem()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
    };
    let request = CreateDataset {
        name,
        purpose: m.get_one::<String>("purpose").map(|p| purpose(p)),
        idempotency_key: Some(idempotency_key()),
        description: None,
    };
    let client = crate::sdk::client(ctx);
    let project = value(m, "project");
    let created = crate::sdk::block_on(client.datasets.create_dataset(project, &request, None))?;
    let upload = UploadRequest {
        content,
        format: upload_format(file),
        idempotency_key: idempotency_key(),
    };
    let uploaded = crate::sdk::block_on(client.datasets.upload_examples(
        project,
        &created.dataset.id,
        &upload,
        None,
    ))?;
    emit(m, &json!({"dataset": created.dataset, "upload": uploaded}))
}

fn pull(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let dataset = value(m, "dataset");
    let file = match m.get_one::<String>("file") {
        Some(file) => file.clone(),
        None => format!("{dataset}.jsonl"),
    };
    let query = ExportDatasetQueryRequest {
        version: dataset_version_of(m),
    };
    let client = crate::sdk::client(ctx);
    let bytes = crate::sdk::block_on(async {
        let stream = client
            .datasets
            .export_dataset(value(m, "project"), dataset, &query, None)
            .await?;
        stream.collect().await
    })?;
    std::fs::write(&file, &bytes).map_err(invalid)?;
    emit(
        m,
        &json!({"dataset": dataset, "file": file, "bytes": bytes.len()}),
    )
}
