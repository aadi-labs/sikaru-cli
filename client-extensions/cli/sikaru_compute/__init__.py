"""Customer-owned compute. Hosted API transport is the generated Sikaru CLI."""
from __future__ import annotations

import argparse
import asyncio
import contextlib
import json
import math
import os
from pathlib import Path
import signal
import tempfile
import uuid

TERMINAL = {'completed', 'failed', 'cancelled', 'recovery_required'}


def positive(value, name, zero=False):
    if type(value) not in (int, float) or not math.isfinite(value) or value < 0 or (not zero and value == 0):
        raise ValueError(f'{name} must be finite and positive')
    return value


class Client:
    def __init__(self, options):
        self.options = options

    async def call(self, operation, body=None, **query):
        o = self.options
        args = [o.cli, 'runs', operation.replace('-', '_'), '--project-id', o.project_id,
                '--run-id', o.run_id, '--format', 'json']
        if o.base_url:
            args += ['--base-url', o.base_url]
        for key, value in query.items():
            args += ['--' + key.replace('_', '-'), str(value)]
        # The generated CLI resolves @ references inside JSON once. Materialize
        # only special-prefix strings as private text files to preserve their exact
        # contents, including literal backslash-at prefixes. Pass the whole body
        # via stdin, avoiding OS per-argument limits for escaped command output.
        with tempfile.TemporaryDirectory(prefix='sikaru-receipt-') as directory:
            def literal(value):
                if isinstance(value, str) and value.startswith(('@', '\\@')):
                    path = Path(directory) / uuid.uuid4().hex
                    with path.open('x', encoding='utf-8') as stream:
                        os.chmod(path, 0o600)
                        stream.write(value)
                    return '@file://' + str(path)
                if isinstance(value, dict):
                    return {key: literal(item) for key, item in value.items()}
                if isinstance(value, (list, tuple)):
                    return [literal(item) for item in value]
                return value
            data = None
            if body is not None:
                args += ['--json', '-']
                data = json.dumps(literal(body)).encode('utf-8')
            process = await asyncio.create_subprocess_exec(*args, stdin=asyncio.subprocess.PIPE,
                stdout=asyncio.subprocess.PIPE, stderr=asyncio.subprocess.PIPE)
            try:
                async with asyncio.timeout(30):
                    stdout, stderr = await process.communicate(data)
            finally:
                if process.returncode is None:
                    process.kill()
                    await process.communicate()
            if process.returncode:
                raise RuntimeError(f'Sikaru {operation} failed: {(stderr or stdout).decode(errors="replace")[:512]}')
            return json.loads(stdout) if stdout.strip() else None


class Journal:
    """An existing journal never authorizes command replay, including after a crash."""
    def __init__(self, path):
        path.parent.mkdir(parents=True, exist_ok=True)
        self.stream = path.open('x', encoding='utf-8')
        os.chmod(path, 0o600)

    def append(self, **record):
        self.stream.write(json.dumps(record) + '\n')
        self.stream.flush()
        os.fsync(self.stream.fileno())

    def close(self):
        self.stream.close()


