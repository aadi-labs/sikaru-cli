//! Native inference commands: a one-shot chat and the model catalog over the
//! public edge. They print for people; `--output json` prints the raw edge response (the chat
//! completion, the model list, or one model card) for scripts and agents.
use clap::{Arg, ArgAction, ArgMatches, Command};
use fern_cli_sdk::{
    app::CliApp,
    error::CliError,
    openapi::{AppContext, OpenApiBinding},
    sdk_executor::{SdkError, SdkRequestExecutor},
};
use futures_util::StreamExt;
use reqwest::Method;
use serde_json::{json, Value};
use std::cmp::Ordering;
use std::io::{IsTerminal, Read, Write};

/// The standard effort vocabulary, in the order the normalization table is shown.
const EFFORTS: [&str; 9] = [
    "none", "minimal", "low", "medium", "high", "xhigh", "max", "true", "false",
];

const COLUMNS: [&str; 7] = [
    "id",
    "name",
    "input $/1M tok",
    "output $/1M tok",
    "context",
    "max out",
    "reasoning",
];

pub fn install(app: CliApp) -> CliApp {
    app.command_under(
        &["inference"],
        chat_command(),
        OpenApiBinding::handler(|m, ctx| blocking(chat(m, ctx))),
    )
    .command_under(
        &["inference"],
        models_command(),
        OpenApiBinding::handler(|m, ctx| blocking(models(m, ctx))),
    )
}

fn blocking<T>(future: impl std::future::Future<Output = T>) -> T {
    tokio::task::block_in_place(|| tokio::runtime::Handle::current().block_on(future))
}

fn chat_command() -> Command {
    Command::new("chat")
        .about("Send a one-shot chat message to a catalog model")
        .after_help(
            "Examples:\n  sikaru inference chat MODEL \"say hi\"\n  echo \"explain RL in one line\" | sikaru inference chat MODEL\n  sikaru inference chat MODEL \"hi\" --stream",
        )
        .arg(
            Arg::new("model")
                .required(true)
                .help("Model id (see `sikaru inference models`)"),
        )
        .arg(Arg::new("message").help("User message. If omitted, reads from stdin."))
        .arg(
            Arg::new("system")
                .long("system")
                .short('s')
                .help("System prompt"),
        )
        .arg(
            Arg::new("temperature")
                .long("temperature")
                .short('t')
                .value_parser(clap::value_parser!(f64))
                .help("Sampling temperature"),
        )
        .arg(
            Arg::new("max-tokens")
                .long("max-tokens")
                .value_parser(clap::value_parser!(i64))
                .help("Maximum tokens to generate"),
        )
        .arg(Arg::new("reasoning-effort").long("reasoning-effort").help(
            "Supported values depend on the model; see `sikaru inference models MODEL`",
        ))
        .arg(
            Arg::new("stream")
                .long("stream")
                .action(ArgAction::SetTrue)
                .help("Stream answer tokens to stdout and reasoning to stderr"),
        )
        .arg(output_arg(["text", "json"], "text"))
}

fn models_command() -> Command {
    Command::new("models")
        .about("List catalog models, or describe one model")
        .arg(
            Arg::new("model")
                .help("Model id to describe: specs, reasoning efforts and their normalization"),
        )
        .arg(output_arg(["table", "json"], "table"))
        .arg(
            Arg::new("search")
                .long("search")
                .help("Case-insensitive substring match on model id"),
        )
        .arg(
            Arg::new("sort")
                .long("sort")
                .value_parser(["id", "input", "output"])
                .default_value("id")
                .help("Sort by: id, input, output"),
        )
        .arg(
            Arg::new("order")
                .long("order")
                .value_parser(["asc", "desc"])
                .default_value("asc")
                .help("Sort order: asc, desc"),
        )
}

