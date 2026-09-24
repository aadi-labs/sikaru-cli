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

## Everyday workflow

```sh
sikaru auth login
export SIKARU_PROJECT=YOUR_PROJECT_ID
export SIKARU_AGENT=YOUR_AGENT_SLUG
sikaru doctor
sikaru chat
sikaru exec "Fix the failing tests"
cat task.txt | sikaru exec -
sikaru exec --dry-run "Check this task before starting"
```

`--project` and `--agent` override the environment defaults. The workspace defaults
to the current directory; use `-C PATH`, `--cd PATH`, or `--workspace PATH` to
select another existing directory. The hosted Sikaru model remains managed.

`chat` is a terminal conversation with `/help`, `/status`, and `/exit`. Each
message continues the same durable session. It shows completed replies and the
state directory on stderr; a final JSON result stays on stdout. It stops on
approval, cancellation, failure, or uncertain recovery. Resume explicitly with
`sikaru chat --resume STATE_DIRECTORY` in the original workspace. This is a simple
line-based interface, not a full-screen editor or a token-by-token text renderer.

`exec` retains its single JSON result and existing exit codes. Supply one of a
positional prompt, `--prompt`, or `--prompt-file`. Use `-` as the positional prompt
or prompt-file to read stdin explicitly. Empty input fails before local or remote
state is created. File/stdin prompts must be UTF-8 and at most 1 MiB. A dry run
validates input only: it does not create journals, make requests, or verify a
saved resume journal. Global formatting options apply to generated API commands;
native commands retain their documented JSON output contract.

`doctor` checks the workspace and project read access. It does not certify
billing, agent readiness, write permissions, or sandbox isolation. No run is
started. Native request failures provide safe next steps for authentication,
access, missing resources, and billing without displaying arbitrary provider
response bodies. Preserve the state directory when recovering an interrupted run.

## Run compute on your machine

The installed `sikaru` binary bundles the native executor on macOS and Linux.
No Python runtime or compute companion is required.

```sh
sikaru exec --project PROJECT --agent AGENT --workspace /absolute/workspace --prompt "Complete this task"
sikaru exec --project PROJECT --workspace /absolute/workspace --resume /absolute/workspace/.sikaru-STATE
sikaru exec --project PROJECT --agent AGENT --workspace /absolute/workspace --prompt-file task.txt --approval-wait 300 --timeout 3600
```

Each fresh invocation creates a session, environment, attachment, and private state
directory. The result includes `state_dir` for explicit resume. Resume preserves
the original physical workspace and journal; it rejects uncertain prior effects
and never silently substitutes a directory. A new `--prompt` on resume appends or
steers a turn; omit it to observe the existing run. Setup interrupted before the
executor starts can resume idempotently with the original prompt. Environments
remain active for continuation. Files and journals remain in the caller workspace.

Final stdout is one JSON object. Progress and diagnostics use stderr. Exit codes:
0 completed, 1 failed, 2 approval required, 3 cancelled, 4 recovery required.
`cleanup` and `cancel_acknowledged` are independent: stopping children does not prove
an interrupted effect's receipt. Missing usage/cost is `available:false`.
`final_output` contains the durable product result when available. The default
approval policy parks after confirmed cleanup; `--approval-wait` waits a bounded
number of seconds for the controller's decision. INT/TERM and deadlines await
owned process cleanup before returning. A crash or SIGKILL can require explicit
controller teardown and recovery.

For an application-provisioned sandbox, use `sikaru compute serve --bootstrap -`.
Supply the scoped executor bootstrap JSON over stdin (or a private 0600 file),
never a controller credential. The bootstrap includes project/session/attachment,
owner epoch, original workspace provenance/generation, journal and credential IDs,
scoped token, workspace and state directory. Scoped serving returns the same
lifecycle fields; it cannot fetch project-wide run results, and usage is unavailable.

For customer infrastructure:

```sh
sikaru compute worker --bootstrap /private/worker.json --launcher /absolute/launcher --concurrency 4
```

Worker bootstrap fields are `project_id`, `environment_id`, `token` (restricted
worker credential), and `state_dir` (private durable directory). `--once` drains
one queue page. The worker renews only its credential, never an executor lease.
Startup remains bounded at 180 seconds; executor readiness owns the 60-second lease.

