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
