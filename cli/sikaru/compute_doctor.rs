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
    let mut checks = local_checks(
        workspace,
        std::path::Path::new("/bin/bash"),
        &std::env::var("PATH").unwrap_or_default(),
    );
    let project = project_check(m, ctx).await;
    let ready = checks
        .iter()
        .filter(|c| c["required"] == true)
        .all(|c| c["ok"] == true)
        && project["ok"] == true;
    checks.insert(1, project);
    json!({"status":if ready {"completed"} else {"failed"}, "checks":checks,
        "scope":"Advisory checks on this CLI's local executor and authenticated project read access. Session admission remains authoritative; billing and agent release readiness are not checked."})
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn missing_shell_is_required_but_search_is_optional() {
        let dir = tempfile::tempdir().unwrap();
        let checks = local_checks(dir.path(), &dir.path().join("missing-shell"), "");
        assert_eq!(checks[0]["ok"], true);
        assert_eq!(checks[1]["ok"], false);
        assert_eq!(checks[1]["required"], true);
        assert_eq!(checks[2]["ok"], false);
        assert_eq!(checks[2]["required"], false);
        assert!(checks[2]["help"].as_str().unwrap().contains("fallback"));
    }
}

fn executable(path: &std::path::Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .is_ok_and(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
}

fn local_checks(workspace: &std::path::Path, shell: &std::path::Path, path: &str) -> Vec<Value> {
    let rg = std::env::split_paths(path).any(|p| executable(&p.join("rg")));
    vec![
        json!({"name":"workspace_read_access","ok":std::fs::read_dir(workspace).is_ok(),"required":true,
            "help":"Use a readable directory with --workspace PATH. Write access is not checked."}),
        json!({"name":"local_shell","ok":executable(shell),"required":true,
            "help":"The local executor requires executable /bin/bash."}),
        json!({"name":"local_search","ok":rg,"required":false,
            "help":"Install rg for fast search. If unavailable, use a scoped shell search fallback."}),
        json!({"name":"local_protocol","ok":true,"required":true,"version":sikaru_sdk::api::ReadyInputProtocolVersion::SikaruComputeV1,
            "scope":"This CLI's supported protocol; remote acceptance is checked when attaching."}),
    ]
}

pub fn require_shell() -> anyhow::Result<()> {
    if !executable(std::path::Path::new("/bin/bash")) {
        return Err(super::input::InvalidInput(
            "The local executor requires executable /bin/bash. No work started.",
        )
        .into());
    }
    Ok(())
}
