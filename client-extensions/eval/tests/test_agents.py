import asyncio
import importlib
import json
import os
import shutil
import sys
from pathlib import Path
from types import SimpleNamespace

import pytest


class Environment:
    """Real subprocess and file transport, isolated to a disposable workspace."""

    def __init__(self, workspace):
        self.workspace = workspace
        self.calls = []

    async def exec(self, command, cwd=None, env=None, timeout_sec=None, user=None):
        self.calls.append(command)
        process = await asyncio.create_subprocess_exec(
            "sh",
            "-c",
            command,
            cwd=cwd or self.workspace,
            env={**os.environ, **(env or {})},
            stdout=asyncio.subprocess.PIPE,
            stderr=asyncio.subprocess.PIPE,
        )
        stdout, stderr = await process.communicate()
        return SimpleNamespace(
            return_code=process.returncode,
            stdout=stdout.decode(),
            stderr=stderr.decode(),
        )

    async def upload_file(self, source, target):
        shutil.copy(source, target)

    async def download_file(self, source, target):
        shutil.copy(source, target)


@pytest.fixture(params=["harbor", "pier"])
def trial(request, tmp_path, monkeypatch):
    monkeypatch.setenv("SIKARU_API_KEY", "test-secret")
    binary = tmp_path / "fake sikaru"
    binary.write_text("""#!/usr/bin/env python3
import json, pathlib, sys, os
if "--version" in sys.argv:
    print("sikaru 0.1.0"); sys.exit()
if "--help" in sys.argv:
    print("exec --prompt-file --state-dir"); sys.exit()
args = sys.argv
def arg(name): return args[args.index(name)+1]
prompt = pathlib.Path(arg("--prompt-file")).read_text()
pathlib.Path(arg("--workspace"), "observed.txt").write_text(prompt)
print(json.dumps({"status": "completed", "cleanup": "confirmed", "session_id": arg("--state-dir"), "usage": {"available": False}}))
""")
    binary.chmod(0o755)
    cls = importlib.import_module(f"sikaru_eval.{request.param}").SikaruAgent
    context = importlib.import_module(
        f"{request.param}.models.agent.context"
    ).AgentContext()
    agent = cls(
        logs_dir=tmp_path / "logs",
        project="project",
        agent="agent",
        binary_path=str(binary),
    )
    yield agent, Environment(tmp_path), context
    shutil.rmtree(agent.remote, ignore_errors=True)


async def test_real_cli_launch_and_filesystem(trial):
    agent, env, context = trial
    await agent.setup(env)
    prompt = "quotes ' \" $(touch INJECTED)\nUnicode: λ"
    await agent.run(prompt, env, context)
    assert (env.workspace / "observed.txt").read_text() == prompt
    assert not (env.workspace / "INJECTED").exists()
    assert context.metadata["sikaru"]["cleanup"] == "confirmed"
    assert context.n_input_tokens is None and context.cost_usd is None
    assert (
        json.loads((agent.logs_dir / "sikaru-result.json").read_text())["status"]
        == "completed"
    )
    assert all("test-secret" not in call for call in env.calls)
    assert all("test-secret" not in p.read_text() for p in agent.logs_dir.iterdir())
    with pytest.raises(RuntimeError, match="replayed"):
        await agent.run(prompt, env, context)


@pytest.mark.parametrize(
    "status,cleanup,code",
    [
        ("failed", "confirmed", 1),
        ("approval_required", "confirmed", 2),
        ("cancelled", "confirmed", 3),
        ("recovery_required", "unconfirmed", 4),
        ("completed", "unconfirmed", 0),
    ],
)
async def test_non_success_retains_evidence(trial, status, cleanup, code):
    agent, env, context = trial
    binary = Path(agent.binary_path)
    binary.write_text(
        binary.read_text()
        .replace('"completed"', repr(status))
        .replace('"confirmed"', repr(cleanup))
        + f"\nsys.exit({code})\n"
    )
    await agent.setup(env)
    with pytest.raises(RuntimeError, match="did not complete cleanly"):
        await agent.run("task", env, context)
    assert context.metadata["sikaru"]["status"] == status
    assert (agent.logs_dir / "sikaru-result.json").exists()


async def test_missing_evidence_is_not_success(trial):
    agent, env, context = trial
    await agent.setup(env)

    async def missing(*args):
        raise OSError("lost transport")

    env.download_file = missing
    with pytest.raises(RuntimeError, match="did not complete cleanly"):
        await agent.run("task", env, context)
    assert context.metadata["sikaru"]["cleanup"] == "unconfirmed"