class Executor:
    def __init__(self, workspace, output_dir, timeout=120, output_limit=65536):
        self.workspace = Path(workspace).resolve(strict=True)
        if not self.workspace.is_dir():
            raise ValueError('Workspace must be a directory')
        self.output_dir = Path(output_dir)
        self.output_dir.mkdir(parents=True, exist_ok=True)
        self.timeout = positive(timeout, 'command timeout')
        if type(output_limit) is not int or output_limit <= 0:
            raise ValueError('Output limit must be a positive integer')
        self.output_limit = output_limit
        self.handles = {}

    @staticmethod
    def kill(job):
        if job.get('group_closed'):
            return
        # The session/group belongs to this child. Kill descendants even when the shell exited.
        with contextlib.suppress(ProcessLookupError):
            os.killpg(job['process'].pid, signal.SIGKILL)
        job['group_closed'] = True

    async def collect(self, job):
        process = job['process']
        try:
            async with asyncio.timeout(self.timeout):
                with job['path'].open('wb') as output:
                    remaining = self.output_limit
                    while data := await process.stdout.read(65536):
                        output.write(data[:remaining])
                        output.flush()
                        remaining -= min(remaining, len(data))
                        if remaining == 0:
                            job['truncated'] = True
                            self.kill(job)
                            break
                await process.communicate()
        except TimeoutError:
            job['timed_out'] = True
        finally:
            self.kill(job)
            await process.communicate()

    def status(self, handle):
        job = self.handles[handle]
        if job['task'].done():
            job['task'].result()
        return {'id': handle, 'status': 'cancelled' if job['cancelled'] else
                ('exited' if job['task'].done() else 'running'),
                'returncode': job['process'].returncode, 'truncated': job['truncated'],
                'timed_out': job['timed_out']}

    async def execute(self, method, arguments):
        if not isinstance(arguments, dict):
            raise ValueError('Compute arguments must be an object')
        if method == 'workspace.write_text':
            if set(arguments) != {'path', 'text'} or any(not isinstance(v, str) for v in arguments.values()):
                raise ValueError('Workspace write requires path and text strings')
            path = Path(arguments['path'])
            if not path.is_absolute():
                path = self.workspace / path
            path.write_text(arguments['text'], encoding='utf-8')
            return {'written': True}
        if method == 'bash.start':
            if set(arguments) - {'command', 'cwd', 'env'}:
                raise ValueError('Unknown Bash start arguments')
            if not isinstance(arguments.get('command'), str) or not arguments['command'].strip():
                raise ValueError('Bash requires a command')
            cwd = arguments.get('cwd', str(self.workspace))
            if not isinstance(cwd, str) or not cwd:
                raise ValueError('Bash cwd must be a path')
            if not Path(cwd).is_absolute():
                cwd = str(self.workspace / cwd)
            env = arguments.get('env', {})
            if not isinstance(env, dict) or any(not isinstance(k, str) or not isinstance(v, str) for k, v in env.items()):
                raise ValueError('Bash env must contain strings')
            handle = uuid.uuid4().hex
            path = self.output_dir / handle
            path.touch(mode=0o600)
            # Shield launch so cancellation cannot lose ownership of a newly created child.
            launch = asyncio.create_task(asyncio.create_subprocess_exec(
                '/bin/bash', '-c', arguments['command'], cwd=cwd, env={**{k: v for k, v in os.environ.items() if not k.startswith('SIKARU_')}, **env},
                stdin=asyncio.subprocess.DEVNULL, stdout=asyncio.subprocess.PIPE,
                stderr=asyncio.subprocess.STDOUT, start_new_session=True))
            try:
                process = await asyncio.shield(launch)
            except asyncio.CancelledError:
                async def abort_launch():
                    process = await launch
                    self.kill({'process': process})
                    await process.communicate()
                cleanup = asyncio.create_task(abort_launch())
                while not cleanup.done():
                    try:
                        await asyncio.shield(cleanup)
                    except asyncio.CancelledError:
                        continue
                cleanup.result()
                raise
            job = {'process': process, 'path': path, 'cancelled': False,
                   'truncated': False, 'timed_out': False}
            self.handles[handle] = job
            job['task'] = asyncio.create_task(self.collect(job))
            return self.status(handle)
        allowed = {'bash.read': {'handle_id', 'offset', 'limit'},
                   'bash.wait': {'handle_id', 'timeout'}, 'bash.cancel': {'handle_id'}}
        if method not in allowed or set(arguments) - allowed[method]:
            raise ValueError('Unknown compute method or arguments')
        handle = arguments.get('handle_id')
        if not isinstance(handle, str) or handle not in self.handles:
            raise ValueError('Unknown process handle')
        job = self.handles[handle]
        if method == 'bash.cancel':
            job['cancelled'] = True
            self.kill(job)
            await asyncio.shield(job['task'])
        elif method == 'bash.wait':
            timeout = min(positive(arguments.get('timeout', self.timeout), 'wait timeout', True), self.timeout)
            await asyncio.wait([job['task']], timeout=timeout)
        else:
            limit = arguments.get('limit', min(16384, self.output_limit))
            offset = arguments.get('offset')
            if type(limit) is not int or not 0 < limit <= self.output_limit:
                raise ValueError('Invalid output page limit')
            if offset is not None and (type(offset) is not int or offset < 0):
                raise ValueError('Invalid output page offset')
            size = job['path'].stat().st_size
            start = max(0, size - limit) if offset is None else offset
            with job['path'].open('rb') as stream:
                stream.seek(start)
                data = stream.read(limit)
            return {**self.status(handle), 'output': data.decode('utf-8', errors='replace'),
                    'offset': start, 'next_offset': start + len(data), 'total_bytes': size,
                    'omitted_before': min(start, size), 'omitted_after': max(0, size-start-len(data)),
                    'artifact_path': str(job['path'])}
        return self.status(handle)

    async def close(self):
        for job in self.handles.values():
            if not job['task'].done():
                self.kill(job)
        results = await asyncio.gather(*(job['task'] for job in self.handles.values()), return_exceptions=True)
        for result in results:
            if isinstance(result, BaseException):
                raise RuntimeError('Local process cleanup could not be confirmed') from result