The launcher is a customer-owned executable receiving JSON on stdin. Protocol v1
requests contain `version:1`, `operation` (`launch`, `status`, `teardown`), stable
`launch_id`, project/environment/session/attachment IDs, claim, workspace generation,
original provenance, journal ID, and an optional previously proven `handle`.
Only `launch` includes the scoped `credential` and `base_url`. The launcher must
create one sandbox for the stable launch identity, retain a lookup after lost ACK,
and start `sikaru compute serve` there using the supplied identity and credentials.
Responses contain `version`, `launch_id`, `status` (`launched`, `running`,
`terminated`, `not_launched`, or `unknown`), optional `handle`, and `evidence`.
A handle is `{ "kind":"container"|"sandbox", "id":"immutable identity", "proof":"creation nonce" }`;
a bare PID is never sufficient. Teardown must prove termination with the same
handle and nonempty evidence, or prove that the launch never happened. JSON responses
are limited to 64 KiB. The worker fsyncs intent before launch and never relaunches
an ambiguous result. Preserve worker state across restarts. Revoked worker credentials
require controller teardown if reporting cleanup can no longer authenticate.

Local commands use the caller's OS permissions; a workspace is not isolation.
Task and launcher child environments strip `SIKARU_*` control credentials. The agent runs through the Sikaru service.

## Verify

```sh
cargo test --locked --bin sikaru --test sdk_retry_policy --test compute_executor --test compute_recovery --test compute_workflows --test compute_worker --test compute_contract
cargo build --locked --bin sikaru
node scripts/check-cli.mjs target/debug/sikaru
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
The `sikaru` binary includes generated API commands and authored native execution commands.

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

### Native attachment executor

On macOS and Linux, `sikaru compute serve --bootstrap -` serves one already
claimed attachment using JSON on stdin. `--bootstrap /absolute/private.json`
also accepts an owned regular file with mode `0600`. The bootstrap contains
`project_id`, `session_id`, `attachment_id`, `owner_epoch`, `workspace_generation`,
`journal_id`, `credential_id`, `token`, `workspace_provenance` (`kind` and opaque
`identity`), `workspace`, and `state_dir`. An optional `command_timeout_seconds`
is between 1 and 86400 (default 120). Pass credentials through stdin or the
private file, never arguments. `--base-url` and shared TLS/proxy configuration
apply to the generated SDK transport.

The workspace must already exist. The state directory must have an existing
parent, contain no symlink path components, and be private to the current user;
the executor creates it with mode `0700` when absent. Canonicalize temporary
paths on macOS (for example `/private/var/...`). Keep the state directory across
reconnections. It holds an exclusively locked, fsynced operation journal and
output artifacts. Task files remain in the workspace. The workspace is not a
security sandbox; execution trusts the host and the account running the CLI.
Task environments omit `SIKARU_*`, including task-provided values, while retaining
customer infrastructure configuration.

A process restart requires confirmed teardown and a new claim/epoch/credential.
The new scoped API status must match the original attachment, session, journal,
workspace generation and provenance; the journal additionally checks the local
workspace device/inode. Persisted numeric process IDs never authorize signals.
An intent without an immutable receipt requires recovery and is never replayed.
A lost acknowledgement permits resending the identical receipt. Caller-owned
launchers must prove teardown before issuing a replacement claim.

The final stdout JSON reports `status`, durable `execution` when available,
`cleanup`, `cancel_acknowledged`, and `usage.available` (false when unavailable).
Progress belongs on stderr. Exit codes are 0 for completed, 1 for failed,
2 for approval required, 3 for cancellation, and 4 for recovery required.
Cancellation with an interrupted operation can report recovery required while
separately confirming cancellation acknowledgement and process teardown.
Empty work pages do not indicate completion. SIGINT and SIGTERM await owned
process-group cleanup. Output artifacts retain up to 1 MiB per process; reaching
that limit terminates the process and reports truncation. Output pages use byte
offsets, decode UTF-8 lossily, and cap raw pages at 24 KiB so escaped control
characters remain within the receipt's serialized size limit.
