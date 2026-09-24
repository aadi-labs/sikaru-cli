//! Controller convenience uses generated lifecycle bindings exclusively.
use super::{
    config::Bootstrap,
    journal::Anchor,
    runtime::{self, RunOptions},
    state::{self, call, State},
};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sikaru_sdk::api::*;
use std::{path::PathBuf, time::Duration};
use tokio::{sync::watch, time::Instant};
#[derive(Serialize, Deserialize)]
struct Saved {
    project: String,
    agent: String,
    #[serde(default)]
    model: Option<String>,
    anchor: Anchor,
    key: String,
    session: Option<String>,
    environment: Option<String>,
    attachment: Option<AttachmentView>,
    credential_id: Option<String>,
    run_id: Option<String>,
    clean: bool,
    claim_key: Option<String>,
    claim: Option<ClaimView>,
    executor_started: bool,
    turn_key: Option<String>,
}
pub fn command() -> clap::Command {
    clap::Command::new("exec")
        .alias("chat")
        .about("Work with an agent conversationally; use --print for a single JSON result")
        .after_help("In a terminal, each message continues the same session. Commands: /help, /status, /exit.\nPiped input and --print run one task and exit. Local commands run with your OS permissions.")
        .arg(clap::Arg::new("print").long("print").short('p').action(clap::ArgAction::SetTrue)
            .help("Run one task, emit its JSON result, and exit"))
        .arg(clap::Arg::new("model").long("model").short('m')
            .help("Session default model, e.g. kimi-k3; omitted uses the project default"))
        .arg(
            clap::Arg::new("project")
                .long("project")
                .env("SIKARU_PROJECT")
                .required(true),
        )
        .arg(
            clap::Arg::new("agent")
                .long("agent")
                .env("SIKARU_AGENT")
                .required_unless_present("resume"),
        )
        .arg(
            clap::Arg::new("workspace")
                .long("workspace")
                .visible_alias("cd")
                .short('C')
                .default_value("."),
        )
        .arg(
            clap::Arg::new("prompt")
                .long("prompt")
                .conflicts_with_all(["prompt-file", "task"]),
        )
        .arg(
            clap::Arg::new("prompt-file")
                .long("prompt-file")
                .conflicts_with("task")
                .help("UTF-8 task file, or - for stdin"),
        )
        .arg(
            clap::Arg::new("task")
                .value_name("PROMPT")
                .help("Task to run, or - to read stdin"),
        )
        .arg(
            clap::Arg::new("resume")
                .long("resume")
                .help("Existing state directory; never replaces the workspace"),
        )
        .arg(
            clap::Arg::new("state-dir")
                .long("state-dir")
                .conflicts_with("resume"),
        )
        .arg(
            clap::Arg::new("tenant")
                .long("tenant")
                .default_value("local"),
        )
        .arg(clap::Arg::new("user").long("user").default_value("local"))
        .arg(
            clap::Arg::new("approval-wait")
                .long("approval-wait")
                .default_value("0")
                .value_parser(clap::value_parser!(u64).range(0..=86400)),
        )
        .arg(
            clap::Arg::new("timeout")
                .long("timeout")
                .default_value("3600")
                .value_parser(clap::value_parser!(u64).range(1..=86400)),
        )
}
pub async fn execute(
    m: &clap::ArgMatches,
    ctx: &fern_cli_sdk::openapi::AppContext,
) -> Result<Value> {
    let text = super::input::prompt(m)?;
    let workspace = PathBuf::from(m.get_one::<String>("workspace").unwrap())
        .canonicalize()
        .map_err(|_| {
            super::input::InvalidInput(
                "Workspace does not exist or cannot be read. Use --workspace PATH.",
            )
        })?;
    if m.try_get_one::<bool>("dry-run").ok().flatten() == Some(&true) {
        return Ok(
            json!({"status":"completed","dry_run":true,"workspace":workspace,
            "project":m.get_one::<String>("project"),"agent":m.get_one::<String>("agent"),
            "model":m.get_one::<String>("model"),
            "resume":m.get_one::<String>("resume"),"cleanup":"not_started",
            "usage":{"available":false},"scope":"Input validation only; does not verify remote readiness or resume journal."}),
        );
    }
    super::doctor::require_shell()?;
    let (mut saved, path) = open_state(m, &workspace)?;
    runtime::progress(
        super::chat::is_interactive(m),
        "Opening workspace…",
        json!({"event":"workspace_opened","state_dir":path}),
    );
    let client = crate::sdk::client(ctx);
    let outcome = execute_saved(m, ctx, &client, &mut saved, &path, text).await;
    let mut result = outcome
        .unwrap_or_else(|error| super::diagnostics::failure(&error, "controller_or_resume_failed"));
    result["session_id"] = json!(saved.value.session);
    result["attachment_id"] = json!(saved.value.attachment.as_ref().map(|a| &a.id));
    result["state_dir"] = json!(path);
    saved.value.clean = result["cleanup"] == "confirmed";
    saved.save()?;
    Ok(result)
}
fn open_state(
    m: &clap::ArgMatches,
    workspace: &std::path::Path,
) -> Result<(State<Saved>, PathBuf)> {
    let project = m.get_one::<String>("project").unwrap().clone();
    if let Some(resume) = m.get_one::<String>("resume") {
        return resume_state(m, workspace, &project, resume);
    }
    let key = state::identity();
    let path = m
        .get_one::<String>("state-dir")
        .map(PathBuf::from)
        .unwrap_or_else(|| workspace.join(format!(".sikaru-{key}")));
    let path = if path.is_absolute() {
        path
    } else {
        std::env::current_dir()?.join(path)
    };
    let saved = State::open(
        &path,
        Some(Saved {
            project,
            agent: m.get_one::<String>("agent").unwrap().clone(),
            model: m.get_one::<String>("model").cloned(),
            anchor: Anchor::capture(workspace)?,
            key,
            session: None,
            environment: None,
            attachment: None,
            credential_id: None,
            run_id: None,
            clean: false,
            claim_key: None,
            claim: None,
            executor_started: false,
            turn_key: None,
        }),
    )?;
    Ok((saved, path))
}
fn resume_state(
    m: &clap::ArgMatches,
    workspace: &std::path::Path,
    project: &str,
    resume: &str,
) -> Result<(State<Saved>, PathBuf)> {
    let path = PathBuf::from(resume).canonicalize()?;
    let saved: State<Saved> = State::open(&path, None)?;
    saved.value.anchor.verify()?;
    if saved.value.project != project || saved.value.anchor != Anchor::capture(workspace)? {
        bail!("resume identity mismatch");
    }
    validate_resume(m, &saved.value, &path)?;
    Ok((saved, path))
}
fn validate_resume(m: &clap::ArgMatches, saved: &Saved, path: &std::path::Path) -> Result<()> {
    if let Some(model) = m.get_one::<String>("model") {
        if saved.model.as_ref() != Some(model) {
            bail!(
                "Resumed sessions retain their model. Start a new session to use --model {model}."
            );
        }
    }
    if saved.executor_started
        && (!saved.clean || !path.join("executor").join("journal.jsonl").is_file())
    {
        bail!("resume requires original cleaned journal");
    }
    if let Some(agent) = m.get_one::<String>("agent") {
        if agent != &saved.agent {
            bail!("resume agent mismatch");
        }
    }
    Ok(())
}

