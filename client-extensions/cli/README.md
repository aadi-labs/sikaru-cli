# Sikaru CLI

Official command-line client for the Sikaru public API.
Use it to manage agents and sessions, start runs, inspect events, submit tool
results, and work with files and artifacts from scripts or your terminal.

## Install

Install the latest release on macOS or Linux:

```sh
curl -fsSL https://sikaru.ai/install.sh | sh
```

Or install with npm (Node.js 20+, also supports Windows):

```sh
npm install -g sikaru-cli
sikaru --help
```

Or run without a global installation:

```sh
npx sikaru-cli@latest --help
```

Standalone binaries and installers are available on the
[releases page](https://github.com/aadi-labs/sikaru-cli/releases/latest).

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

## Automatic harness improvement

Add `--auto-improve true` when starting a run:

```sh
sikaru runs start --project-id PROJECT --harness-id AGENT \
  --tenant-id TENANT --user-id USER --input '{"goal":"Complete the task"}' \
  --product-context '{}' --policy '{}' --auto-improve true
```

Or enable it for the turns of a new session:

```sh
sikaru execution-sessions create --project-id PROJECT --harness-id AGENT \
  --tenant-id TENANT --user-id USER --auto-improve true
```

The default is off; `--auto-improve false` explicitly disables it at startup.
This requires `runs:create`, `harness:write`, and an agent configured for
improvement. Completed turns trigger background improvement after their evidence
is graded and the configured evaluation cases are ready. Work continues when the
CLI exits. Improvement uses additional model and evaluation compute.

Only evaluated improvements can become active. Running sessions keep their
pinned release; start a new session to use an approved release. A job requiring
review pauses further automatic campaigns for that agent. Inspect progress with:

```sh
sikaru harnesses list_improvements --project-id PROJECT --harness-id AGENT
```

The service must support automatic improvement and have its learning worker
enabled. This flag does not run an optimizer on your computer.

### Author an agent

Install the authored companion separately: `cargo install --path client-extensions/authoring --locked`.
The generated `sikaru` binary contains only API commands.

```sh
sikaru-authoring init my-agent --name my-agent
# Edit my-agent/instructions.md and explicitly list any additional sources.
sikaru-authoring check my-agent
sikaru-authoring dev my-agent --project PROJECT_ID --tenant TENANT_ID --user USER_ID --prompt 'Hello'
```

`init` creates a new directory and never overwrites an existing directory.
`check` reads only the files listed in `sikaru.json`, prints the customer definition
and its canonical SHA-256 digest, and makes no API request. Keep sensitive files
out of the source manifest. Paths must be relative; source paths cannot contain
symlinks. The complete definition is limited to 1 MB and 100 files.

```json
{
  "name": "my-agent",
  "sources": [
    {"path": "instructions.md", "kind": "agent_md"},
    {"path": "skills/research/SKILL.md", "kind": "agent_skill"},
    {"path": "skills/research/assets/input.csv", "kind": "skill_asset"},
    {"path": "evals/acceptance.md", "kind": "eval_md"}
  ]
}
```

Source instructions and skills are Markdown. Skill assets under `scripts/`,
`references/`, or `assets/` are encoded as base64 and require their package's
`SKILL.md` in the manifest. Named agents use `agents/NAME/instructions.md` and
`agents/NAME/skills/...`. The manifest contains customer-authored source only.

`dev` creates an inactive draft and a hosted session explicitly bound to the draft
environment. It uses the same generated API executor, credentials and base URL as
other commands. A short source digest is appended to the draft slug; editing
source creates a separate immutable draft. Repeating unchanged source reuses that
draft and creates a new session. The optional prompt queues one turn and returns
its IDs; this command does not wait for completion or start an interactive chat.
Use the execution-session and run commands to follow or continue the session.
Draft testing requires the server's draft execution API and appropriate project
permissions. This workflow does not activate a production release.
