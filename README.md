# Sikaru CLI

Official command-line client for the Sikaru public API.
Use it to manage agents and sessions, start runs, inspect events, submit tool
results, and work with files and artifacts from scripts or your terminal.

## Install

macOS and Linux:

```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/aadi-labs/sikaru-cli/releases/download/v0.1.0/sikaru-cli-installer.sh | sh
```

Windows PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/aadi-labs/sikaru-cli/releases/download/v0.1.0/sikaru-cli-installer.ps1 | iex"
```

Prebuilt archives and checksums are available on the
[releases page](https://github.com/aadi-labs/sikaru-cli/releases).
The installer places `sikaru` in your Cargo bin directory (normally `~/.cargo/bin`).

## Build from source

```sh
cargo build --locked --release
./target/release/sikaru --help
```

The CLI is a standalone Rust binary.

## Use

Set `SIKARU_API_KEY` in your environment. Use `--base-url` to override the API
endpoint. Discover operations and request fields with help or machine-readable
schemas:

```sh
sikaru agents --help
sikaru runs --help
sikaru runs start --help
sikaru runs start --schema
```

Commands support `--dry-run` to validate requests without sending them, and
`--format json` for scripts. Use `--help` on an operation for its required fields.
Mutation requests are not automatically retried.

For customer-owned compute, start a run or append a session turn with
`execution_environment: "local"`, `compute_provider_id`, a `compute.execute`
capability grant, and a matching tool provider reference with capability prefix
`compute`. Managed compute remains the default. Register and attach the public
compute capability through the ordinary API commands:

```sh
sikaru tool-providers register_tool_provider --project-id PROJECT --json '{"provider_type":"custom","display_name":"My computer","capability_prefix":"compute","tool_catalog_ref":"inline://local-compute","broker_endpoint_ref":"inline://local-compute"}'
# Use toolProviderId from that response as PROVIDER below.
sikaru tool-providers attach_source_tool_skill --project-id PROJECT --tool-provider-id PROVIDER --json '{"description":"Local process and workspace operations","capability_refs":["compute.execute"],"source":{"kind":"markdown","content":"Execute local compute operations through compute.execute with method and arguments."}}'
```

Use the returned source skill reference in the run's provider reference when
selecting source skills explicitly. Inspect `sikaru runs start --schema` for the
full start body and required agent input fields.

## Run compute on your machine

Install the generated `sikaru` binary and this companion (Python 3.11+, macOS or
Linux):

```sh
pipx install "git+https://github.com/aadi-labs/sikaru-cli.git@v0.1.0#subdirectory=client-extensions/cli"
sikaru-compute --project-id PROJECT --run-id RUN --provider-id PROVIDER \
  --workspace /absolute/path/to/workspace --state-dir /absolute/path/to/new-run-state
```

The companion connects to an existing local-compute run. It polls the hosted
run through the generated CLI and executes admitted `compute.execute` requests:
`bash.start`, `bash.read`, `bash.wait`, `bash.cancel`, and `workspace.write_text`.
The agent and its instructions continue running on Sikaru. Set `SIKARU_API_KEY`
as for the CLI; `--cli` selects its executable and `--base-url` selects an endpoint.

Commands default to the supplied workspace and execute with your user's machine
permissions and environment (excluding inherited `SIKARU_*` variables). The workspace is a working directory, not a sandbox.
Use a dedicated account or isolated machine when appropriate. Output is bounded
by `--output-limit` (default 65536 bytes), command lifetime by `--command-timeout`
(default 120 seconds), and connection lifetime by `--timeout` (default 3600 seconds).
Hosted termination, loss of status visibility, interruption, or failure stops local
process groups. Graceful cleanup cannot run after a machine crash or SIGKILL.

Use one companion per run and a fresh state directory. The fsynced journal records
execution before dispatch and receipts before submission. Only identical receipts
are retried; commands are never automatically replayed. An existing journal is
refused, including after an uncertain interruption: inspect it and reconcile the
hosted run before starting a new run. Do not reconnect an interrupted run with a
new state directory. Output and journals are retained locally for inspection.

The Sikaru service team maintains the API contract.
This repository contains public client code only. Language clients are available
in [sikaru-sdk](https://github.com/aadi-labs/sikaru-sdk).

## Verify

```sh
cargo test --locked --bin sikaru --test sdk_retry_policy
cargo build --locked --bin sikaru
node scripts/check-cli.mjs target/debug/sikaru
SIKARU_TEST_CLI="$PWD/target/debug/sikaru" python3 -m unittest discover -s tests -p test_local_compute.py
```
