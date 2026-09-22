# Publishing the CLI

The release workflow builds platform archives and an npm installer package for each
version tag. Keep the Cargo package version and tag aligned. Never move a published
tag or replace its binaries.

After the GitHub release succeeds, download its `sikaru-cli-npm-package.tar.gz`
asset and publish it with an authenticated npm maintainer account:

```sh
npm publish ./sikaru-cli-npm-package.tar.gz --access public --tag latest
```

Users install or update the current npm release with `npm install -g sikaru-cli`.
The npm package downloads the platform binary from its matching GitHub release.
For prereleases, use a separate npm dist-tag such as `next` instead of `latest`.

Automated npm publishing is not enabled until repository publishing credentials
or npm trusted publishing are configured. Generating an npm asset alone does not
publish it to the registry.

## Coordinated self-hosted compute adoption

Stage the gateway contract/migration, all six generated SDKs, and the native CLI
from matching inputs before publication. Preserve authored compute module/test
paths in both `.fernignore` and the backend publisher's exact lists. Do not edit
generated Cargo manifests/locks/types/transports to make a release pass. Retain
the independent source-authoring package; the Python compute package is retired.

Before tagging, verify a clean installation on macOS and Linux:

```sh
cargo install --locked --debug --path . --root /tmp/sikaru-installed
SIKARU_TEST_INSTALLED=/tmp/sikaru-installed/bin/sikaru cargo test --locked --test compute_packaged_acceptance
node scripts/check-cli.mjs /tmp/sikaru-installed/bin/sikaru
```

The packaged suite exercises real subprocess effects, scoped generated transport,
receipt replay, approval, cancellation and launcher failure. A Python launcher
fixture is a test dependency only; installed local execution runs without Python.
Also run the backend's installed-native local gateway/harness acceptance, the
polling-only Ruby contract and the existing generation/docs/private-export gates.
Record exact artifact hashes and operating systems. Local fixture evidence is
not live hosted acceptance or proof of deployed migration compatibility.

Publish compatible packages before announcing released documentation. Source
quickstarts must continue to say build/install from the repository until release
assets actually exist. Publication, hosted acceptance and production rollout
require explicit authorization; these local commands perform none of them.
