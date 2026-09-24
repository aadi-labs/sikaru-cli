//! Human input validation happens before a workflow journal or remote mutation.
use anyhow::{bail, Result};
use clap::ArgMatches;
use std::io::{IsTerminal, Read};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub struct InvalidInput(pub &'static str);

pub fn prompt(m: &ArgMatches) -> Result<Option<String>> {
    let text = source(m)?;
    if text.as_ref().is_some_and(|s| s.trim().is_empty()) {
        bail!(InvalidInput(
            "The prompt is empty. Supply a task as an argument, --prompt, or --prompt-file."
        ));
    }
    if text.is_none() && m.get_one::<String>("resume").is_none() {
        bail!(InvalidInput(
            "Supply a task: sikaru exec 'Your task', or pipe it to sikaru exec -."
        ));
    }
    Ok(text)
}

pub fn source(m: &ArgMatches) -> Result<Option<String>> {
    if let Some(path) = m.get_one::<String>("prompt-file") {
        if path == "-" {
            return read_stdin().map(Some);
        }
        return std::fs::File::open(path)
            .map_err(|_| InvalidInput("Cannot read --prompt-file. Check the path and permissions."))
            .and_then(|file| {
                read_text(file)
                    .map_err(|_| InvalidInput("Prompt file must be UTF-8 and at most 1 MiB."))
            })
            .map(Some)
            .map_err(Into::into);
    }
    let text = m
        .get_one::<String>("prompt")
        .or_else(|| m.get_one::<String>("task"));
    match text.map(String::as_str) {
        Some("-") => read_stdin().map(Some),
        Some(value) => Ok(Some(value.to_owned())),
        None => Ok(None),
    }
}

fn read_stdin() -> Result<String> {
    if std::io::stdin().is_terminal() {
        bail!(InvalidInput(
            "Pipe a task to stdin when using '-'. For a conversation, use sikaru exec."
        ));
    }
    read_text(std::io::stdin())
}

fn read_text(reader: impl Read) -> Result<String> {
    let mut text = String::new();
    reader
        .take(1_048_577)
        .read_to_string(&mut text)
        .map_err(|_| InvalidInput("Prompt input must be valid UTF-8."))?;
    if text.len() > 1_048_576 {
        bail!(InvalidInput("Prompt input exceeds 1 MiB."));
    }
    Ok(text)
}
