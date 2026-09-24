# Repository instructions

## Public repository boundary

The CLI and SDK repositories are public customer interfaces. Do not publish
serving-provider identities, internal infrastructure or deployment details,
internal benchmark adapters, runs, results, or development-work narratives in
public documentation, examples, tests, package contents, or API/CLI contracts.
Use provider-neutral model identifiers and customer-facing terminology.
Keep internal tooling in the private service repository. Make generated-interface
changes in gateway/OpenAPI/Fern inputs and regenerate; never edit generated clients.
Customer-selected integration names and supported client execution environments
may be documented when necessary for customers to use the interface.

Always apply the cyclomatic-complexity and behavior-focused-tests skills when
changing code. Preserve unrelated work. Do not commit, publish, or deploy unless
requested.