fn output_arg(formats: [&'static str; 2], default: &'static str) -> Arg {
    Arg::new("output")
        .long("output")
        .short('o')
        .value_parser(formats)
        .default_value(default)
        .help(format!("{}|{}", formats[0], formats[1]))
}

fn json_output(m: &ArgMatches) -> bool {
    m.get_one::<String>("output")
        .is_some_and(|output| output == "json")
}

async fn chat(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    let stream = m.get_flag("stream");
    if stream && json_output(m) {
        return Err(invalid("--stream is not supported with --output json"));
    }
    let message = message(m)?;
    let accept = if stream {
        "text/event-stream"
    } else {
        "application/json"
    };
    let request = endpoint(ctx, Method::POST, "v1/chat/completions")
        .header("Accept", accept)
        .json(&chat_body(m, &message, stream));
    let response = execute(ctx, request).await?;
    if stream {
        let usage = print_stream(response).await?;
        eprintln!("{}", usage_line(&usage));
        return Ok(());
    }
    let completion: Value = response.json().await.map_err(transport_error)?;
    if json_output(m) {
        return print_json(&completion);
    }
    print_reply(&completion);
    eprintln!("{}", usage_line(&completion["usage"]));
    Ok(())
}

/// The message argument, or piped stdin when it is omitted.
fn message(m: &ArgMatches) -> Result<String, CliError> {
    if let Some(message) = m.get_one::<String>("message") {
        return Ok(message.clone());
    }
    let stdin = std::io::stdin();
    if stdin.is_terminal() {
        return Err(invalid(
            "no message provided (pass it as an argument or via stdin)",
        ));
    }
    let mut text = String::new();
    stdin
        .lock()
        .read_to_string(&mut text)
        .map_err(|error| invalid(format!("reading stdin failed: {error}")))?;
    let text = text.trim();
    if text.is_empty() {
        return Err(invalid("empty message from stdin"));
    }
    Ok(text.to_string())
}

fn chat_body(m: &ArgMatches, message: &str, stream: bool) -> Value {
    let mut messages = Vec::new();
    if let Some(system) = m.get_one::<String>("system") {
        messages.push(json!({"role": "system", "content": system}));
    }
    messages.push(json!({"role": "user", "content": message}));
    let mut body =
        json!({"model": m.get_one::<String>("model"), "messages": messages, "stream": stream});
    if stream {
        body["stream_options"] = json!({"include_usage": true});
    }
    if let Some(limit) = m.get_one::<i64>("max-tokens") {
        body["max_tokens"] = json!(limit);
    }
    if let Some(temperature) = m.get_one::<f64>("temperature") {
        body["temperature"] = json!(temperature);
    }
    if let Some(effort) = m.get_one::<String>("reasoning-effort") {
        body["reasoning_effort"] = effort_value(effort);
    }
    body
}

/// Boolean and integer efforts are typed on the wire; the edge normalizes or names the field.
fn effort_value(effort: &str) -> Value {
    effort
        .parse::<bool>()
        .map(Value::from)
        .or_else(|_| effort.parse::<i64>().map(Value::from))
        .unwrap_or_else(|_| Value::from(effort))
}

fn endpoint(ctx: &AppContext, method: Method, path: &str) -> reqwest::RequestBuilder {
    reqwest::Client::new().request(
        method,
        sikaru_sdk::join_url(&ctx.effective_base_url(), path),
    )
}

/// Sends through the CLI's own transport, so credentials, base URL and `--debug` apply.
async fn execute(
    ctx: &AppContext,
    request: reqwest::RequestBuilder,
) -> Result<reqwest::Response, CliError> {
    let request = request
        .build()
        .map_err(|error| invalid(error.to_string()))?;
    let response = ctx
        .build_sdk_executor()
        .execute(request)
        .await
        .map_err(SdkError::into_cli_error)?;
    if response.status().is_success() {
        return Ok(response);
    }
    let status = response.status().as_u16();
    Err(edge_error(
        status,
        &response.text().await.unwrap_or_default(),
    ))
}

/// Model responses are read untyped, so the CLI follows the edge's card without regenerated types.
async fn get_json(ctx: &AppContext, path: &str) -> Result<Value, CliError> {
    let request = endpoint(ctx, Method::GET, path).header("Accept", "application/json");
    execute(ctx, request)
        .await?
        .json()
        .await
        .map_err(transport_error)
}

/// The edge names the rejected field in `error.param`; show it ahead of the message.
fn edge_error(status: u16, body: &str) -> CliError {
    let parsed: Value = serde_json::from_str(body).unwrap_or(Value::Null);
    let error = &parsed["error"];
    let message = error["message"].as_str().unwrap_or(body);
    let reason = error["code"].as_str().unwrap_or("inference_error");
    match error["param"].as_str() {
        Some(param) => CliError::api(status, format!("{param}: {message}"), reason),
        None => CliError::api(status, message, reason),
    }
}

fn print_reply(completion: &Value) {
    let content = completion["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("");
    if content.ends_with('\n') {
        print!("{content}");
    } else {
        println!("{content}");
    }
}

/// Reads the event stream line by line; the final usage chunk carries the token counts.
async fn print_stream(response: reqwest::Response) -> Result<Value, CliError> {
    let mut bytes = response.bytes_stream();
    let mut pending: Vec<u8> = Vec::new();
    let mut printer = StreamPrinter::default();
    while let Some(piece) = bytes.next().await {
        pending.extend_from_slice(&piece.map_err(transport_error)?);
        while let Some(end) = pending.iter().position(|byte| *byte == b'\n') {
            let line: Vec<u8> = pending.drain(..=end).collect();
            printer.event(String::from_utf8_lossy(&line).trim_end())?;
        }
    }
    printer.finish()
}

/// Answer tokens go to stdout; reasoning goes to stderr inside a `<think>` bracket.
#[derive(Default)]
struct StreamPrinter {
    in_reasoning: bool,
    usage: Value,
}

impl StreamPrinter {
    fn event(&mut self, line: &str) -> Result<(), CliError> {
        let Some(chunk) = stream_chunk(line)? else {
            return Ok(());
        };
        if !chunk["usage"].is_null() {
            self.usage = chunk["usage"].clone();
        }
        let delta = &chunk["choices"][0]["delta"];
        if let Some(reasoning) = non_empty(&delta["reasoning_content"]) {
            self.reasoning(reasoning);
        }
        if let Some(answer) = non_empty(&delta["content"]) {
            self.answer(answer);
        }
        flush()
    }

    fn reasoning(&mut self, text: &str) {
        if !self.in_reasoning {
            eprintln!("<think>");
            self.in_reasoning = true;
        }
        eprint!("{text}");
    }

    fn answer(&mut self, text: &str) {
        if self.in_reasoning {
            eprint!("\n</think>\n\n");
            self.in_reasoning = false;
        }
        print!("{text}");
    }

    fn finish(self) -> Result<Value, CliError> {
        if self.in_reasoning {
            eprint!("\n</think>\n");
        }
        println!();
        flush()?;
        Ok(self.usage)
    }
}

fn stream_chunk(line: &str) -> Result<Option<Value>, CliError> {
    let Some(data) = line
        .strip_prefix("data:")
        .map(str::trim)
        .filter(|data| *data != "[DONE]")
    else {
        return Ok(None);
    };
    let chunk: Value = serde_json::from_str(data).map_err(|error| invalid(error.to_string()))?;
    if chunk.get("error").is_some() {
        return Err(edge_error(502, data));
    }
    Ok(Some(chunk))
}

fn non_empty(value: &Value) -> Option<&str> {
    value.as_str().filter(|text| !text.is_empty())
}

fn flush() -> Result<(), CliError> {
    std::io::stderr()
        .flush()
        .and_then(|()| std::io::stdout().flush())
        .map_err(|error| invalid(error.to_string()))
}

fn usage_line(usage: &Value) -> String {
    if usage.is_null() {
        return "usage: not reported".to_string();
    }
    let count = |key: &str| {
        usage[key]
            .as_i64()
            .map_or_else(|| "?".to_string(), |n| n.to_string())
    };
    format!(
        "usage: {} prompt + {} completion = {} tokens",
        count("prompt_tokens"),
        count("completion_tokens"),
        count("total_tokens")
    )
}

fn print_json(value: &Value) -> Result<(), CliError> {
    let text = serde_json::to_string_pretty(value).map_err(|error| invalid(error.to_string()))?;
    println!("{text}");
    Ok(())
}

fn transport_error(error: reqwest::Error) -> CliError {
    CliError::Validation(format!("inference response failed: {error}"))
}

fn invalid(message: impl Into<String>) -> CliError {
    CliError::Validation(message.into())
}

async fn models(m: &ArgMatches, ctx: &AppContext) -> Result<(), CliError> {
    if let Some(model) = m.get_one::<String>("model") {
        let card = get_json(ctx, &format!("v1/models/{model}")).await?;
        if json_output(m) {
            return print_json(&card);
        }
        print!("{}", describe(&card));
        return Ok(());
    }
    let mut listing = get_json(ctx, "v1/models").await?;
    let listed = selected(m, listing["data"].as_array().cloned().unwrap_or_default());
    if json_output(m) {
        listing["data"] = Value::Array(listed);
        return print_json(&listing);
    }
    print_table(&listed);
    Ok(())
}

/// Applies `--search`, `--sort` and `--order` to the catalog.
fn selected(m: &ArgMatches, models: Vec<Value>) -> Vec<Value> {
    let needle = m
        .get_one::<String>("search")
        .map(|search| search.to_lowercase())
        .unwrap_or_default();
    let mut models: Vec<Value> = models
        .into_iter()
        .filter(|model| model_id(model).to_lowercase().contains(&needle))
        .collect();
    let sort = m.get_one::<String>("sort").map_or("id", String::as_str);
    let descending = m
        .get_one::<String>("order")
        .is_some_and(|order| order == "desc");
    if sort == "id" {
        models.sort_by_key(|model| model_id(model).to_lowercase());
        if descending {
            models.reverse();
        }
        return models;
    }
    let key = format!("{sort}_usd_per_mtok");
    models.sort_by(|a, b| by_price(price(a, &key), price(b, &key), descending));
    models
}

/// Models without a price sort last in either order.
fn by_price(a: Option<f64>, b: Option<f64>, descending: bool) -> Ordering {
    match (a, b) {
        (Some(a), Some(b)) if descending => b.total_cmp(&a),
        (Some(a), Some(b)) => a.total_cmp(&b),
        (Some(_), None) => Ordering::Less,
        (None, Some(_)) => Ordering::Greater,
        (None, None) => Ordering::Equal,
    }
}

fn model_id(model: &Value) -> &str {
    model["id"].as_str().unwrap_or("")
}

fn price(model: &Value, key: &str) -> Option<f64> {
    model["pricing"][key].as_f64()
}

fn print_table(models: &[Value]) {
    if models.is_empty() {
        eprintln!("No models returned.");
        return;
    }
    let mut rows = vec![COLUMNS.map(str::to_string)];
    rows.extend(models.iter().map(table_row));
    print!("{}", render(&rows));
    let legend = "reasoning ✓ = supports reasoning effort";
    if std::io::stdout().is_terminal() {
        println!("\x1b[2m{legend}\x1b[0m");
    } else {
        println!("{legend}");
    }
}

fn table_row(model: &Value) -> [String; 7] {
    let specs = &model["specs"];
    let reasoning = if specs["supports_reasoning"].as_bool() == Some(true) {
        "✓"
    } else {
        "—"
    };
    [
        model_id(model).to_string(),
        model["display_name"].as_str().unwrap_or("—").to_string(),
        dollars(price(model, "input_usd_per_mtok")),
        dollars(price(model, "output_usd_per_mtok")),
        tokens(&specs["context_window"]),
        tokens(&specs["max_output_tokens"]),
        reasoning.to_string(),
    ]
}

/// Left-aligns text columns and right-aligns the price and token columns.
fn render(rows: &[[String; 7]]) -> String {
    let widths: Vec<usize> = (0..COLUMNS.len())
        .map(|column| {
            rows.iter()
                .map(|row| row[column].chars().count())
                .max()
                .unwrap_or(0)
        })
        .collect();
    rows.iter()
        .map(|row| {
            let cells: Vec<String> = row
                .iter()
                .zip(&widths)
                .enumerate()
                .map(|(column, (cell, &width))| match column {
                    2..=5 => format!("{cell:>width$}"),
                    _ => format!("{cell:<width$}"),
                })
                .collect();
            format!("{}\n", cells.join("  ").trim_end())
        })
        .collect()
}

/// USD per 1M tokens with trailing zeros trimmed: 0.1875 -> `$0.1875`.
fn dollars(value: Option<f64>) -> String {
    value.map_or_else(
        || "—".to_string(),
        |value| {
            format!("${value:.6}")
                .trim_end_matches('0')
                .trim_end_matches('.')
                .to_string()
        },
    )
}

/// Compact token counts: 1000000 -> `1M`, 65536 -> `65.5k`.
fn tokens(value: &Value) -> String {
    let count = value.as_i64().unwrap_or(0);
    let compact = |scaled: f64, digits: usize, unit: &str| {
        let text = format!("{scaled:.digits$}");
        format!("{}{unit}", text.trim_end_matches('0').trim_end_matches('.'))
    };
    match count {
        ..=0 => "—".to_string(),
        1_000_000.. => compact(count as f64 / 1e6, 2, "M"),
        1_000.. => compact(count as f64 / 1e3, 1, "k"),
        _ => count.to_string(),
    }
}

/// One model's published specs, with what each standard effort runs at.
fn describe(card: &Value) -> String {
    let specs = &card["specs"];
    let mut text = format!(
        "{}  {}\ncontext window: {}\nmax output tokens: {}\n",
        model_id(card),
        card["display_name"].as_str().unwrap_or(""),
        count(&specs["context_window"]),
        count(&specs["max_output_tokens"]),
    );
    text.push_str(&format!(
        "input modalities: {}\n",
        joined(&specs["modalities"]["input"])
    ));
    for (label, key) in [
        ("endpoints", "endpoints"),
        ("parameters", "supported_parameters"),
    ] {
        text.push_str(&format!("{label}: {}\n", joined(&specs[key])));
    }
    text.push_str(&reasoning_text(&specs["reasoning"]));
    text
}

fn reasoning_text(reasoning: &Value) -> String {
    if !reasoning.is_object() {
        return "reasoning: not supported\n".to_string();
    }
    let mut text = format!(
        "reasoning efforts: {} (default {})\nreasoning effort normalization:\n",
        joined(&reasoning["efforts"]),
        reasoning["default_effort"].as_str().unwrap_or("none")
    );
    for effort in EFFORTS {
        let level = reasoning["normalization"][effort]
            .as_str()
            .unwrap_or("rejected");
        text.push_str(&format!("  {effort:<8} -> {level}\n"));
    }
    text
}

fn count(value: &Value) -> String {
    value
        .as_i64()
        .map_or_else(|| "unknown".to_string(), |n| n.to_string())
}

fn joined(values: &Value) -> String {
    let names: Vec<&str> = values
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
        .collect();
    if names.is_empty() {
        "none".to_string()
    } else {
        names.join(", ")
    }
}
