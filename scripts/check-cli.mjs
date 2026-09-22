// Local public API command checks; no request is sent to a service.
import assert from "node:assert/strict";
import { spawnSync } from "node:child_process";
import { resolve } from "node:path";

const binary = resolve(process.argv[2] ?? "target/debug/sikaru");
const env = { ...process.env };
for (const key of Object.keys(env)) {
  if (key.startsWith("SIKARU_")) delete env[key];
}
env.SIKARU_API_KEY = "local-validation-only";
function run(args) {
  const result = spawnSync(binary, args, { env, encoding: "utf8", timeout: 15_000 });
  if (result.error) throw result.error;
  assert.equal(result.status, 0, result.stderr);
  return result.stdout;
}
const help = run(["--help"]);
for (const command of ["agents", "execution-sessions", "runs", "tool-providers", "agent-budgets", "specialists"]) {
  assert.ok(help.includes(command));
  assert.match(run([command, "--help"]), /Commands:/);
}
assert.match(run(["runs", "start", "--help"]), /--project-id/);
assert.doesNotThrow(() => JSON.parse(run(["runs", "start", "--schema"])));
const preview = run(["runs", "get", "--project-id", "project_example", "--run-id", "run_example", "--base-url", "http://127.0.0.1:1", "--dry-run", "--format", "json"]);
assert.ok(preview.includes("project_example"));
assert.ok(preview.includes("run_example"));
for (const [resource, methods] of Object.entries({
  "agent-budgets": ["get", "add", "configure_auto_reload", "setup_payment_method"],
  "specialists": ["list", "get", "message", "cancel"],
  "execution-sessions": ["spend"],
})) {
  for (const method of methods) {
    assert.doesNotThrow(() => JSON.parse(run([resource, method, "--schema"])));
  }
}
const funding = run(["agent-budgets", "add", "--project-id", "project_example", "--harness-id", "agent_example",
  "--amount-usd", "5.00", "--idempotency-key", "local-funding-check", "--base-url", "http://127.0.0.1:1", "--dry-run", "--format", "json"]);
assert.ok(funding.includes("local-funding-check"));
if (process.platform !== "win32") {
  assert.match(run(["exec", "--help"]), /--resume/);
  assert.match(run(["compute", "serve", "--help"]), /--bootstrap/);
  assert.match(run(["compute", "worker", "--help"]), /--launcher/);
}
console.log("Public API help, operation schema, and offline request validation passed.");
