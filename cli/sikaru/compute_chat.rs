//! A terminal conversation reuses the same durable hosted workflow for every turn.
use super::{diagnostics, input, workflow};
use anyhow::Result;
use clap::ArgMatches;
use fern_cli_sdk::openapi::AppContext;
use serde_json::{json, Value};
use std::io::{IsTerminal, Write};

pub fn is_interactive(m: &ArgMatches) -> bool {
    std::io::stdin().is_terminal()
        && !m.get_flag("print")
        && m.try_get_one::<bool>("dry-run").ok().flatten() != Some(&true)
}

pub async fn execute(m: &ArgMatches, ctx: &AppContext) -> Result<Value> {
    if !std::io::stdin().is_terminal() {
        return Err(input::InvalidInput(
            "Chat requires a terminal. Use sikaru exec - for piped tasks.",
        )
        .into());
    }
    if m.try_get_one::<bool>("dry-run").ok().flatten() == Some(&true) {
        return Err(input::InvalidInput(
            "For a read-only preview, use sikaru exec --dry-run 'Your task'.",
        )
        .into());
    }
    let mut resume = m.get_one::<String>("resume").cloned();
    let mut text = input::source(m)?;
    let mut last = json!({"status":"completed","execution":null,"cleanup":"not_started","usage":{"available":false}});
    eprintln!("Sikaru — enter a task, /help for commands, /exit to leave.");
    loop {
        let Some(task) = next_task(text.take(), &last).await? else {
            return Ok(last);
        };
        let args = turn_matches(m, resume.as_deref(), &task)?;
        last = workflow::execute(&args, ctx)
            .await
            .unwrap_or_else(|e| diagnostics::failure(&e, "controller_or_resume_failed"));
        render(&last);
        if last["status"] != "completed" {
            return Ok(last);
        }
        resume = last["state_dir"].as_str().map(str::to_owned);
    }
}

async fn next_task(initial: Option<String>, last: &Value) -> Result<Option<String>> {
    if let Some(text) = initial {
        return Ok(Some(text));
    }
    loop {
        eprint!("\nYou> ");
        std::io::stderr().flush()?;
        if !wait_for_input().await {
            return Ok(None);
        }
        let mut line = String::new();
        if std::io::stdin().read_line(&mut line)? == 0 {
            return Ok(None);
        }
        match line.trim() {
            "/exit" | "/quit" => return Ok(None),
            "/help" => eprintln!("Enter a task to continue this session. /status shows the last result. /exit leaves the session available for explicit --resume."),
            "/status" => eprintln!("{}", last),
            "" => continue,
            _ => return Ok(Some(line)),
        }
    }
}

async fn wait_for_input() -> bool {
    tokio::select! {
        _ = super::runtime::termination_signal() => false,
        _ = async {
            loop {
                let mut descriptor = libc::pollfd { fd: libc::STDIN_FILENO, events: libc::POLLIN, revents: 0 };
                // Canonical terminal input becomes readable when a full line or EOF is available.
                if unsafe { libc::poll(&mut descriptor, 1, 0) } != 0 { break; }
                tokio::time::sleep(std::time::Duration::from_millis(50)).await;
            }
        } => true,
    }
}

fn turn_matches(m: &ArgMatches, resume: Option<&str>, text: &str) -> Result<ArgMatches> {
    let mut args = vec!["exec".to_owned()];
    for name in ["project", "agent", "workspace", "tenant", "user", "model"] {
        if let Some(value) = m.get_one::<String>(name) {
            args.extend([format!("--{name}"), value.clone()]);
        }
    }
    for name in ["timeout", "approval-wait"] {
        args.extend([
            format!("--{name}"),
            m.get_one::<u64>(name).unwrap().to_string(),
        ]);
    }
    add_state(&mut args, m, resume);
    args.push(format!("--prompt={text}"));
    Ok(workflow::command().try_get_matches_from(args)?)
}

fn add_state(args: &mut Vec<String>, m: &ArgMatches, resume: Option<&str>) {
    if let Some(path) = resume {
        args.extend(["--resume".into(), path.to_owned()]);
    } else if let Some(path) = m.get_one::<String>("state-dir") {
        args.extend(["--state-dir".into(), path.clone()]);
    }
}

pub fn render(result: &Value) {
    let output = &result["final_output"]["output"];
    if let Some(text) = output.as_str() {
        let safe: String = text
            .chars()
            .filter(|c| !c.is_control() || matches!(c, '\n' | '\t'))
            .collect();
        eprintln!("\nSikaru> {safe}");
    } else if !output.is_null() {
        eprintln!("\nSikaru> {output}");
    }
    render_usage(result);
    if let Some(path) = result["state_dir"].as_str() {
        eprintln!("Saved session: {path}");
    }
    if let Some(help) = result["help"].as_str() {
        eprintln!("{help}");
    }
}

fn render_usage(result: &Value) {
    let usage = &result["usage"]["usage"];
    let counts = (
        usage["n_input_tokens"].as_u64(),
        usage["n_cache_tokens"].as_u64(),
        usage["n_output_tokens"].as_u64(),
    );
    if let (Some(input), Some(cached), Some(output)) = counts {
        let coverage = if usage["complete"] == false {
            " (partial)"
        } else {
            ""
        };
        eprintln!("Tokens{coverage}: {input} input ({cached} cached), {output} output.");
    }
}
