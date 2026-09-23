//! Read-only checks for the prerequisites of native execution.
use super::{state::call, transport::TransportFailure};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};

pub fn command() -> Command {
    Command::new("doctor")
        .about("Check workspace and authenticated project access without starting a run")
        .arg(Arg::new("project").long("project").env("SIKARU_PROJECT"))
        .arg(Arg::new("workspace").long("workspace").default_value("."))
}

pub async fn execute(m: &ArgMatches, ctx: &AppContext) -> Value {
    let workspace = std::path::Path::new(m.get_one::<String>("workspace").unwrap());
    let local = workspace.is_dir();
    let project = project_check(m, ctx).await;
    json!({"status":if local && project["ok"] == true {"completed"} else {"failed"},
        "checks":[{"name":"workspace","ok":local,"help":"Use an existing directory with --workspace PATH."},project],
        "scope":"Checks workspace existence and project read access. Does not verify billing, agent readiness, or sandbox isolation."})
}

async fn project_check(m: &ArgMatches, ctx: &AppContext) -> Value {
    let Some(project) = m.get_one::<String>("project") else {
        return json!({"name":"project_access","ok":false,"help":"Set SIKARU_PROJECT or pass --project PROJECT. Run sikaru auth login to configure credentials."});
    };
    let client = crate::sdk::client(ctx);
    let result = call(client.managed_agents.list_managed_agents(project, None)).await;
    match result {
        Ok(_) => json!({"name":"project_access","ok":true}),
        Err(error) => {
            json!({"name":"project_access","ok":false,"help":advice(&error)})
        }
    }
}

fn advice(error: &anyhow::Error) -> &'static str {
    match error.downcast_ref::<TransportFailure>() {
        Some(TransportFailure::Rejected(401)) => "Run sikaru auth login, then retry doctor.",
        Some(TransportFailure::Rejected(403)) => "Check your project membership and API key scopes.",
        Some(TransportFailure::Rejected(404)) => "Project not found. Check --project or SIKARU_PROJECT.",
        _ => "Project access could not be verified. Check connectivity, --base-url, and service availability. No run was started.",
    }
}