async def test_cancellation_attempts_stop_and_preserves_unknown_cleanup(trial):
    agent, env, context = trial
    await agent.setup(env)
    original = env.exec
    started = asyncio.Event()

    async def blocked(command, **kwargs):
        if "echo $$" in command:
            started.set()
            await asyncio.Event().wait()
        return await original(command, **kwargs)

    env.exec = blocked
    agent.cleanup_timeout = 0.01
    task = asyncio.create_task(agent.run("task", env, context))
    await started.wait()
    task.cancel()
    with pytest.raises(asyncio.CancelledError):
        await task
    assert any("kill -TERM" in call for call in env.calls)
    assert context.metadata["sikaru"]["cleanup"] == "unconfirmed"


def test_managed_policy_and_pier_network(trial):
    agent, _, _ = trial
    if hasattr(agent, "network_allowlist"):
        assert agent.network_allowlist().domains == ["api.sikaru.ai"]


def test_runner_factory_and_explicit_usage(trial):
    agent, _, context = trial
    runner = type(agent).__module__.split(".")[-1]
    factory = importlib.import_module(f"{runner}.agents.factory").AgentFactory
    created = factory.create_agent_from_import_path(
        f"sikaru_eval.{runner}:SikaruAgent",
        logs_dir=agent.logs_dir,
        project="project",
        agent="agent",
        extra_env={"SIKARU_API_KEY": "key"},
    )
    assert created.name() == "sikaru"
    created._usage(
        {
            "available": True,
            "usage": {
                "n_input_tokens": 50,
                "n_cache_tokens": 20,
                "n_output_tokens": 10,
            },
            "cost": {"cost_usd": 0.25},
        },
        context,
    )
    assert (
        context.n_input_tokens,
        context.n_cache_tokens,
        context.n_output_tokens,
        context.cost_usd,
    ) == (50, 20, 10, 0.25)


def test_runner_config_and_unknown_usage(trial):
    agent, _, context = trial
    runner = type(agent).__module__.split(".")[-1]
    factory = importlib.import_module(f"{runner}.agents.factory").AgentFactory
    config_type = importlib.import_module(f"{runner}.models.trial.config").AgentConfig
    config = config_type(
        import_path=f"sikaru_eval.{runner}:SikaruAgent",
        kwargs={"project": "project", "agent": "agent"},
    )
    created = factory.create_agent_from_config(config, logs_dir=agent.logs_dir)
    assert created.to_agent_info().model_info.name == "sikaru-managed"
    created._usage(
        {
            "available": True,
            "usage": {
                "input_tokens": 50,
                "n_cache_tokens": -1,
                "n_output_tokens": True,
            },
            "cost": {"cost_usd": float("nan")},
        },
        context,
    )
    assert context.n_input_tokens is None and context.n_cache_tokens is None
    assert context.n_output_tokens is None and context.cost_usd is None


@pytest.mark.skipif(sys.platform != "linux", reason="Linux /proc ownership check")
async def test_term_reaches_owned_cli_and_collects_final_result(trial):
    agent, env, context = trial
    binary = Path(agent.binary_path)
    script = binary.read_text()
    script = script.replace(
        "prompt = pathlib.Path",
        """import signal, time
def stop(*_):
    print(json.dumps({"status":"cancelled", "cleanup":"confirmed", "cancel_acknowledged":True}), flush=True)
    sys.exit(3)
signal.signal(signal.SIGTERM, stop)
pathlib.Path(arg("--workspace"), "ready").touch()
while True: time.sleep(0.01)
prompt = pathlib.Path""",
    )
    binary.write_text(script)
    await agent.setup(env)
    task = asyncio.create_task(agent.run("task", env, context))
    for _ in range(200):
        if (env.workspace / "ready").exists():
            break
        await asyncio.sleep(0.01)
    assert (env.workspace / "ready").exists()
    task.cancel()
    with pytest.raises(asyncio.CancelledError):
        await task
    assert context.metadata["sikaru"]["cancel_acknowledged"] is True
    assert context.metadata["sikaru"]["cleanup"] == "confirmed"


async def test_selected_model_and_headless_mode_reach_cli(trial):
    agent, env, context = trial
    selected = type(agent)(logs_dir=agent.logs_dir, project="project", agent="agent",
        model_name="kimi-k3", binary_path=agent.binary_path)
    selected.remote = agent.remote
    agent = selected
    assert agent.to_agent_info().model_info.name == "kimi-k3"
    binary = Path(agent.binary_path)
    source = binary.read_text().replace('args = sys.argv', 'args = sys.argv\nassert "--print" in args\nassert args[args.index("--model") + 1] == "kimi-k3"')
    binary.write_text(source)
    await agent.setup(env)
    await agent.run("change workspace", env, context)
    assert (env.workspace / "observed.txt").read_text() == "change workspace"
    assert context.metadata["sikaru"]["model_policy"] == "kimi-k3"


def test_partial_measurements_are_preserved_without_claiming_complete_totals(trial):
    agent, _, context = trial
    agent._usage({"available": True, "usage": {"complete": False, "n_input_tokens": 100},
                  "cost": {"cost_usd": None, "observed_cost_usd": 0.5}}, context)
    assert context.n_input_tokens is None
    assert context.cost_usd is None
