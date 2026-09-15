# Sikaru API CLI Reference

Full command reference for `sikaru`.

## Commands

- [`sikaru activation`](#sikaru-activation)
- [`sikaru agent-imports`](#sikaru-agent-imports)
- [`sikaru agents`](#sikaru-agents)
- [`sikaru changesets`](#sikaru-changesets)
- [`sikaru context-registry`](#sikaru-context-registry)
- [`sikaru conversations`](#sikaru-conversations)
- [`sikaru deployments`](#sikaru-deployments)
- [`sikaru environments`](#sikaru-environments)
- [`sikaru eval-seeds`](#sikaru-eval-seeds)
- [`sikaru evaluation-comparisons`](#sikaru-evaluation-comparisons)
- [`sikaru evaluation-criteria`](#sikaru-evaluation-criteria)
- [`sikaru evaluation-jobs`](#sikaru-evaluation-jobs)
- [`sikaru evaluation-results`](#sikaru-evaluation-results)
- [`sikaru evaluator-runs`](#sikaru-evaluator-runs)
- [`sikaru execution-objectives`](#sikaru-execution-objectives)
- [`sikaru execution-sessions`](#sikaru-execution-sessions)
- [`sikaru executions`](#sikaru-executions)
- [`sikaru feedback`](#sikaru-feedback)
- [`sikaru harness-versions`](#sikaru-harness-versions)
- [`sikaru harnesses`](#sikaru-harnesses)
- [`sikaru import-sessions`](#sikaru-import-sessions)
- [`sikaru issue-clusters`](#sikaru-issue-clusters)
- [`sikaru judge-alignment`](#sikaru-judge-alignment)
- [`sikaru managed-agents`](#sikaru-managed-agents)
- [`sikaru memory-registry`](#sikaru-memory-registry)
- [`sikaru model-gateway`](#sikaru-model-gateway)
- [`sikaru model-settings`](#sikaru-model-settings)
- [`sikaru online-evaluations`](#sikaru-online-evaluations)
- [`sikaru release-watches`](#sikaru-release-watches)
- [`sikaru retention-policies`](#sikaru-retention-policies)
- [`sikaru review-queue`](#sikaru-review-queue)
- [`sikaru run-schedules`](#sikaru-run-schedules)
- [`sikaru run-webhooks`](#sikaru-run-webhooks)
- [`sikaru runs`](#sikaru-runs)
- [`sikaru sessions`](#sikaru-sessions)
- [`sikaru tool-providers`](#sikaru-tool-providers)
- [`sikaru trace-import-connections`](#sikaru-trace-import-connections)
- [`sikaru trace-imports`](#sikaru-trace-imports)
- [`sikaru trace-streams`](#sikaru-trace-streams)
- [`sikaru workflow-intents`](#sikaru-workflow-intents)
- [`sikaru workflow-runs`](#sikaru-workflow-runs)
- [`sikaru workflows`](#sikaru-workflows)

---

### `sikaru activation`

#### `sikaru activation project-activation-status`

Project Activation Status

`GET /v1/projects/{project_id}/activation`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru agent-imports`

#### `sikaru agent-imports create-agent-import`

Create Agent Import

`POST /v1/projects/{project_id}/agent-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru agent-imports list-agent-imports`

List Agent Imports

`GET /v1/projects/{project_id}/agent-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru agents`

#### `sikaru agents create-managed-session`

Create Managed Session

`POST /v1/projects/{project_id}/agents/{agent_id}/sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--agent-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru changesets`

#### `sikaru changesets approve-changeset`

Approve Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/approve`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets create-changeset`

Create Changeset

`POST /v1/projects/{project_id}/changesets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets get-changeset`

Get Changeset

`GET /v1/projects/{project_id}/changesets/{changeset_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru changesets list-changeset-diffs`

List Changeset Diffs

`GET /v1/projects/{project_id}/changesets/{changeset_id}/diffs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru changesets list-changeset-evidence`

List Changeset Evidence

`GET /v1/projects/{project_id}/changesets/{changeset_id}/evidence`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |

#### `sikaru changesets list-changesets`

List Changesets

`GET /v1/projects/{project_id}/changesets`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--status` | `string` | No |  |

#### `sikaru changesets promote-changeset`

Promote Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/promote`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets reject-changeset`

Reject Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/reject`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets rollback-changeset`

Rollback Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/rollback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru changesets stage-changeset`

Stage Changeset

`POST /v1/projects/{project_id}/changesets/{changeset_id}/stage`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--changeset-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `sikaru context-registry`

#### `sikaru context-registry create-context-registry-change`

Create Context Registry Change

`POST /v1/projects/{project_id}/context-registry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru conversations`

#### `sikaru conversations list-messages`

List Messages

`GET /v1/projects/{project_id}/conversations/{conversation_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--conversation-id` | `string` | Yes |  |
| `--account-id` | `string` | Yes |  |
| `--limit` | `integer` | No |  |
| `--cursor` | `string` | No |  |

#### `sikaru conversations record-message`

Record Message

`POST /v1/projects/{project_id}/conversations/{conversation_id}/messages`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--conversation-id` | `string` | Yes |  |
| `--account-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru deployments`

#### `sikaru deployments list-console-deployments`

List Console Deployments

`GET /v1/projects/{project_id}/deployments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru environments`

#### `sikaru environments create-managed-environment`

Create Managed Environment

`POST /v1/projects/{project_id}/environments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru environments list-managed-environments`

List Managed Environments

`GET /v1/projects/{project_id}/environments`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru eval-seeds`

#### `sikaru eval-seeds create-eval-seed`

Create Eval Seed

`POST /v1/projects/{project_id}/eval-seeds`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru evaluation-comparisons`

#### `sikaru evaluation-comparisons cancel-comparison`

Cancel Comparison

`POST /v1/projects/{project_id}/evaluation-comparisons/{comparison_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--comparison-id` | `string` | Yes |  |

#### `sikaru evaluation-comparisons create-comparison`

Create Comparison

`POST /v1/projects/{project_id}/evaluation-comparisons`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru evaluation-comparisons get-comparison`

Get Comparison

`GET /v1/projects/{project_id}/evaluation-comparisons/{comparison_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--comparison-id` | `string` | Yes |  |

#### `sikaru evaluation-comparisons list-comparisons`

List Comparisons

`GET /v1/projects/{project_id}/evaluation-comparisons`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--after` | `string` | No |  |

---

### `sikaru evaluation-criteria`

#### `sikaru evaluation-criteria list-criteria`

List Criteria

`GET /v1/projects/{project_id}/evaluation-criteria`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--after` | `string` | No |  |

---

### `sikaru evaluation-jobs`

#### `sikaru evaluation-jobs cancel-job`

Cancel Job

`POST /v1/projects/{project_id}/evaluation-jobs/{job_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |

#### `sikaru evaluation-jobs create-job`

Create Job

`POST /v1/projects/{project_id}/evaluation-jobs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru evaluation-jobs get-job`

Get Job

`GET /v1/projects/{project_id}/evaluation-jobs/{job_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |

#### `sikaru evaluation-jobs list-jobs`

List Jobs

`GET /v1/projects/{project_id}/evaluation-jobs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cursor` | `string` | No |  |

---

### `sikaru evaluation-results`

#### `sikaru evaluation-results list-results`

List Results

`GET /v1/projects/{project_id}/evaluation-results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--environment` | `production | staging | development` | No |  |
| `--evaluator` | `string` | No |  |
| `--verdict` | `string` | No |  |
| `--limit` | `integer` | No |  |
| `--cursor` | `string` | No |  |

#### `sikaru evaluation-results record-result`

Record Result

`POST /v1/projects/{project_id}/evaluation-results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru evaluator-runs`

#### `sikaru evaluator-runs create-evaluator-run`

Create Evaluator Run

`POST /v1/projects/{project_id}/evaluator-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru execution-objectives`

#### `sikaru execution-objectives cancel`

Cancel

`POST /v1/projects/{project_id}/execution-objectives/{objective_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |

#### `sikaru execution-objectives create`

Create

`POST /v1/projects/{project_id}/execution-objectives`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-objectives get`

Get

`GET /v1/projects/{project_id}/execution-objectives/{objective_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |

#### `sikaru execution-objectives list-objectives`

List Objectives

`GET /v1/projects/{project_id}/execution-objectives`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | No |  |
| `--status` | `string` | No |  |
| `--after` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru execution-objectives pause`

Pause

`POST /v1/projects/{project_id}/execution-objectives/{objective_id}/pause`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |

#### `sikaru execution-objectives resume`

Resume

`POST /v1/projects/{project_id}/execution-objectives/{objective_id}/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--objective-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

---

### `sikaru execution-sessions`

#### `sikaru execution-sessions append-turn`

Append Turn

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/turns`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-sessions branch`

Branch Session

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/branches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-sessions create`

Create Session

`POST /v1/projects/{project_id}/harnesses/{harness_id}/execution-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru execution-sessions delete-file`

Delete File

`DELETE /v1/projects/{project_id}/execution-sessions/{session_id}/files/{file_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--file-id` | `string` | Yes |  |

#### `sikaru execution-sessions download-file`

Download File

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/files/{file_id}/content`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--file-id` | `string` | Yes |  |

#### `sikaru execution-sessions get`

Get Session

`GET /v1/projects/{project_id}/execution-sessions/{session_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions list`

List Sessions

`GET /v1/projects/{project_id}/execution-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | No |  |
| `--after` | `string` | No |  |
| `--limit` | `integer` | No |  |
| `--agent-slug` | `string` | No |  |

#### `sikaru execution-sessions list-files`

List Files

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions list-session-inputs`

List Session Inputs

`GET /v1/projects/{project_id}/execution-sessions/{session_id}/inputs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru execution-sessions upload-file`

Upload File

`POST /v1/projects/{project_id}/execution-sessions/{session_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--filename` | `string` | Yes |  |
| `--idempotency-key` | `string` | Yes |  |
| `--content-type` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru executions`

#### `sikaru executions execution-runtime-lineage`

Execution Runtime Lineage

`GET /v1/projects/{project_id}/executions/{trace_id}/runtime`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-id` | `string` | Yes |  |
| `--account-id` | `string` | Yes |  |
| `--inference-after` | `string` | No |  |

---

### `sikaru feedback`

#### `sikaru feedback create-feedback`

Create Feedback

`POST /v1/projects/{project_id}/feedback`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru harness-versions`

#### `sikaru harness-versions create-harness-version`

Create Harness Version

`POST /v1/projects/{project_id}/harness-versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru harnesses`

#### `sikaru harnesses get-improvement`

Get Improvement

`GET /v1/projects/{project_id}/harnesses/{harness_id}/improvements/{job_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |

#### `sikaru harnesses improvement-options`

Improvement Options

`GET /v1/projects/{project_id}/harnesses/{harness_id}/improvement-options`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |

#### `sikaru harnesses list-improvements`

List Improvements

`GET /v1/projects/{project_id}/harnesses/{harness_id}/improvements`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--limit` | `integer` | No |  |

#### `sikaru harnesses resume-improvement`

Resume Improvement

`POST /v1/projects/{project_id}/harnesses/{harness_id}/improvements/{job_id}/resume`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--job-id` | `string` | Yes |  |
| `--json` | `JSON` | No | Request body as JSON (or use individual body-field flags) |

#### `sikaru harnesses start-improvement`

Start Improvement

`POST /v1/projects/{project_id}/harnesses/{harness_id}/improvements`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru harnesses train-model-stub`

Reserved, unavailable model-training step; no learning job is submitted.

`POST /v1/projects/{project_id}/harnesses/{harness_id}/training`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |

---

### `sikaru import-sessions`

#### `sikaru import-sessions create-compatibility-profile`

Create Compatibility Profile

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/compatibility-profile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-import-session`

Create Import Session

`POST /v1/projects/{project_id}/import-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-replay-run`

Create Replay Run

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/replay-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-source-artifact`

Create Source Artifact

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/source-artifacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions create-staging-run`

Create Staging Run

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/staging-runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru import-sessions get-import-session`

Get Import Session

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions get-parity-report`

Get Parity Report

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}/parity-report`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions list-import-session-diffs`

List Import Session Diffs

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}/diffs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions list-import-sessions`

List Import Sessions

`GET /v1/projects/{project_id}/import-sessions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru import-sessions list-source-artifacts`

List Source Artifacts

`GET /v1/projects/{project_id}/import-sessions/{import_session_id}/source-artifacts`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |

#### `sikaru import-sessions promote-import-session`

Promote Import Session

`POST /v1/projects/{project_id}/import-sessions/{import_session_id}/promote`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--import-session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru issue-clusters`

#### `sikaru issue-clusters get-issue-cluster`

Get Issue Cluster

`GET /v1/projects/{project_id}/issue-clusters/{cluster_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cluster-id` | `string` | Yes |  |

#### `sikaru issue-clusters list-issue-clusters`

List Issue Clusters

`GET /v1/projects/{project_id}/issue-clusters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--status` | `string` | No |  |
| `--severity` | `string` | No |  |

#### `sikaru issue-clusters mine-project-issue-clusters`

Mine Project Issue Clusters

`POST /v1/projects/{project_id}/issue-clusters/mine`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--since` | `string` | No |  |
| `--until` | `string` | No |  |

#### `sikaru issue-clusters propose-issue-cluster-fix`

Propose Issue Cluster Fix

`POST /v1/projects/{project_id}/issue-clusters/{cluster_id}/propose-fix`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cluster-id` | `string` | Yes |  |

#### `sikaru issue-clusters update-issue-cluster-status`

Update Issue Cluster Status

`PATCH /v1/projects/{project_id}/issue-clusters/{cluster_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--cluster-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru issue-clusters upsert-issue-cluster`

Upsert Issue Cluster

`POST /v1/projects/{project_id}/issue-clusters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru judge-alignment`

#### `sikaru judge-alignment get-judge-alignment`

Get Judge Alignment

`GET /v1/projects/{project_id}/judge-alignment`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--evaluator` | `string` | Yes |  |
| `--revision` | `string` | Yes |  |
| `--environment` | `production | staging | development` | No |  |

---

### `sikaru managed-agents`

#### `sikaru managed-agents create-managed-agent`

Create Managed Agent

`POST /v1/projects/{project_id}/managed-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru managed-agents list-managed-agents`

List Managed Agents

`GET /v1/projects/{project_id}/managed-agents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru memory-registry`

#### `sikaru memory-registry create-memory-registry-change`

Create Memory Registry Change

`POST /v1/projects/{project_id}/memory-registry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru model-gateway`

#### `sikaru model-gateway capture-model-gateway-chat-completion`

Capture Model Gateway Chat Completion

`POST /v1/projects/{project_id}/model-gateway/{provider}/chat/completions/capture`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--provider` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru model-settings`

#### `sikaru model-settings get-model-settings`

Get Model Settings

`GET /v1/projects/{project_id}/model-settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru model-settings update-model-settings`

Update Model Settings

`PUT /v1/projects/{project_id}/model-settings`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru online-evaluations`

#### `sikaru online-evaluations create-policy`

Create Policy

`POST /v1/projects/{project_id}/online-evaluations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru online-evaluations list-policies`

List Policies

`GET /v1/projects/{project_id}/online-evaluations`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--after` | `string` | No |  |

#### `sikaru online-evaluations preview-policy-eligibility`

Preview Policy Eligibility

`GET /v1/projects/{project_id}/online-evaluations/preview`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--environment` | `production | staging | development` | No |  |

#### `sikaru online-evaluations update-policy`

Update Policy

`PATCH /v1/projects/{project_id}/online-evaluations/{policy_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--policy-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru release-watches`

#### `sikaru release-watches create-release-watch`

Create Release Watch

`POST /v1/projects/{project_id}/release-watches`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru retention-policies`

#### `sikaru retention-policies create-retention-policy-update`

Create Retention Policy Update

`POST /v1/projects/{project_id}/retention-policies`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru review-queue`

#### `sikaru review-queue create-review-queue-item`

Create Review Queue Item

`POST /v1/projects/{project_id}/review-queue`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru run-schedules`

#### `sikaru run-schedules create-schedule`

Create Schedule

`POST /v1/projects/{project_id}/run-schedules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru run-schedules delete-schedule`

Delete Schedule

`DELETE /v1/projects/{project_id}/run-schedules/{schedule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |

#### `sikaru run-schedules list-schedules`

List Schedules

`GET /v1/projects/{project_id}/run-schedules`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | No |  |

#### `sikaru run-schedules pause-schedule`

Pause Schedule

`PATCH /v1/projects/{project_id}/run-schedules/{schedule_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--schedule-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru run-webhooks`

#### `sikaru run-webhooks create-webhook`

Create Webhook

`POST /v1/projects/{project_id}/run-webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru run-webhooks delete-webhook`

Delete Webhook

`DELETE /v1/projects/{project_id}/run-webhooks/{webhook_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--webhook-id` | `string` | Yes |  |

#### `sikaru run-webhooks list-webhooks`

List Webhooks

`GET /v1/projects/{project_id}/run-webhooks`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru runs`

#### `sikaru runs cancel`

Cancel Run

`POST /v1/projects/{project_id}/runs/{run_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs decide-approval`

Decide Approval

`POST /v1/projects/{project_id}/runs/{run_id}/tool-calls/{tool_call_id}/approval`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--tool-call-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs events`

Managed Run Events

`GET /v1/projects/{project_id}/runs/{run_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--limit` | `string` | No |  |
| `--stream` | `string` | No |  |
| `--last-event-id` | `string` | No |  |

#### `sikaru runs get`

Managed Run Summary

`GET /v1/projects/{project_id}/runs/{run_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs get-trajectory`

Read retained ATIF structure and usage with private content redacted.

This is a committed snapshot and can be partial while a run is active or
interrupted. Messages, reasoning, tool payloads and provider metadata are
omitted. No trajectory is synthesized when retained evidence is unavailable.

`GET /v1/projects/{project_id}/runs/{run_id}/trajectory`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs pending-actions`

Pending Actions

`GET /v1/projects/{project_id}/runs/{run_id}/actions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru runs recover`

Recover Managed Run

`POST /v1/projects/{project_id}/runs/{run_id}/recover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs start`

Start Managed Harness Run

`POST /v1/projects/{project_id}/harnesses/{harness_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--harness-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru runs submit-tool-result`

Submit Tool Result

`POST /v1/projects/{project_id}/runs/{run_id}/tool-results`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru sessions`

#### `sikaru sessions create-managed-interpreter`

Create Managed Interpreter

`POST /v1/projects/{project_id}/sessions/{session_id}/interpreters`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru sessions execute-managed-interpreter`

Execute Managed Interpreter

`POST /v1/projects/{project_id}/sessions/{session_id}/interpreters/{interpreter_id}/execute`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--interpreter-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru sessions get-managed-session`

Get Managed Session

`GET /v1/projects/{project_id}/sessions/{session_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru sessions list-managed-session-events`

List Managed Session Events

`GET /v1/projects/{project_id}/sessions/{session_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--after` | `string` | No |  |
| `--limit` | `string` | No |  |

#### `sikaru sessions list-managed-session-files`

List Managed Session Files

`GET /v1/projects/{project_id}/sessions/{session_id}/files`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru sessions list-managed-session-plan`

List Managed Session Plan

`GET /v1/projects/{project_id}/sessions/{session_id}/plan`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |

#### `sikaru sessions start-managed-sandbox-execution`

Start Managed Sandbox Execution

`POST /v1/projects/{project_id}/sessions/{session_id}/sandbox-executions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--session-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru tool-providers`

#### `sikaru tool-providers attach-source-tool-skill`

Attach Source Tool Skill

`POST /v1/projects/{project_id}/tool-providers/{tool_provider_id}/skills`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--tool-provider-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru tool-providers register-tool-provider`

Register Tool Provider

`POST /v1/projects/{project_id}/tool-providers`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru trace-import-connections`

#### `sikaru trace-import-connections list-trace-import-connections`

List Trace Import Connections

`GET /v1/projects/{project_id}/trace-import-connections`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

---

### `sikaru trace-imports`

#### `sikaru trace-imports cancel-trace-import`

Cancel Trace Import

`POST /v1/projects/{project_id}/trace-imports/{trace_import_id}/cancel`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru trace-imports create-trace-import`

Create Trace Import

`POST /v1/projects/{project_id}/trace-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru trace-imports get-trace-import`

Get Trace Import

`GET /v1/projects/{project_id}/trace-imports/{trace_import_id}`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |

#### `sikaru trace-imports get-trace-import-receipt`

Get Trace Import Receipt

`GET /v1/projects/{project_id}/trace-imports/{trace_import_id}/receipt`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |

#### `sikaru trace-imports list-trace-imports`

List Trace Imports

`GET /v1/projects/{project_id}/trace-imports`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |

#### `sikaru trace-imports plan-trace-import`

Plan Trace Import

`POST /v1/projects/{project_id}/trace-imports/plan`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru trace-imports retry-trace-import`

Retry Trace Import

`POST /v1/projects/{project_id}/trace-imports/{trace_import_id}/retry`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--trace-import-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru trace-streams`

#### `sikaru trace-streams stream-openinference-spans`

Stream Openinference Spans

`POST /v1/trace-streams`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--idempotency-key` | `string` | No |  |
| `--x-sikaru-client-id` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru workflow-intents`

#### `sikaru workflow-intents compile-project-workflow-intent`

Compile Project Workflow Intent

`POST /v1/projects/{project_id}/workflow-intents/{intent_id}/compile`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--intent-id` | `string` | Yes |  |

#### `sikaru workflow-intents create-project-workflow-intent`

Create Project Workflow Intent

`POST /v1/projects/{project_id}/workflow-intents`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru workflow-runs`

#### `sikaru workflow-runs project-workflow-run-events`

Project Workflow Run Events

`GET /v1/projects/{project_id}/workflow-runs/{run_id}/events`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |

#### `sikaru workflow-runs recover-project-workflow-run`

Recover Project Workflow Run

`POST /v1/projects/{project_id}/workflow-runs/{run_id}/recover`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--run-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

### `sikaru workflows`

#### `sikaru workflows create-project-workflow-version`

Create Project Workflow Version

`POST /v1/projects/{project_id}/workflows/{workflow_id}/versions`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--workflow-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru workflows export-product-workflow`

Export Product Workflow

`GET /v1/projects/{project_id}/workflows/{workflow_id}/export`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--workflow-id` | `string` | Yes |  |

#### `sikaru workflows import-workflow`

Import Workflow

`POST /v1/projects/{project_id}/workflows/import`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

#### `sikaru workflows start-project-workflow-run`

Start Project Workflow Run

`POST /v1/projects/{project_id}/workflows/{workflow_id}/runs`

| Flag | Type | Required | Description |
|------|------|----------|-------------|
| `--project-id` | `string` | Yes |  |
| `--workflow-id` | `string` | Yes |  |
| `--idempotency-key` | `string` | No |  |
| `--json` | `JSON` | Yes | Request body as JSON (or use individual body-field flags) |

---

## Global flags

These flags are available on every command:

| Flag | Description |
|------|-------------|
| `--dry-run` | Print the HTTP request without sending it |
| `--json <JSON\|->` | Supply the request body as JSON (or `-` for stdin) |
| `--params <JSON>` | Merge extra parameters as JSON |
| `--format <json\|table\|yaml\|csv>` | Output format (default: `json`) |
| `--output <PATH>` | Write binary responses to a file |
| `--base-url <URL>` | Override the API base URL |
| `--page-all` | Auto-paginate and stream all results |
| `--page-limit <N>` | Max pages to fetch (default: `10`) |
| `-q, --quiet` | Suppress stdout on success |
| `-h, --help` | Print help |
| `-V, --version` | Print version |

