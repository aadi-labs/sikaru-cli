# Changelog

## 0.2.13

- Compatibility: subscription commands move from `harnesses` to `organizations` and take an organization ID.
- Create and version datasets, capture permitted run content, upload examples, and export examples as NDJSON.
- Preview dataset checks, run eligible examples against an agent, and read check results.
- Compatibility: remove the former agent-budget operations; manage credits and spending caps in the dashboard.
- Create managed agents with an initial HTTP or Slack channel, manage channel bindings, and inspect delivery status.
- Invoke HTTP channels and poll their receipts using a channel credential, separately from the project API key.
- Manage personal channel connections, approvals, files, and Slack identity links.
- Edit individual agent document settings, review access, and rename managed agents.
- Refresh typed session, schedule, and transcript responses, including transcript usage and memory policy settings.
- Start new workspace checkpoint branches without copying task repository history, so ignored files from earlier commits are not uploaded. Skip files exceeding the checkpoint storage limit.
- These features require an updated managed service.

## 0.2.12

- Author agents as Markdown documents: pull and push drafts, validate access, review changes, publish with a reviewed access digest, and retrieve usage snippets.
- Use document revisions to detect conflicting edits and inspect published versions, comparisons, and suggested changes.
- Discover connection apps, update connection settings, and inspect which agents use a connection.
- Sign in for personal connections and resume conversations after connecting an account. Personal conversation content is visible only to its runner.
- These features require an updated managed service.

## 0.2.11

- Save workspace checkpoints as git commits on the session's branch in one bounded upload when a run ends, instead of one request per file. A `run_finished` event reports the end of the run before the upload.
- Checkpoints keep executable permissions and in-workspace symbolic links, apply the workspace's `.gitignore` files and default ignore rules, and start from the workspace repository's current commit.
- Checkpoints are also saved when a run is stopped, interrupted or loses its lease; failures are reported on stderr without changing the result.
- The persistent connection identifies the CLI with the same User-Agent as its other requests.
- Remove the per-file workspace upload.
- These features require an updated managed service.

## 0.2.10

- Declare an agent's reach in typed `web`, `tools` and `setup` sections of its definition: web search provider and domain allow and block lists, built-in tool enablement and approval policies, and setup packages, commands and repositories.
- Read and update project capability ceilings: sandbox egress, denied domains, disallowed built-in tools and allowed git hosts.
- Create and read definition revisions. A changed definition is staged as a draft revision that is reviewed and released through changesets. Creating an existing agent with a changed definition now stages a draft revision instead of failing.
- Store git credentials for a host, list them, and grant them to agents. Credential values are write-only and never returned.
- Create, list and run task checks for an agent, and list their results. A check verifies a run with a test script or a rubric.
- `sikaru-authoring init --capabilities` scaffolds the `web`, `tools` and `setup` sections, and `check --ceilings FILE` reports every field that exceeds the project's capability ceilings before upload.
- Files an agent writes under `outputs/` in its workspace are published as session files.
- These features require an updated managed service.

## 0.2.9

- Deliver compute operations over a persistent connection when the service offers it, falling back to polling automatically without re-running recorded operations.
- Answer shell waits as soon as the process exits instead of on a fixed interval.
- Add condition waits for process exit, file state, log patterns, TCP ports and HTTP status, and an operation that returns the next background process to finish.
- Advertise condition-wait support so the service only sends these operations to executors that can serve them.
- The persistent connection and condition waits require an updated managed service; older services keep using polling.

## 0.2.8

- Capture regular files in the selected task workspace and wait for checkpoint publication before reporting completion.
- Recover interrupted checkpoint uploads from the same retained capture without rerunning task commands.
- Reject mismatched workspace ownership and unsupported filesystem entries before accepting a checkpoint.
- Improve shell process recovery and refresh generated workspace checkpoint commands.
- Workspace checkpoints require an updated managed service. Captures include hidden regular files; keep credentials and executor state outside the selected workspace.

## 0.2.7

- Update the CLI package version; commands and public API behavior are unchanged.

## 0.2.6

- Improve diagnostic error reporting while keeping server response details private.

## 0.2.3

- Refresh the generated client for the current public API.
- Run measurements distinguish observed execution costs, retail usage, and customer charges; unavailable costs remain unknown.
- Compatible with improved argument feedback and recovery for managed execution. These behavior improvements require the updated managed service.

## 0.1.0

Initial public release of the Sikaru CLI.

- Manage agents, execution sessions, runs, tool providers, files, and artifacts.
- Select a model when starting a run and read retained ATIF trajectories.
- Inspect operation schemas and validate requests offline with `--dry-run`.
- Run customer-owned compute with the separately installable `sikaru-compute` companion.
- Distribute macOS, Linux, and Windows binaries with shell and PowerShell installers, checksums, and build provenance.
