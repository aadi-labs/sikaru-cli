# Sikaru authoring companion

The generated `sikaru` binary exposes the API contract. This separate authored
binary adds directory packaging and draft workflows through the generated API
executor and generated OpenAPI document. It does not modify generated files.

From the CLI repository:

```sh
cargo install --path client-extensions/authoring --locked
sikaru-authoring init support-agent --name support
sikaru-authoring init research-agent --name research --capabilities
sikaru-authoring check support-agent
sikaru-authoring check support-agent --ceilings ceilings.json
sikaru-authoring dev support-agent --project PROJECT --tenant TENANT --user USER --prompt 'Help this customer.'
```

Use `SIKARU_API_KEY` for authentication. `dev` creates a hosted inactive draft and
session; it does not execute an agent locally. Source compilation includes only
the manifest-listed files. Optional `web`, `tools` and `setup` manifest sections
are validated locally and packaged as authored (`init --capabilities` scaffolds
them, and the Python SDK authoring client compiles them identically); `--ceilings` reports every field
that explicitly exceeds the project's capability ceilings. The companion is built from this repository because
its dependency is the generated CLI library in the same checkout.

```sh
cargo test --manifest-path client-extensions/authoring/Cargo.toml --locked
```
