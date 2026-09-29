# Changelog

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
