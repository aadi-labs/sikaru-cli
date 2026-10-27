//! Top-level account utilities: `login`, `logout` and `config`.
//!
//! `config set project|organization` persists to `<config dir>/sikaru/config.json`, beside the
//! credential file (`~/Library/Application Support` on macOS, `$XDG_CONFIG_HOME` or `~/.config`
//! elsewhere). At startup a saved project fills `SIKARU_PROJECT` when it is unset, so every
//! `--project` flag resolves flag, then environment, then saved project. The saved organization
//! is shown only; the hidden `config organizations` operations take it as an explicit argument.
use super::user_auth;
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{app::CliApp, error::CliError};
use serde_json::{json, Map, Value};
use std::path::PathBuf;

const SETTINGS: [&str; 2] = ["project", "organization"];

pub fn install(app: CliApp) -> CliApp {
    app.command(login_command(), Box::new(|m, _| login(m)))
        .command(logout_command(), Box::new(|_, _| user_auth::logout()))
        .command_under(&["config"], show_command(), Box::new(|_, _| show()))
        .command_under(&["config"], set_command(), Box::new(|m, _| set(m)))
}

fn login_command() -> Command {
    Command::new("login")
        .about("Sign in to Sikaru in the browser, or store a pasted token")
        .arg(
            Arg::new("with-token")
                .long("with-token")
                .action(ArgAction::SetTrue)
                .help("Read a token from stdin instead of signing in through the browser"),
        )
        .arg(
            Arg::new("no-browser")
                .long("no-browser")
                .action(ArgAction::SetTrue)
                .help("Print the sign-in link instead of opening a browser"),
        )
}

fn logout_command() -> Command {
    Command::new("logout").about("Remove the stored Sikaru sign-in")
}

fn show_command() -> Command {
    Command::new("show").about("Show the active project and organization")
}

fn set_command() -> Command {
    Command::new("set")
        .about("Save the active project or organization")
        .arg(Arg::new("setting").required(true).value_parser(SETTINGS))
        .arg(Arg::new("id").required(true))
}

fn login(m: &ArgMatches) -> Result<(), CliError> {
    // Token paste already warns when SIKARU_API_KEY shadows the stored credential.
    if m.get_flag("with-token") {
        return user_auth::login_with_token();
    }
    user_auth::login(m.get_flag("no-browser"))?;
    if std::env::var("SIKARU_API_KEY").is_ok_and(|key| !key.is_empty()) {
        eprintln!("SIKARU_API_KEY is set and takes precedence over this sign-in. Unset it to use the sign-in.");
    }
    Ok(())
}

fn show() -> Result<(), CliError> {
    let saved = read_settings();
    let active = |key: &str, env: Option<&str>| {
        env.and_then(|name| std::env::var(name).ok())
            .filter(|v| !v.is_empty())
            .map(Value::from)
            .or_else(|| saved.get(key).cloned())
            .unwrap_or(Value::Null)
    };
    println!(
        "{}",
        json!({
            "project": active("project", Some("SIKARU_PROJECT")),
            "organization": active("organization", None),
        })
    );
    Ok(())
}

fn set(m: &ArgMatches) -> Result<(), CliError> {
    let setting = m.get_one::<String>("setting").expect("required");
    let id = m.get_one::<String>("id").expect("required");
    let mut settings = read_settings();
    settings.insert(setting.clone(), Value::from(id.as_str()));
    write_settings(&settings)?;
    eprintln!("Saved {setting} {id}.");
    Ok(())
}

/// Saved project fills `SIKARU_PROJECT`; flag and environment still win. clap captures `.env()`
/// values when an argument is built, so this runs before any command is constructed.
pub fn apply_saved_project() {
    if std::env::var_os("SIKARU_PROJECT").is_some() {
        return;
    }
    if let Some(project) = read_settings().get("project").and_then(Value::as_str) {
        std::env::set_var("SIKARU_PROJECT", project);
    }
}

fn settings_path() -> Option<PathBuf> {
    let home = std::env::var_os("HOME")
        .filter(|h| !h.is_empty())
        .map(PathBuf::from)?;
    #[cfg(target_os = "macos")]
    let root = home.join("Library").join("Application Support");
    #[cfg(not(target_os = "macos"))]
    let root = std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home.join(".config"));
    Some(root.join("sikaru").join("config.json"))
}

fn read_settings() -> Map<String, Value> {
    settings_path()
        .and_then(|path| std::fs::read(path).ok())
        .and_then(|bytes| serde_json::from_slice(&bytes).ok())
        .unwrap_or_default()
}

fn write_settings(settings: &Map<String, Value>) -> Result<(), CliError> {
    let path = settings_path()
        .ok_or_else(|| CliError::Validation("No home directory to save settings in".into()))?;
    let failed = |e: std::io::Error| CliError::Validation(format!("Could not save settings: {e}"));
    std::fs::create_dir_all(path.parent().expect("settings file has a parent")).map_err(failed)?;
    let temp = path.with_extension("json.tmp");
    std::fs::write(&temp, Value::Object(settings.clone()).to_string()).map_err(failed)?;
    std::fs::rename(&temp, &path).map_err(failed)
}
