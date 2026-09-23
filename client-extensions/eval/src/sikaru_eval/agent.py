"""Launch the public CLI in the runner's environment, never on the runner host."""

from __future__ import annotations

import asyncio
import json
import math
import os
import shlex
import tempfile
from importlib.metadata import version
from pathlib import Path
from urllib.parse import urlsplit
from uuid import uuid4


class SikaruExecutionError(RuntimeError):
    """Execution did not complete cleanly; this is not a verifier verdict."""


class SikaruAgentMixin:
    SUPPORTS_ATIF = False
    SUPPORTS_WINDOWS = False

    def __init__(
        self,
        logs_dir,
        model_name=None,
        *,
        project=None,
        agent=None,
        base_url=None,
        binary="sikaru",
        binary_path=None,
        workspace=None,
        timeout_sec=3600,
        cleanup_timeout_sec=60,
        mcp_servers=None,
        skills_dir=None,
        **kwargs,
    ):
        if model_name not in (None, "sikaru-managed"):
            raise ValueError(
                "Sikaru owns model policy; omit --model or use sikaru-managed"
            )
        if mcp_servers or skills_dir or kwargs.get("load_trajectory"):
            raise ValueError("Task MCP, skills and trajectory loading are unsupported")
        allowed = {
            "logger",
            "environment_logs_dir",
            "override_setup_timeout_sec",
            "extra_env",
            "load_trajectory",
        }
        if set(kwargs) - allowed:
            raise ValueError(f"Unknown Sikaru options: {sorted(set(kwargs) - allowed)}")
        extra_env = kwargs.pop("extra_env", None) or {}
        if set(extra_env) - {"SIKARU_API_KEY"}:
            raise ValueError("Only SIKARU_API_KEY is accepted in extra_env")
        self.project = project or os.environ.get("SIKARU_PROJECT_ID")
        self.agent = agent or os.environ.get("SIKARU_AGENT_ID")
        self.base_url = base_url or os.environ.get(
            "SIKARU_BASE_URL", "https://api.sikaru.ai"
        )
        parsed = urlsplit(self.base_url)
        if (
            parsed.scheme not in {"http", "https"}
            or not parsed.hostname
            or parsed.username
            or parsed.password
        ):
            raise ValueError("base_url must be an HTTP(S) endpoint without credentials")
        if not self.project or not self.agent:
            raise ValueError(
                "Set project and agent, or SIKARU_PROJECT_ID and SIKARU_AGENT_ID"
            )
        self._key = extra_env.get("SIKARU_API_KEY") or os.environ.get("SIKARU_API_KEY")
        if not self._key:
            raise ValueError("SIKARU_API_KEY must be set in the runner environment")
        self.timeout = int(timeout_sec)
        self.cleanup_timeout = int(cleanup_timeout_sec)
        if not 1 <= self.timeout <= 86400 or not 1 <= self.cleanup_timeout <= 300:
            raise ValueError("timeout_sec must be 1..86400; cleanup_timeout_sec 1..300")
        self.binary, self.binary_path, self.workspace = binary, binary_path, workspace
        self.remote = f"/tmp/sikaru-eval-{uuid4().hex}"
        self._started = False
        self._cli_version = None
        super().__init__(logs_dir=Path(logs_dir), model_name="sikaru-managed", **kwargs)

    @staticmethod
    def name():
        return "sikaru"

    def version(self):
        return self._cli_version or f"adapter-{version('sikaru-eval')}"

    async def setup(self, environment):
        self.logs_dir.mkdir(parents=True, exist_ok=True)
        await self._checked(environment, f"umask 077; mkdir {shlex.quote(self.remote)}")
        if self.binary_path:
            self.binary = f"{self.remote}/sikaru"
            await environment.upload_file(Path(self.binary_path), self.binary)
            await self._checked(
                environment, f"chmod 755 {shlex.quote(self.binary)}", user="root"
            )
        result = await self._checked(
            environment, f"{shlex.quote(self.binary)} --version"
        )
        self._cli_version = (result.stdout or "").strip()
        await self._checked(environment, f"{shlex.quote(self.binary)} exec --help")
        if self.workspace is None:
            result = await self._checked(environment, "pwd -P")
            self.workspace = result.stdout.strip()
        if not self.workspace.startswith("/"):
            raise ValueError(
                "workspace must be an absolute path inside the task environment"
            )
        await self._checked(environment, f"test -d {shlex.quote(self.workspace)}")

    async def _checked(self, environment, command, **kwargs):
        result = await environment.exec(command=command, timeout_sec=30, **kwargs)
        if result.return_code != 0:
            raise SikaruExecutionError(
                "Sikaru environment setup or control command failed"
            )
        return result

    def _write(self, name, value):
        # Credentials never belong in retained CLI diagnostics or metadata.
        text = json.dumps(value, indent=2) if not isinstance(value, str) else value
        (self.logs_dir / name).write_text(text.replace(self._key, "[REDACTED]"))

    async def run(self, instruction, environment, context):
        if self._started:
            raise SikaruExecutionError(
                "A trial cannot be replayed; create a new agent for a new trial"
            )
        self._started = True
        with tempfile.TemporaryDirectory() as directory:
            prompt = Path(directory) / "instruction.txt"
            prompt.write_text(instruction)
            await environment.upload_file(prompt, f"{self.remote}/instruction.txt")
        args = [
            self.binary,
            "--base-url",
            self.base_url,
            "exec",
            "--project",
            self.project,
            "--agent",
            self.agent,
            "--workspace",
            self.workspace,
            "--prompt-file",
            f"{self.remote}/instruction.txt",
            "--state-dir",
            f"{self.remote}/state",
            "--timeout",
            str(self.timeout),
        ]
        # exec keeps this shell's PID. The unique state argument lets cancellation
        # verify ownership before signaling on Linux, rather than trusting a PID.
        command = (
            f"umask 077; echo $$ > {self.remote}/pid; exec {shlex.join(args)}"
            f" > {self.remote}/result.json 2> {self.remote}/progress.log"
        )
        context.metadata = {
            **(context.metadata or {}),
            "sikaru": {
                "adapter_version": version("sikaru-eval"),
                "cli_version": self._cli_version,
                "project_id": self.project,
                "agent_id": self.agent,
                "remote_evidence": self.remote,
                "cleanup": "unconfirmed",
                "model_policy": "sikaru-managed",
            },
        }
        self._write("sikaru-adapter.json", context.metadata["sikaru"])
        pending = asyncio.create_task(
            environment.exec(
                command=command,
                env={"SIKARU_API_KEY": self._key},
                timeout_sec=self.timeout + 2 * self.cleanup_timeout,
            )
        )
        try:
            result = await asyncio.wait_for(
                asyncio.shield(pending), self.timeout + self.cleanup_timeout
            )
        except (asyncio.CancelledError, Exception):
            # Do not replay after transport failure or cancellation. Best-effort
            # TERM lets the CLI cancel hosted work and drain its owned processes.
            try:
                await self._stop(environment)
                await asyncio.wait_for(asyncio.shield(pending), self.cleanup_timeout)
            except (asyncio.CancelledError, Exception) as exc:  # noqa: BLE001 -- runner transports have provider-specific errors
                context.metadata["sikaru"]["cancellation_error"] = type(exc).__name__
            finally:
                if not pending.done():
                    pending.cancel()
                await asyncio.gather(pending, return_exceptions=True)
                await self._collect(environment, context)
            raise
        await self._collect(environment, context)
        evidence = context.metadata["sikaru"]
        if (
            result.return_code != 0
            or evidence.get("status") != "completed"
            or evidence.get("cleanup") != "confirmed"
        ):
            raise SikaruExecutionError(
                f"Sikaru execution did not complete cleanly (exit={result.return_code}, "
                f"status={evidence.get('status', 'unknown')}, cleanup={evidence.get('cleanup')}); "
                "see sikaru-result.json. The task verifier determines pass/fail."
            )

    async def _stop(self, environment):
        command = (
            f"p=$(cat {self.remote}/pid) || exit 1; "
            'case "$p" in ""|*[!0-9]*) exit 1;; esac; '
            f"tr '\\000' '\\n' < /proc/$p/cmdline | grep -Fx -- {self.remote}/state "
            '>/dev/null && kill -TERM "$p"'
        )
        await self._checked(environment, command)

    async def _collect(self, environment, context):
        evidence = context.metadata["sikaru"]
        for remote, local in (
            ("result.json", "sikaru-result.json"),
            ("progress.log", "sikaru-progress.log"),
        ):
            try:
                with tempfile.TemporaryDirectory() as directory:
                    target = Path(directory) / remote
                    await asyncio.wait_for(
                        environment.download_file(f"{self.remote}/{remote}", target), 30
                    )
                    content = target.read_text()
                self._write(local, content)
                if remote == "result.json":
                    data = json.loads(content)
                    if not isinstance(data, dict):
                        raise ValueError("Expected a CLI JSON object")
                    for key in (
                        "status",
                        "cleanup",
                        "cancel_acknowledged",
                        "session_id",
                        "attachment_id",
                        "run_id",
                        "state_dir",
                        "usage",
                    ):
                        if key in data:
                            evidence[key] = data[key]
                    self._usage(data.get("usage"), context)
            except Exception:  # noqa: BLE001 -- preserve partial evidence after any transport/parser failure
                evidence.setdefault("evidence_errors", []).append(remote)
        if evidence.get("evidence_errors"):
            evidence["cleanup"] = "unconfirmed"
        self._write("sikaru-adapter.json", evidence)

    @staticmethod
    def _usage(payload, context):
        # Only explicitly named inclusive totals are portable to runner fields.
        # Keep other service payloads verbatim instead of guessing their units.
        if not isinstance(payload, dict) or payload.get("available") is not True:
            return
        usage, cost = payload.get("usage"), payload.get("cost")
        if isinstance(usage, dict):
            for key in ("n_input_tokens", "n_cache_tokens", "n_output_tokens"):
                value = usage.get(key)
                if type(value) is int and value >= 0:
                    setattr(context, key, value)
        if isinstance(cost, dict):
            value = cost.get("cost_usd")
            if type(value) in (int, float) and math.isfinite(value) and value >= 0:
                context.cost_usd = value
