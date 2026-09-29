//! Document authoring over the generated public client. The server owns parsing.
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use serde_json::Value;
use sikaru_sdk::api::*;

pub fn install(mut app: CliApp) -> CliApp {
    for name in ["pull", "push", "validate", "review", "publish", "snippets"] {
        app = app.command_under(
            &["agents"],
            command(name),
            OpenApiBinding::handler(|m, ctx| {
                let result = tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(execute(
                        value(m, "document-action"),
                        m,
                        ctx,
                    ))
                })
                .map_err(|error| CliError::Validation(error.to_string()))?;
                println!(
                    "{}",
                    serde_json::to_string_pretty(&result)
                        .map_err(|e| CliError::Validation(e.to_string()))?
                );
                Ok(())
            }),
        );
    }
    app
}

fn command(name: &'static str) -> Command {
    let command = Command::new(name)
        .arg(
            Arg::new("document-action")
                .long("document-action")
                .hide(true)
                .default_value(name)
                .value_parser([name]),
        )
        .about("Author a hosted agent document through the public API")
        .arg(
            Arg::new("project")
                .long("project")
                .env("SIKARU_PROJECT")
                .required(true),
        )
        .arg(
            Arg::new("agent")
                .long("agent")
                .env("SIKARU_AGENT")
                .required(true),
        );
    match name {
        "pull" | "push" | "validate" => file_command(command, name),
        "review" => command
            .arg(Arg::new("revision").long("revision").required(true).value_parser(clap::value_parser!(i64)))
            .arg(Arg::new("live-version").long("live-version").help("Expected live version; omit before first publication")),
        "publish" => command
            .arg(Arg::new("access-digest").long("access-digest").required(true)
                .help("accessDigest returned by the access review you approved"))
            .arg(
                Arg::new("revision")
                    .long("revision")
                    .required(true)
                    .value_parser(clap::value_parser!(i64)),
            )
            .arg(
                Arg::new("live-version")
                    .long("live-version")
                    .help("Expected live version; omit for the first publication"),
            )
            .arg(
                Arg::new("acknowledge-widening")
                    .long("acknowledge-widening")
                    .action(ArgAction::SetTrue),
            )
            .arg(
                Arg::new("acknowledge-removals")
                    .long("acknowledge-removals")
                    .action(ArgAction::SetTrue),
            ),
        _ => command,
    }
}

fn file_command(command: Command, name: &str) -> Command {
    let command = command.arg(Arg::new("file").long("file").default_value("agent.md"));
    if name == "push" {
        command.arg(
            Arg::new("revision")
                .long("revision")
                .required(true)
                .value_parser(clap::value_parser!(i64)),
        )
    } else {
        command
    }
}

fn value<'a>(m: &'a ArgMatches, name: &str) -> &'a str {
    m.get_one::<String>(name)
        .expect("clap validates required arguments")
}

async fn execute(name: &str, m: &ArgMatches, ctx: &AppContext) -> anyhow::Result<Value> {
    let client = crate::sdk::client(ctx);
    let project = value(m, "project");
    let agent = value(m, "agent");
    match name {
        "pull" => {
            let draft = client.agent_documents.get(project, agent, None).await?;
            std::fs::write(value(m, "file"), draft.document.as_bytes())?;
            Ok(serde_json::to_value(draft)?)
        }
        "push" => Ok(serde_json::to_value(
            client
                .agent_documents
                .save(
                    project,
                    agent,
                    &SaveDocument {
                        document: std::fs::read_to_string(value(m, "file"))?,
                        expected_revision: *m.get_one::<i64>("revision").unwrap(),
                    },
                    None,
                )
                .await?,
        )?),
        "validate" => Ok(serde_json::to_value(
            client
                .agent_documents
                .validate(
                    project,
                    agent,
                    &DocumentInput {
                        document: std::fs::read_to_string(value(m, "file"))?,
                    },
                    None,
                )
                .await?,
        )?),
        "review" => Ok(serde_json::to_value(
            client.agent_documents.review(project, agent, &ReviewDocument {
                revision: *m.get_one::<i64>("revision").unwrap(),
                expected_live_version_id: m.get_one::<String>("live-version").cloned(),
                ..Default::default()
            }, None).await?,
        )?),
        "publish" => Ok(serde_json::to_value(
            client
                .agent_documents
                .publish(
                    project,
                    agent,
                    &PublishDocument {
                        revision: *m.get_one::<i64>("revision").unwrap(),
                        expected_live_version_id: m.get_one::<String>("live-version").cloned(),
                        expected_access_digest: Some(value(m, "access-digest").to_owned()),
                        acknowledge_widening: Some(m.get_flag("acknowledge-widening")),
                        acknowledge_removals: Some(m.get_flag("acknowledge-removals")),
                        ..Default::default()
                    },
                    None,
                )
                .await?,
        )?),
        "snippets" => Ok(serde_json::to_value(
            client
                .agent_documents
                .snippets(project, agent, None)
                .await?,
        )?),
        _ => anyhow::bail!("Unsupported document action"),
    }
}