async def serve(client, executor, journal, provider_id, poll_seconds=.25, timeout=3600):
    seen = {}
    cursor = 0
    owner = asyncio.current_task()
    terminal = None
    async def monitor():
        nonlocal terminal
        while True:
            await asyncio.sleep(poll_seconds)
            try:
                status = await client.call('get')
                if status['status'] in TERMINAL:
                    terminal = status
                    owner.cancel()
                    return
            except Exception:
                # Stop local execution if hosted state cannot be confirmed.
                owner.cancel()
                return
    watcher = None
    try:
        async with asyncio.timeout(timeout):
            current = await client.call('get')
            if current['status'] in TERMINAL:
                return current
            if current.get('executionEnvironment') != 'local' or current.get('computeProviderId') != provider_id:
                raise RuntimeError('Run is not bound to this local compute provider')
            watcher = asyncio.create_task(monitor())
            while True:
                page = await client.call('events', after=cursor)
                terminal_event = False
                for event in page['events']:
                    if type(event['sequence']) is not int or event['sequence'] <= cursor:
                        raise RuntimeError('Hosted cursor did not advance')
                    cursor = event['sequence']
                    if event['eventType'] in {'run.' + state for state in TERMINAL}:
                        terminal_event = True
                if page['nextAfter'] != cursor:
                    raise RuntimeError('Hosted page cursor does not match events')
                if terminal_event:
                    return await client.call('get')
                # Poll current actions, so approvals arriving after their request event are admitted.
                actions = (await client.call('pending-actions'))['actions']
                for action in actions:
                    if action.get('tool_provider_id') != provider_id or action.get('capability_name') != 'compute.execute':
                        continue
                    if action.get('status') != 'awaiting_product_result':
                        raise RuntimeError('Compute request needs reconciliation')
                    if action.get('approval') not in {'approved', 'not_required'}:
                        continue
                    current = await client.call('get')
                    if current['status'] in TERMINAL:
                        return current
                    if current.get('executionEnvironment') != 'local' or current.get('computeProviderId') != provider_id:
                        raise RuntimeError('Run compute binding changed')
                    identity = action['tool_call_id']
                    payload = {key: action[key] for key in ('tool_call_id', 'tool_provider_id', 'capability_name', 'arguments')}
                    digest = json.dumps(payload, sort_keys=True)
                    if identity in seen:
                        previous, receipt = seen[identity]
                        if previous != digest:
                            raise RuntimeError('Repeated tool identity changed arguments')
                        continue  # This exact immutable receipt was already acknowledged.
                    envelope = payload['arguments']
                    if not isinstance(envelope, dict) or set(envelope) != {'method', 'arguments'}:
                        raise ValueError('Invalid compute operation envelope')
                    journal.append(state='executing', tool_call_id=identity, request=payload)
                    output = await executor.execute(envelope['method'], envelope['arguments'])
                    receipt = {'tool_call_id': identity, 'tool_provider_id': provider_id,
                               'capability_name': 'compute.execute', 'idempotency_key': identity,
                               'status': 'completed', 'payload': output}
                    journal.append(state='executed', receipt=receipt)
                    seen[identity] = digest, receipt
                    for attempt in range(3):
                        try:
                            await client.call('submit-tool-result', body=receipt)
                            break
                        except Exception:
                            if attempt == 2:
                                raise
                            await asyncio.sleep(poll_seconds)
                    journal.append(state='acknowledged', tool_call_id=identity)
                await asyncio.sleep(poll_seconds)
    except BaseException:
        if terminal is not None:
            return terminal
        journal.append(state='interrupted', execution_may_be_incomplete=True)
        with contextlib.suppress(Exception, asyncio.CancelledError):
            await client.call('cancel')
        raise
    finally:
        if watcher:
            watcher.cancel()
        async def cleanup():
            if watcher:
                with contextlib.suppress(asyncio.CancelledError):
                    await watcher
            await executor.close()
            journal.append(state='cleanup_confirmed')
        task = asyncio.create_task(cleanup())
        while not task.done():
            try:
                await asyncio.shield(task)
            except asyncio.CancelledError:
                continue
        task.result()


async def run(options):
    for field in ('poll_seconds', 'timeout', 'command_timeout'):
        positive(getattr(options, field), field)
    state = Path(options.state_dir).resolve()
    state.mkdir(parents=True, exist_ok=True)
    journal = Journal(state / 'execution.jsonl')
    try:
        journal.append(state='connected', project_id=options.project_id, run_id=options.run_id,
                       provider_id=options.provider_id)
        executor = Executor(options.workspace, state / 'output', options.command_timeout, options.output_limit)
        result = await serve(Client(options), executor, journal, options.provider_id,
                             options.poll_seconds, options.timeout)
        print(json.dumps(result))
    finally:
        journal.close()


def main():
    parser = argparse.ArgumentParser(description='Serve a hosted Sikaru run using this machine as local compute.')
    for flag in ('project-id', 'run-id', 'provider-id', 'workspace', 'state-dir'):
        parser.add_argument('--' + flag, required=True)
    parser.add_argument('--cli', default='sikaru', help='Sikaru CLI executable')
    parser.add_argument('--base-url')
    parser.add_argument('--poll-seconds', type=float, default=.25)
    parser.add_argument('--timeout', type=float, default=3600)
    parser.add_argument('--command-timeout', type=float, default=120)
    parser.add_argument('--output-limit', type=int, default=65536)
    options = parser.parse_args()
    if os.name != 'posix':
        parser.error('Local compute requires macOS or Linux')
    try:
        async def entry():
            task = asyncio.current_task()
            loop = asyncio.get_running_loop()
            for sig in (signal.SIGINT, signal.SIGTERM):
                loop.add_signal_handler(sig, task.cancel)
            try:
                await run(options)
            finally:
                for sig in (signal.SIGINT, signal.SIGTERM):
                    loop.remove_signal_handler(sig)
        asyncio.run(entry())
    except (Exception, KeyboardInterrupt, asyncio.CancelledError) as error:
        parser.exit(1, f'Local compute stopped: {error}\n')
