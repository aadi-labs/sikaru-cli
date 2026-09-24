# Sikaru agents for Harbor and Pier

Run a hosted Sikaru agent with the public `sikaru exec` CLI inside the benchmark
task environment, like an installed coding CLI. Harbor/Pier own the task image,
filesystem, verifier and container cleanup. Sikaru owns the hosted agent loop.
The adapter never imports the private Harness or executes task commands on the
runner host. No new backend API or benchmark execution mode is required.

## Install and run

Install this authored package into the same Python 3.12+ environment as the
runner. It is independent of Fern-generated API clients and preserved during
client regeneration. It has not yet been published to PyPI.

```sh
uv pip install './client-extensions/eval[harbor,pier]'
export SIKARU_API_KEY=YOUR_PROJECT_KEY
export SIKARU_PROJECT_ID=YOUR_PROJECT
export SIKARU_AGENT_ID=YOUR_AGENT

harbor run -d terminal-bench@2.0 -i TASK \
  --agent sikaru_eval.harbor:SikaruAgent \
  --ak binary_path=/absolute/runner-host/linux/sikaru -r 0

pier run -p /absolute/deep-swe/tasks/TASK \
  --agent-import-path sikaru_eval.pier:SikaruAgent \
  --ak binary_path=/absolute/runner-host/linux/sikaru -r 0
```

`binary_path` uploads a pinned Linux binary from the runner host into each task
container. Match its architecture and runtime libraries to the task image; a
macOS binary cannot run in Linux. Alternatively preinstall `sikaru` in each task
image and omit this option, or set `--ak binary=/absolute/container/sikaru`.
Setup checks the version and `exec --help`. Build from a compatible CLI revision
until matching release artifacts are published; use a compatible deployed gateway.

The adapter defaults to the task image's working directory (`pwd -P`), not `/work`.
Override with `--ak workspace=/absolute/task/workspace` only when the task requires
it. Credentials travel in the CLI process environment, never command arguments.
The CLI strips `SIKARU_*` variables from task subprocess environments. As with other
installed agents, the task container and its account are a trust boundary.

Options: `project`, `agent`, `base_url` (or `SIKARU_BASE_URL`, default
`https://api.sikaru.ai`), `binary`, `binary_path`, `workspace`, `timeout_sec`
(default 3600), and `cleanup_timeout_sec` (default 60, maximum 300).
Set the CLI timeout below the runner's task deadline to reserve cleanup time.
Each agent instance starts exactly one fresh session; it never resumes or retries
an uncertain execution. Automatic improvement is not enabled by this adapter.
Task MCP servers, injected skills and trajectory loading are rejected explicitly.

Pass the runner's `--model` to select any supported Sikaru catalog model. The
adapter forwards it to `sikaru exec --print --model MODEL`; omitting it inherits
the project default. The legacy `sikaru-managed` label also means project default.
Use `SIKARU_PROJECT` and `SIKARU_AGENT` as in the CLI; the older `_ID` names
remain accepted. Model validation belongs to the public Sikaru API.

## FrontierHarness

Install this package and stage the Linux binary on the Runta runner host in your
FrontierHarness install script before freezing the golden checkpoint. Supply the
three `SIKARU_*` variables to the Harbor/Pier runner process using your secret
provisioning. Permit the Sikaru gateway hostname in Runta's outer egress policy;
the Pier adapter also declares that hostname in its agent network allowlist.
Keep the benchmark's package and verifier network access intact.
Set `--harness-topology external-service` during provisioning, not on `run-trials.sh`.

FrontierHarness supports runner command overrides, so no upstream registry patch
is necessary. Run the two suites with their respective templates (replace the
binary path with the staged Linux artifact):

```sh
FH=skills/frontierharness-eval/scripts
bash "$FH/run-trials.sh" --checkpoint YOUR_CHECKPOINT --run-id sikaru-terminal \
  --harness sikaru --tasks terminal-tasks.txt --out runs \
  --provider custom --model kimi-k3 \
  --secret-name SIKARU_API_KEY --secret-host api.sikaru.ai \
  --cmd 'harbor run -d terminal-bench@2.0 -i {task} --agent sikaru_eval.harbor:SikaruAgent --model {model} --ak binary_path=/opt/sikaru/sikaru --jobs-dir {jobs} --extra-docker-compose /work/runta-ca-overlay.yaml -r 0 -y'
```

Use a task list containing **only Terminal-Bench selections** for that command.
For DeepSWE, use a separate task selection and run ID, and replace `--cmd` with:

```sh
'pier run -p /work/deep-swe/tasks/{task} --agent-import-path sikaru_eval.pier:SikaruAgent --model {model} --ak binary_path=/opt/sikaru/sikaru --jobs-dir {jobs} -r 0'
```

Check the upstream script's current provision/provider/secret arguments as well:
[FrontierHarness reference](https://github.com/frontier-harness-eval/eval/blob/main/skills/frontierharness-eval/reference.md).
The command override passes `{model}` through the ordinary public interface. Record hosted service version/resource limits, managed
model policy, fresh-session behavior, egress and task environment in the manifest.
These runs evaluate the Sikaru service. They do not establish equivalence to the
published Kimi K3 harness-only baseline; use matched controls for comparison.
Use a catalog model name matching the intended provider route.
Disable runner retries when running these agents so a new runner instance does
not replay an uncertain trial. FrontierHarness also requires measured model
usage for a scored verifier outcome: a deployment returning unavailable usage
can execute tasks but may be classified `infra_invalid` by its collector.

## Evidence and failures

The runner's agent log directory retains `sikaru-result.json`,
`sikaru-progress.log`, and `sikaru-adapter.json`. Metadata includes available
session/run/attachment IDs, CLI version, remote evidence path, status, cleanup,
and the original public usage payload. Private harness trajectories are not
exported and the adapter does not advertise ATIF support.

Exit 0 requires `status=completed` and `cleanup=confirmed`. This means execution
finished; only the runner's verifier determines task success. Approval required,
failure, cancellation, recovery required and incomplete evidence raise execution
errors while retaining evidence. Cancellation requests TERM only after checking
the Linux process command line for this invocation's unique state path, then
waits for the CLI to drain. Transport loss never causes a relaunch. If cleanup
cannot be established, metadata stays unconfirmed; the runner owns final sandbox
teardown. State directories are retained in the task container for investigation.

Unknown usage/cost stays unknown. Recognized public usage fields
`usage.usage.n_input_tokens` (inclusive of cache), `n_cache_tokens`,
`n_output_tokens`, and `usage.cost.cost_usd` populate runner context only when
`usage.available=true`. Other schemas remain in raw evidence without guessed
conversion. FrontierHarness first-call cache normalization needs a separate
parser and public per-call evidence; this adapter does not fabricate it.

## Local verification

```sh
uv pip install './client-extensions/eval[harbor,pier,test]'
python -m pytest -q client-extensions/eval/tests
uv build client-extensions/eval
```

Tests exercise the actual pinned runner classes with subprocess/file transport
and a deterministic CLI fixture. They do not contact Sikaru or claim benchmark
scores. Start live acceptance with one task from each suite before a full sweep.

Public run summaries expose durable measured usage independently of chargeability.
`cost_usd` uses Sikaru's resource tariff; it is not provider cost or standardized
benchmark cost. `chargeable_cost_usd` is zero for complimentary receipts. Calculate
benchmark prices separately from measured tokens. Partial usage (`complete=false`)
stays in evidence and is not promoted to complete runner totals. Per-call ledger
timestamps do not establish provider execution order; first-call cache correction
requires additional ordered evidence.
