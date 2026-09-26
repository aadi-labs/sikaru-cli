# Changelog

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