async fn provision(m: &clap::ArgMatches, c: &ApiClient, s: &mut State<Saved>) -> Result<()> {
    if s.value.session.is_none() {
        let response = super::diagnostics::session_admission(c.execution_sessions.create(
            &s.value.project,
            &s.value.agent,
            &SessionInput {
                tenant_id: m.get_one::<String>("tenant").unwrap().clone(),
                user_id: m.get_one::<String>("user").unwrap().clone(),
                idempotency_key: Some(s.value.key.clone()),
                model: s.value.model.clone(),
                ..Default::default()
            },
            None,
        ))
        .await?;
        s.value.session = Some(
            response
                .get("session")
                .and_then(|s| s["id"].as_str())
                .context("missing session identity")?
                .to_owned(),
        );
        s.save()?;
    }
    if s.value.environment.is_none() {
        let env = call(c.compute_environments.create(
            &s.value.project,
            &EnvironmentInput {
                environment_slug: format!("local-{}", s.value.key),
                idempotency_key: s.value.key.clone(),
                session_id: s.value.session.clone(),
                product_id: None,
            },
            None,
        ))
        .await?;
        s.value.environment = Some(env.id);
        s.save()?;
    }
    if s.value.attachment.is_some() {
        return Ok(());
    }
    let provenance = WorkspaceProvenance {
        kind: WorkspaceProvenanceKind::ExistingDirectory,
        identity: s.value.key.clone(),
    };
    let a = call(c.compute_attachments.create(
        &s.value.project,
        s.value.session.as_ref().unwrap(),
        &AttachmentInput {
            environment_id: s.value.environment.clone().unwrap(),
            idempotency_key: s.value.key.clone(),
            workspace_provenance: provenance,
            replace_existing: None,
        },
        None,
    ))
    .await?;
    s.value.attachment = Some(a);
    s.save()
}
async fn execute_saved(
    m: &clap::ArgMatches,
    ctx: &fern_cli_sdk::openapi::AppContext,
    c: &ApiClient,
    s: &mut State<Saved>,
    path: &std::path::Path,
    text: Option<String>,
) -> Result<Value> {
    if !s.value.executor_started && text.is_none() {
        bail!("fresh execution requires a prompt");
    }
    provision(m, c, s).await?;
    let a = s.value.attachment.clone().context("missing attachment")?;
    runtime::progress(
        super::chat::is_interactive(m),
        "Connecting session…",
        json!({"event":"attachment_created","session_id":a.session_id,"attachment_id":a.id,"state_dir":path}),
    );
    validate_remote(c, s, &a).await?;
    let bootstrap = acquire_bootstrap(c, s, &a, path).await?;
    let credential_id = bootstrap.credential_id.clone();
    let mut result = run_attached(m, ctx, c, s, &a, text, bootstrap).await?;
    revoke_after_cleanup(c, s, &credential_id, &mut result).await;
    enrich(c, s, &mut result).await;
    Ok(result)
}
async fn acquire_claim(
    c: &ApiClient,
    s: &mut State<Saved>,
    a: &AttachmentView,
) -> Result<ClaimView> {
    if s.value.claim_key.is_none() || s.value.clean {
        s.value.claim_key = Some(state::identity());
        s.value.claim = None;
        s.value.executor_started = false;
        s.value.clean = false;
        s.save()?;
    }
    if s.value.claim.is_none() {
        s.value.claim = Some(
            call(c.compute_attachments.claim(
                &s.value.project,
                &a.id,
                &ClaimInput {
                    idempotency_key: s.value.claim_key.clone().unwrap(),
                },
                None,
            ))
            .await?,
        );
        s.save()?;
    }
    let claim = s.value.claim.clone().unwrap();
    Ok(claim)
}
async fn acquire_bootstrap(
    c: &ApiClient,
    s: &mut State<Saved>,
    a: &AttachmentView,
    path: &std::path::Path,
) -> Result<Bootstrap> {
    let claim = acquire_claim(c, s, a).await?;
    let credential = call(c.compute_attachments.issue_credential(
        &s.value.project,
        &a.id,
        &ExecutorCredentialInput {
            owner_id: claim.owner_id,
            owner_epoch: claim.owner_epoch,
        },
        None,
    ))
    .await?;
    s.value.credential_id = Some(credential.credential_id.clone());
    s.value.clean = false;
    s.value.executor_started = true;
    s.save()?;
    Ok(Bootstrap {
        project_id: s.value.project.clone(),
        session_id: a.session_id.clone(),
        attachment_id: a.id.clone(),
        owner_epoch: claim.owner_epoch,
        workspace_generation: a.workspace_generation.clone(),
        journal_id: a.journal_id.clone(),
        credential_id: credential.credential_id.clone(),
        token: credential.token,
        workspace_provenance: a.workspace_provenance.clone(),
        workspace: s.value.anchor.path().to_owned(),
        state_dir: path.join("executor"),
        command_timeout_seconds: 120,
    })
}
async fn run_attached(
    m: &clap::ArgMatches,
    ctx: &fern_cli_sdk::openapi::AppContext,
    c: &ApiClient,
    s: &mut State<Saved>,
    a: &AttachmentView,
    text: Option<String>,
    bootstrap: Bootstrap,
) -> Result<Value> {
    let (stop, receiver) = watch::channel(false);
    let (admitted, admission) = watch::channel(text.is_none());
    let options = RunOptions {
        approval_wait: Duration::from_secs(*m.get_one::<u64>("approval-wait").unwrap()),
        timeout: Some(Duration::from_secs(*m.get_one::<u64>("timeout").unwrap())),
        stop: Some(receiver),
        admission: Some(admission),
        interactive: super::chat::is_interactive(m),
    };
    let http = ctx.http_config().build_client()?;
    let serving = async {
        let result =
            runtime::serve_with_options(bootstrap, ctx.effective_base_url(), http, options).await;
        let _ = stop.send(true);
        result
    };
    let submission = async {
        let result =
            submit_when_ready(c, s, &a, text, stop.clone(), super::chat::is_interactive(m)).await;
        if result.is_ok() {
            let _ = admitted.send(true);
        }
        result
    };
    let (served, submitted) = tokio::join!(serving, submission);
    let served_failed = served.is_err();
    let mut result = served.unwrap_or_else(|_| state::failure("executor_startup_failed"));
    if submitted.is_err() && !served_failed {
        result["reason"] = json!("turn_submission_unconfirmed");
        result["status"] = json!("recovery_required");
    }
    Ok(result)
}
async fn revoke_after_cleanup(
    c: &ApiClient,
    s: &State<Saved>,
    credential_id: &str,
    result: &mut Value,
) {
    if result["cleanup"] == "confirmed" && result["status"] != "recovery_required" {
        result["credential_revoked"] = json!(call(c.compute_credentials.revoke(
            &s.value.project,
            credential_id,
            None
        ))
        .await
        .is_ok());
    }
}

