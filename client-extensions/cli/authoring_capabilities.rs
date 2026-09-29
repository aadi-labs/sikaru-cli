//! Local checks of an agent definition's declared reach: its `web`, `tools` and
//! `setup` sections, and the project capability ceilings it must stay within.
//! The service applies the same rules; checking here reports problems before upload.
use serde::Deserialize;
use serde_json::{json, Map, Value};

pub const SECTIONS: [&str; 3] = ["web", "tools", "setup"];

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields)]
struct Web {
    #[serde(default)]
    provider: Provider,
    #[serde(default)]
    allow_domains: Vec<String>,
    #[serde(default)]
    block_domains: Vec<String>,
}

#[derive(Deserialize, Default)]
#[serde(rename_all = "lowercase")]
enum Hosted {
    #[default]
    Sikaru,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ConnectionTool {
    connection_id: String,
    tool: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
enum Provider {
    Hosted(Hosted),
    Connection(ConnectionTool),
}

impl Default for Provider {
    fn default() -> Self {
        Provider::Hosted(Hosted::Sikaru)
    }
}

#[derive(Deserialize, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
enum Policy {
    #[default]
    Allow,
    RequireApproval,
    Deny,
}

fn enabled_by_default() -> bool {
    true
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct ToolSetting {
    #[serde(default = "enabled_by_default")]
    enabled: bool,
    #[serde(default)]
    policy: Policy,
}

impl Default for ToolSetting {
    fn default() -> Self {
        ToolSetting {
            enabled: true,
            policy: Policy::Allow,
        }
    }
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct Tools {
    bash: ToolSetting,
    workspace: ToolSetting,
    agents: ToolSetting,
    memory: ToolSetting,
    web_search: ToolSetting,
    web_fetch: ToolSetting,
}

impl Tools {
    fn setting(&self, tool: &str) -> Option<&ToolSetting> {
        Some(match tool {
            "bash" => &self.bash,
            "workspace" => &self.workspace,
            "agents" => &self.agents,
            "memory" => &self.memory,
            "web_search" => &self.web_search,
            "web_fetch" => &self.web_fetch,
            _ => return None,
        })
    }
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct Packages {
    pip: Vec<String>,
    npm: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Repo {
    url: String,
    path: String,
    #[serde(default)]
    git_credential: Option<String>,
}

#[derive(Deserialize, Default)]
#[serde(deny_unknown_fields, default)]
struct Setup {
    packages: Packages,
    commands: Vec<String>,
    repos: Vec<Repo>,
}

#[derive(Deserialize, Default)]
#[serde(default)]
struct Reach {
    web: Web,
    tools: Tools,
    setup: Setup,
}

fn ensure(condition: bool, message: impl FnOnce() -> String) -> Result<(), String> {
    if condition {
        Ok(())
    } else {
        Err(message())
    }
}

fn is_label(label: &str) -> bool {
    (1..=63).contains(&label.len())
        && label
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
        && !label.starts_with('-')
        && !label.ends_with('-')
}

/// A bare lowercase hostname with at least two labels.
fn is_domain(value: &str) -> bool {
    let labels: Vec<_> = value.split('.').collect();
    value.len() <= 253 && labels.len() >= 2 && labels.iter().all(|label| is_label(label))
}

fn is_reference(value: &str) -> bool {
    let bytes = value.as_bytes();
    (1..=128).contains(&bytes.len())
        && bytes[0].is_ascii_alphanumeric()
        && bytes
            .iter()
            .all(|b| b.is_ascii_alphanumeric() || b"_.:-".contains(b))
}

fn limit<T>(items: &[T], most: usize, field: &str) -> Result<(), String> {
    ensure(items.len() <= most, || {
        format!("{field} allows at most {most} entries")
    })
}

fn check_web(web: &Web) -> Result<(), String> {
    if let Provider::Connection(tool) = &web.provider {
        ensure(is_reference(&tool.connection_id), || {
            "web.provider.connection_id is not a valid reference".into()
        })?;
        ensure((1..=256).contains(&tool.tool.chars().count()), || {
            "web.provider.tool must be 1-256 characters".into()
        })?;
    }
    for (field, domains) in [
        ("web.allow_domains", &web.allow_domains),
        ("web.block_domains", &web.block_domains),
    ] {
        limit(domains, 100, field)?;
        if let Some(bad) = domains.iter().find(|domain| !is_domain(domain)) {
            return Err(format!(
                "Invalid domain {bad:?} in {field}; use a bare lowercase hostname"
            ));
        }
    }
    match web
        .allow_domains
        .iter()
        .find(|domain| web.block_domains.contains(domain))
    {
        Some(both) => Err(format!(
            "Domain {both} cannot be in both allow and block lists"
        )),
        None => Ok(()),
    }
}

fn check_packages(packages: &Packages) -> Result<(), String> {
    for (field, names) in [
        ("setup.packages.pip", &packages.pip),
        ("setup.packages.npm", &packages.npm),
    ] {
        limit(names, 100, field)?;
        let valid = |name: &&String| {
            (1..=200).contains(&name.chars().count())
                && (name.as_bytes()[0].is_ascii_alphanumeric() || name.starts_with('@'))
                && !name.chars().any(char::is_whitespace)
        };
        if let Some(bad) = names.iter().find(|name| !valid(name)) {
            return Err(format!("Invalid setup package {bad:?}"));
        }
    }
    Ok(())
}

fn check_commands(commands: &[String]) -> Result<(), String> {
    limit(commands, 50, "setup.commands")?;
    let valid = |command: &String| {
        command.chars().count() <= 4000 && !command.trim().is_empty() && !command.contains('\0')
    };
    ensure(commands.iter().all(valid), || {
        "Setup commands must be nonempty text up to 4000 characters".into()
    })
}

/// What follows `https://`, compared case-insensitively as URL schemes are.
fn after_https(url: &str) -> Option<&str> {
    let (scheme, rest) = url.split_once("://")?;
    scheme.eq_ignore_ascii_case("https").then_some(rest)
}

/// The repo's lowercase host, when the URL is a credential-free https repository URL.
fn repo_host(url: &str) -> Result<String, String> {
    let invalid = |reason: &str| format!("Setup repo URL {url:?} {reason}");
    let rest = after_https(url).ok_or_else(|| invalid("must use https"))?;
    let (authority, path) = rest.split_at(rest.find('/').unwrap_or(rest.len()));
    let (host, port) = authority.split_once(':').unwrap_or((authority, "443"));
    let problems = [
        (url.len() > 2048, "is too long"),
        (
            rest.contains(['?', '#']),
            "must name a repository without query or fragment",
        ),
        (
            authority.contains('@'),
            "must not embed credentials; use git_credential",
        ),
        (port != "443", "must use the default https port"),
        (host.is_empty(), "must name a host"),
        (path.trim_matches('/').is_empty(), "must name a repository"),
    ];
    match problems.iter().find(|(found, _)| *found) {
        Some((_, reason)) => Err(invalid(reason)),
        None => Ok(host.to_ascii_lowercase()),
    }
}

fn check_workspace_path(path: &str) -> Result<(), String> {
    let relative = path.len() <= 512
        && !path.contains('\\')
        && !path.starts_with('/')
        && path.split('/').all(|part| !matches!(part, "" | "." | ".."));
    ensure(relative, || {
        format!("Setup repo path {path:?} must be a relative workspace path")
    })
}

fn check_repos(repos: &[Repo]) -> Result<(), String> {
    limit(repos, 20, "setup.repos")?;
    for repo in repos {
        repo_host(&repo.url)?;
        check_workspace_path(&repo.path)?;
        let credential_ok = repo.git_credential.as_deref().map_or(true, is_reference);
        ensure(credential_ok, || {
            format!("Git credential for {} is not a valid reference", repo.path)
        })?;
    }
    let mut paths: Vec<_> = repos.iter().map(|repo| format!("{}/", repo.path)).collect();
    paths.sort();
    // Sorted, a path that equals or contains another is immediately followed by it.
    match paths.windows(2).find(|pair| pair[1].starts_with(&pair[0])) {
        Some(pair) => Err(format!(
            "Setup repo path {:?} overlaps another repo path",
            pair[0].trim_end_matches('/')
        )),
        None => Ok(()),
    }
}

/// Every value the definition's reach reads as a struct, with its field name.
fn struct_values(definition: &Value) -> Vec<(String, &Value)> {
    let mut found: Vec<(String, &Value)> = SECTIONS
        .iter()
        .filter_map(|key| Some((key.to_string(), definition.get(key)?)))
        .collect();
    if let Some(provider) = definition
        .pointer("/web/provider")
        .filter(|p| !p.is_string())
    {
        found.push(("web.provider".into(), provider));
    }
    if let Some(tools) = definition.get("tools").and_then(Value::as_object) {
        found.extend(
            tools
                .iter()
                .map(|(tool, setting)| (format!("tools.{tool}"), setting)),
        );
    }
    if let Some(packages) = definition.pointer("/setup/packages") {
        found.push(("setup.packages".into(), packages));
    }
    if let Some(repos) = definition.pointer("/setup/repos").and_then(Value::as_array) {
        found.extend(repos.iter().map(|repo| ("setup.repos".to_owned(), repo)));
    }
    found
}

/// serde also reads structs from JSON arrays; the service accepts only objects.
fn require_objects(definition: &Value) -> Result<(), String> {
    match struct_values(definition)
        .into_iter()
        .find(|(_, value)| !value.is_object())
    {
        Some((field, _)) => Err(format!(
            "Invalid capability section: {field} must be an object"
        )),
        None => Ok(()),
    }
}

fn parse(definition: &Value) -> Result<Reach, String> {
    require_objects(definition)?;
    let mut sections = Map::new();
    for key in SECTIONS {
        if let Some(value) = definition.get(key) {
            sections.insert(key.to_owned(), value.clone());
        }
    }
    let reach: Reach = serde_json::from_value(Value::Object(sections))
        .map_err(|e| format!("Invalid capability section: {e}"))?;
    check_web(&reach.web)?;
    check_packages(&reach.setup.packages)?;
    check_commands(&reach.setup.commands)?;
    check_repos(&reach.setup.repos)?;
    Ok(reach)
}

/// Validate the declared sections and add them to the packaged definition as authored.
pub fn attach(definition: &mut Value, sections: Map<String, Value>) -> Result<(), String> {
    let object = definition.as_object_mut().expect("definition is an object");
    object.extend(sections);
    parse(definition).map(|_| ())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
struct Ceilings {
    #[serde(default = "enabled_by_default")]
    egress_enabled: bool,
    #[serde(default)]
    domain_denylist: Vec<String>,
    #[serde(default)]
    disallowed_tools: Vec<String>,
    /// Absent or null allows any git host; a list, even an empty one, allows only those hosts.
    #[serde(default)]
    allowed_git_hosts: Option<Vec<String>>,
}

fn violation(ceiling: &str, limit: &str, field: &str, message: String) -> Value {
    json!({"ceiling": ceiling, "limit": limit, "field": field, "message": message})
}

fn covers(entry: &str, host: &str) -> bool {
    let domain = entry.strip_prefix("*.").unwrap_or(entry);
    host == domain || host.ends_with(&format!(".{domain}"))
}

fn egress_violations(reach: &Reach, ceilings: &Ceilings) -> Vec<Value> {
    if ceilings.egress_enabled {
        return vec![];
    }
    let setup = &reach.setup;
    let packages = !setup.packages.pip.is_empty() || !setup.packages.npm.is_empty();
    [
        ("setup.packages", packages),
        ("setup.commands", !setup.commands.is_empty()),
        ("setup.repos", !setup.repos.is_empty()),
    ]
    .into_iter()
    .filter(|(_, declared)| *declared)
    .map(|(field, _)| {
        violation(
            "egress",
            "off",
            field,
            format!("{field} needs sandbox egress, which this project's egress ceiling turns off"),
        )
    })
    .collect()
}

fn denied(hosts: &[String], ceilings: &Ceilings, field: &str, subject: &str) -> Vec<Value> {
    let mut found = vec![];
    for host in hosts {
        for entry in ceilings
            .domain_denylist
            .iter()
            .filter(|entry| covers(entry, host))
        {
            found.push(violation(
                "domain_denylist",
                entry,
                field,
                format!("{subject} {host} is denied by the project domain denylist entry {entry}"),
            ));
        }
    }
    found
}

/// The definition states an enabled flag or policy for the tool, and leaves the tool usable.
fn explicitly_enabled(stated: &Value, reach: &Reach, tool: &str) -> bool {
    let named = stated
        .get(tool)
        .and_then(Value::as_object)
        .is_some_and(|setting| !setting.is_empty());
    let setting = reach.tools.setting(tool);
    named && setting.is_some_and(|setting| setting.enabled && setting.policy != Policy::Deny)
}

fn tool_violations(definition: &Value, reach: &Reach, ceilings: &Ceilings) -> Vec<Value> {
    let stated = definition.get("tools").cloned().unwrap_or(Value::Null);
    ceilings.disallowed_tools.iter()
        .filter(|tool| explicitly_enabled(&stated, reach, tool))
        .map(|tool| violation("disallowed_tools", tool, &format!("tools.{tool}"),
            format!("The {tool} tool is disallowed by the project ceilings; disable it or set its policy to deny")))
        .collect()
}

fn git_host_violations(hosts: &[String], ceilings: &Ceilings) -> Vec<Value> {
    let Some(allowed) = &ceilings.allowed_git_hosts else {
        return vec![];
    };
    hosts
        .iter()
        .filter(|host| !allowed.contains(host))
        .map(|host| {
            violation(
                "allowed_git_hosts",
                host,
                "setup.repos",
                format!("Setup repo host {host} is not in the project's allowed git hosts"),
            )
        })
        .collect()
}

/// Every way the definition's declared reach exceeds the project's ceilings. `ceilings` is
/// either the ceilings object or the `{ceilings, canEdit}` view the API returns.
pub fn violations(definition: &Value, ceilings: &Value) -> Result<Vec<Value>, String> {
    let ceilings = ceilings.get("ceilings").unwrap_or(ceilings);
    let ceilings: Ceilings = serde_json::from_value(ceilings.clone())
        .map_err(|e| format!("Invalid capability ceilings: {e}"))?;
    let reach = parse(definition)?;
    let hosts = reach
        .setup
        .repos
        .iter()
        .map(|repo| repo_host(&repo.url))
        .collect::<Result<Vec<_>, _>>()?;
    Ok([
        egress_violations(&reach, &ceilings),
        denied(
            &reach.web.allow_domains,
            &ceilings,
            "web.allow_domains",
            "Web domain",
        ),
        denied(&hosts, &ceilings, "setup.repos", "Setup repo host"),
        tool_violations(definition, &reach, &ceilings),
        git_host_violations(&hosts, &ceilings),
    ]
    .concat())
}

/// The capability sections `init` writes: Sikaru web search with no domain lists and no
/// setup. Tools are left to their defaults so the scaffold never names a tool a
/// project ceiling may disallow.
pub fn scaffold() -> Map<String, Value> {
    let mut sections = Map::new();
    sections.insert(
        "web".into(),
        json!({"provider": "sikaru", "allow_domains": [], "block_domains": []}),
    );
    sections.insert("tools".into(), json!({}));
    sections.insert(
        "setup".into(),
        json!({"packages": {"pip": [], "npm": []}, "commands": [], "repos": []}),
    );
    sections
}
