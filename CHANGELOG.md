# Changelog

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