async fn validate_remote(c: &ApiClient, s: &State<Saved>, a: &AttachmentView) -> Result<()> {
    let remote = call(c.compute_attachments.get(&s.value.project, &a.id, None)).await?;
    if remote.journal_id != a.journal_id
        || remote.workspace_generation != a.workspace_generation
        || remote.workspace_provenance != a.workspace_provenance
        || remote.session_id != a.session_id
    {
        bail!("remote workspace changed");
    }
    Ok(())
}
async fn submit_when_ready(
    c: &ApiClient,
    s: &mut State<Saved>,
    a: &AttachmentView,
    text: Option<String>,
    stop: watch::Sender<bool>,
    interactive: bool,
) -> Result<()> {
    let outcome = submit(c, s, a, text, stop.subscribe(), interactive).await;
    if outcome.is_err() {
        let _ = stop.send(true);
    }
    outcome
}
async fn submit(
    c: &ApiClient,
    s: &mut State<Saved>,
    a: &AttachmentView,
    text: Option<String>,
    stopped: watch::Receiver<bool>,
    interactive: bool,
) -> Result<()> {
    let Some(text) = text else {
        return Ok(());
    };
    wait_ready(c, s, a, stopped).await?;
    if s.value.turn_key.is_none() || s.value.run_id.is_some() {
        s.value.turn_key = Some(state::identity());
        s.value.run_id = None;
        s.save()?;
    }
    let response = call(c.execution_sessions.append_turn(
        &s.value.project,
        &a.session_id,
        &TurnInput {
            idempotency_key: s.value.turn_key.clone().unwrap(),
            input: serde_json::from_value(json!({"goal":text}))?,
            compute_attachment_id: Some(a.id.clone()),
            capability_grants: Some(vec!["compute.execute".into()]),
            ..Default::default()
        },
        None,
    ))
    .await?;
    s.value.run_id = Some(
        response
            .get("run")
            .and_then(|r| r["id"].as_str())
            .context("missing run identity")?
            .to_owned(),
    );
    s.save()?;
    runtime::progress(
        interactive,
        "Working…",
        json!({"event":"turn_submitted","session_id":a.session_id,"attachment_id":a.id,"run_id":s.value.run_id}),
    );
    Ok(())
}
async fn wait_ready(
    c: &ApiClient,
    s: &State<Saved>,
    a: &AttachmentView,
    stopped: watch::Receiver<bool>,
) -> Result<()> {
    let end = Instant::now() + Duration::from_secs(180);
    loop {
        if *stopped.borrow() {
            bail!("executor_stopped_before_turn");
        }
        let status = call(c.compute_attachments.get(&s.value.project, &a.id, None)).await?;
        if status.status == AttachmentViewStatus::Ready {
            break;
        }
        if Instant::now() >= end {
            bail!("executor_startup_timeout");
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    Ok(())
}

async fn enrich(c: &ApiClient, s: &State<Saved>, result: &mut Value) {
    result["result"] = Value::Null;
    let run = result["execution"]["run_id"]
        .as_str()
        .map(str::to_owned)
        .or_else(|| s.value.run_id.clone());
    let Some(run) = run else {
        return;
    };
    result["run_id"] = json!(run);
    if let Ok(value) = call(c.runs.get(&s.value.project, &run, None)).await {
        result["result"] = json!(value);
        if value.usage_summary.is_some() || value.cost_summary.is_some() {
            result["usage"] =
                json!({"available":true,"usage":value.usage_summary,"cost":value.cost_summary});
        }
    }
    result["final_output"] = json!({"available":false});
    let fetched = tokio::time::timeout(
        Duration::from_secs(30),
        product_output(c, &s.value.project, &run),
    )
    .await;
    if let Ok(Ok((output, events))) = fetched {
        result["final_output"] = output;
        result["events"] = events;
    }
}
async fn product_output(c: &ApiClient, project: &str, run: &str) -> Result<(Value, Value)> {
    let mut after = 0;
    for _ in 0..100 {
        let page = call(c.runs.events(
            project,
            run,
            &EventsQueryRequest {
                after: Some(after.to_string()),
                limit: Some("100".into()),
            },
            None,
        ))
        .await?;
        if let Some(event) = page
            .events
            .iter()
            .rev()
            .find(|e| e.event_type == "run.completed")
        {
            let output = event.payload.get("output").cloned().unwrap_or(Value::Null);
            return Ok((
                json!({"available":!output.is_null(),"output":output}),
                json!(page),
            ));
        }
        if page.events.is_empty() || page.next_after <= after {
            return Ok((json!({"available":false}), json!(page)));
        }
        after = page.next_after;
    }
    bail!("product result event bound exceeded")
}
