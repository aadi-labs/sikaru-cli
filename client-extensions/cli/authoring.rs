//! Customer source packaging and workflows over the generated API executor.
use base64::{engine::general_purpose::STANDARD, Engine};
use clap::{Arg, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
};
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    io::{Read, Write},
    path::{Component, Path},
};

fn invalid(message: impl ToString) -> CliError {
    CliError::Validation(message.to_string())
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Manifest {
    name: String,
    sources: Vec<Source>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Source {
    path: String,
    kind: String,
}

fn safe_bytes(root: &Path, relative: &str) -> Result<Vec<u8>, CliError> {
    if relative.contains('\\')
        || relative
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
        || Path::new(relative)
            .components()
            .any(|p| !matches!(p, Component::Normal(_)))
    {
        return Err(invalid("Source paths must be relative without traversal"));
    }
    let mut target = root.to_path_buf();
    for part in Path::new(relative).components() {
        target.push(part);
        if fs::symlink_metadata(&target)
            .map_err(invalid)?
            .file_type()
            .is_symlink()
        {
            return Err(invalid(
                "Symlinks are not permitted in authored source paths",
            ));
        }
    }
    let mut options = fs::OpenOptions::new();
    options.read(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW);
    }
    let file = options.open(&target).map_err(invalid)?;
    let metadata = file.metadata().map_err(invalid)?;
    if !metadata.is_file() || metadata.len() > 1_000_000 {
        return Err(invalid("Source must be a file smaller than 1 MB"));
    }
    let mut content = Vec::new();
    file.take(1_000_001).read_to_end(&mut content).map_err(invalid)?;
    if content.len() > 1_000_000 {
        return Err(invalid("Source exceeds 1 MB"));
    }
    Ok(content)
}
fn safe_file(root: &Path, relative: &str) -> Result<String, CliError> {
    let content = String::from_utf8(safe_bytes(root, relative)?).map_err(invalid)?;
    if content.contains('\0') {
        return Err(invalid("Source contains a NUL byte"));
    }
    Ok(content)
}

fn validate_name(name: &str) -> Result<(), CliError> {
    if name.is_empty()
        || name.len() > 64
        || !name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        || !name.as_bytes()[0].is_ascii_alphanumeric()
    {
        return Err(invalid("Agent name must start with a lowercase letter or digit and contain only lowercase letters, digits and hyphens (up to 64 characters)"));
    }
    Ok(())
}

pub fn package(root: &Path) -> Result<Value, CliError> {
    let manifest: Manifest =
        serde_json::from_str(&safe_file(root, "sikaru.json")?).map_err(invalid)?;
    validate_name(&manifest.name)?;
    if manifest.sources.is_empty() || manifest.sources.len() > 100 {
        return Err(invalid("Specify between 1 and 100 sources"));
    }
    let mut seen = BTreeSet::new();
    let mut sources = Vec::new();
    let mut instructions = false;
    for source in manifest.sources {
        let parts: Vec<_> = source.path.split('/').collect();
        let valid_name = |name: &str| {
            !name.is_empty()
                && name.as_bytes()[0].is_ascii_alphanumeric()
                && name
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        };
        let named = parts.first() == Some(&"agents");
        if named && (parts.len() < 3 || !valid_name(parts[1])) {
            return Err(invalid("Invalid named agent path"));
        }
        let scoped = if named { &parts[2..] } else { &parts[..] };
        let allowed = match source.kind.as_str() {
            "agent_md" => scoped == ["instructions.md"] || (!named && scoped == ["AGENTS.md"]),
            "agent_skill" => scoped.first() == Some(&"skills") && source.path.ends_with(".md"),
            "skill_asset" => {
                scoped.len() >= 4
                    && scoped[0] == "skills"
                    && valid_name(scoped[1])
                    && matches!(scoped[2], "scripts" | "references" | "assets")
            }
            "eval_md" => !named && scoped.first() == Some(&"evals") && source.path.ends_with(".md"),
            _ => false,
        };
        if !allowed || !seen.insert(source.path.clone()) {
            return Err(invalid(
                "Invalid, duplicate, or unsupported authored source path/kind",
            ));
        }
        if source.kind == "skill_asset" {
            let content = STANDARD.encode(safe_bytes(root, &source.path)?);
            sources.push(json!({"path":source.path,"kind":source.kind,"content":content,"encoding":"base64"}));
        } else {
            let content = safe_file(root, &source.path)?;
            if source.kind == "agent_md" && !named && !content.trim().is_empty() {
                instructions = true;
            }
            sources.push(json!({"path":source.path,"kind":source.kind,"content":content}));
        }
    }
    for source in &sources {
        let path = source["path"].as_str().unwrap();
        if path.starts_with("agents/") {
            let agent = path.split('/').nth(1).unwrap();
            if !sources.iter().any(|s| {
                s["path"] == format!("agents/{agent}/instructions.md") && s["kind"] == "agent_md"
            }) {
                return Err(invalid("Named agent requires instructions.md"));
            }
        }
        if source["kind"] == "skill_asset" {
            let segments: Vec<_> = path.split('/').collect();
            let depth = if path.starts_with("agents/") { 4 } else { 2 };
            let skill = format!("{}/SKILL.md", segments[..depth].join("/"));
            if !sources
                .iter()
                .any(|s| s["path"] == skill && s["kind"] == "agent_skill")
            {
                return Err(invalid("Skill asset requires an authored parent SKILL.md"));
            }
        }
    }
    if !instructions {
        return Err(invalid(
            "At least one nonempty instruction source is required",
        ));
    }
    sources.sort_by(|a, b| a["path"].as_str().cmp(&b["path"].as_str()));
    let definition = json!({"schema":"sikaru.agent.contract.v1", "sources":sources});
    let bytes = serde_json::to_vec(&definition).map_err(invalid)?;
    if bytes.len() > 1_000_000 {
        return Err(invalid("Packaged definition exceeds 1 MB"));
    }
    let digest = format!("sha256:{:x}", Sha256::digest(&bytes));
    Ok(json!({"name":manifest.name,"definition":definition,"contentDigest":digest}))
}

pub fn initialize(root: &Path, name: &str) -> Result<(), CliError> {
    validate_name(name)?;
    // Requiring a new directory prevents overwriting existing work, including symlinks.
    fs::create_dir(root).map_err(invalid)?;
    let manifest = json!({"name":name,"sources":[{"path":"instructions.md","kind":"agent_md"}]});
    for (path, content) in [
        (
            "sikaru.json",
            serde_json::to_string_pretty(&manifest).map_err(invalid)? + "\n",
        ),
        (
            "instructions.md",
            "Help the user complete their task. Ask for missing information when needed.\n"
                .to_owned(),
        ),
    ] {
        fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(root.join(path))
            .map_err(invalid)?
            .write_all(content.as_bytes())
            .map_err(invalid)?;
    }
    Ok(())
}
fn directory(matches: &ArgMatches) -> &Path {
    Path::new(matches.get_one::<String>("directory").unwrap())
}
fn init(matches: &ArgMatches, _: &AppContext) -> Result<(), CliError> {
    let root = directory(matches);
    initialize(root, matches.get_one::<String>("name").unwrap())?;
    println!("{}", json!({"directory":root,"status":"created"}));
    Ok(())
}
fn check(matches: &ArgMatches, _: &AppContext) -> Result<(), CliError> {
    println!(
        "{}",
        serde_json::to_string_pretty(&package(directory(matches))?).map_err(invalid)?
    );
    Ok(())
}
fn invoke(
    ctx: &AppContext,
    resource: &str,
    method: &str,
    params: Value,
    body: Value,
) -> Result<Value, CliError> {
    ctx.invoke(
        ctx.find_method(&resource.replace('_', "-"), method)?,
        Some(&params.to_string()),
        Some(&body.to_string()),
        None,
    )
}
fn dev(matches: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let package = package(directory(matches))?;
    let name = package["name"].as_str().unwrap();
    let digest = package["contentDigest"].as_str().unwrap();
    let slug = format!("{}-draft-{}", name, &digest[7..19]);
    let project = matches.get_one::<String>("project").unwrap();
    if matches
        .try_get_one::<bool>("dry-run")
        .ok()
        .flatten()
        .copied()
        .unwrap_or(false)
    {
        println!(
            "{}",
            json!({"dryRun":true,"projectId":project,"agentSlug":slug,"status":"inactive",
            "environment":"draft","source":package,"prompt":matches.get_one::<String>("prompt")})
        );
        return Ok(());
    }
    let agent = invoke(
        ctx,
        "managed_agents",
        "create_managed_agent",
        json!({"project_id":project}),
        json!({
            "agentSlug":slug,"displayName":format!("{name} (draft)"),"status":"inactive",
            "source":{"sourceKind":"source_bundle","definition":package["definition"],"contentDigest":digest}
        }),
    )?;
    eprintln!("Draft ready: {slug}");
    let session = invoke(
        ctx,
        "execution_sessions",
        "create",
        json!({"project_id":project,"harness_id":slug}),
        json!({
            "tenant_id":matches.get_one::<String>("tenant").unwrap(),"user_id":matches.get_one::<String>("user").unwrap(),"environment":"draft"
        }),
    )?;
    let sid = session["session"]["id"]
        .as_str()
        .ok_or_else(|| invalid("Session response has no id"))?;
    eprintln!("Draft session: {sid}");
    let turn = if let Some(prompt) = matches.get_one::<String>("prompt") {
        Some(invoke(
            ctx,
            "execution_sessions",
            "append_turn",
            json!({"project_id":project,"session_id":sid}),
            json!({
                "idempotency_key":format!("dev-{sid}"),"input":{"goal":prompt},"delivery_mode":"queue"
            }),
        )?)
    } else {
        None
    };
    println!("{}", json!({"agent":agent,"session":session,"turn":turn}));
    Ok(())
}
pub fn install(app: CliApp) -> CliApp {
    let path = || Arg::new("directory").default_value(".");
    let required = |name: &'static str| Arg::new(name).long(name).required(true);
    app.command(
        Command::new("init")
            .about("Create a customer agent source directory")
            .arg(Arg::new("directory").required(true))
            .arg(Arg::new("name").long("name").default_value("my-agent")),
        OpenApiBinding::handler(init),
    )
    .command(
        Command::new("check")
            .about("Validate and package explicitly listed customer source files")
            .arg(path()),
        OpenApiBinding::handler(check),
    )
    .command(
        Command::new("dev")
            .about("Create an immutable draft and a hosted draft session")
            .arg(path())
            .arg(required("project"))
            .arg(required("tenant"))
            .arg(required("user"))
            .arg(Arg::new("prompt").long("prompt")),
        OpenApiBinding::handler(dev),
    )
}
