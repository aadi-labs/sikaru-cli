"""Offline checks for local command ownership and fail-closed receipt handling."""
import asyncio
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('compute', Path(__file__).parents[1] / 'client-extensions/cli/sikaru_compute/__init__.py')
compute = importlib.util.module_from_spec(spec)
spec.loader.exec_module(compute)


class LocalComputeTests(unittest.IsolatedAsyncioTestCase):
    async def asyncSetUp(self):
        self.directory = tempfile.TemporaryDirectory()
        self.root = Path(self.directory.name)
        self.executor = compute.Executor(self.root, self.root / 'output', timeout=1, output_limit=1024)

    async def asyncTearDown(self):
        await self.executor.close()
        self.directory.cleanup()

    async def test_workspace_output_paging_and_credentials(self):
        with patch.dict(os.environ, {'SIKARU_API_KEY': 'not-for-commands'}):
            handle = (await self.executor.execute('bash.start', {'command': 'printf "%s" "${SIKARU_API_KEY-unset}"; pwd'}))['id']
        await self.executor.execute('bash.wait', {'handle_id': handle})
        result = await self.executor.execute('bash.read', {'handle_id': handle, 'offset': 0, 'limit': 1024})
        self.assertIn('unset' + str(self.root.resolve()), result['output'])
        self.assertEqual(result['returncode'], 0)
        await self.executor.execute('workspace.write_text', {'path': 'file', 'text': 'hello'})
        self.assertEqual((self.root / 'file').read_text(), 'hello')

    async def test_output_limit_and_timeout(self):
        handle = (await self.executor.execute('bash.start', {'command': 'yes output'}))['id']
        await self.executor.execute('bash.wait', {'handle_id': handle})
        self.assertLessEqual(self.executor.handles[handle]['path'].stat().st_size, 1024)
        self.assertTrue(self.executor.status(handle)['truncated'])
        handle = (await self.executor.execute('bash.start', {'command': 'sleep 30'}))['id']
        await asyncio.sleep(1.1)
        self.assertTrue(self.executor.status(handle)['timed_out'])
        self.assertIsNotNone(self.executor.status(handle)['returncode'])

    async def test_large_pipe_output_cleanup_is_bounded(self):
        async with asyncio.timeout(5):
            handle = (await self.executor.execute('bash.start', {'command': 'head -c 16777216 /dev/zero'}))['id']
            await self.executor.execute('bash.wait', {'handle_id': handle})
            await self.executor.close()
        self.assertEqual(self.executor.handles[handle]['path'].stat().st_size, 1024)
        self.assertTrue(self.executor.status(handle)['truncated'])

    async def test_cancel_cleans_background_descendant(self):
        handle = (await self.executor.execute('bash.start', {'command': 'sleep 30 & echo $!; wait'}))['id']
        await asyncio.sleep(.05)
        result = await self.executor.execute('bash.cancel', {'handle_id': handle})
        self.assertEqual(result['status'], 'cancelled')
        self.assertTrue(self.executor.handles[handle]['task'].done())

    async def test_finished_group_is_never_signalled_again(self):
        handle = (await self.executor.execute('bash.start', {'command': 'true'}))['id']
        await self.executor.execute('bash.wait', {'handle_id': handle})
        with patch.object(os, 'killpg') as kill:
            await self.executor.execute('bash.cancel', {'handle_id': handle})
            await self.executor.close()
            kill.assert_not_called()

    async def test_repeated_cancellation_during_launch_retains_child(self):
        original = asyncio.create_subprocess_exec
        created = []
        async def delayed(*args, **kwargs):
            process = await original(*args, **kwargs)
            created.append(process)
            await asyncio.sleep(.1)
            return process
        with patch.object(asyncio, 'create_subprocess_exec', side_effect=delayed):
            task = asyncio.create_task(self.executor.execute('bash.start', {'command': 'sleep 30'}))
            await asyncio.sleep(.02)
            task.cancel()
            await asyncio.sleep(.02)
            task.cancel()
            with self.assertRaises(asyncio.CancelledError):
                await task
        self.assertEqual(len(created), 1)
        self.assertIsNotNone(created[0].returncode)

    async def test_validation(self):
        for method, args in [('bash.start', {'command': ''}), ('bash.read', {'handle_id': 'bad'}),
                             ('workspace.write_text', {'path': 'a', 'text': 3})]:
            with self.assertRaises(ValueError):
                await self.executor.execute(method, args)

    async def test_journal_cannot_reopen(self):
        journal = compute.Journal(self.root / 'journal')
        journal.append(state='executing')
        journal.close()
        with self.assertRaises(FileExistsError):
            compute.Journal(self.root / 'journal')

    async def test_receipt_retry_never_reexecutes(self):
        request = {'tool_call_id': 'tool', 'tool_provider_id': 'provider',
                   'capability_name': 'compute.execute',
                   'arguments': {'method': 'workspace.write_text', 'arguments': {'path': 'a', 'text': 'value'}}}
        class FakeClient:
            calls = 0
            receipts = []
            async def call(self, operation, body=None, **query):
                if operation == 'get':
                    return {'status': 'waiting_for_tool_results', 'executionEnvironment': 'local', 'computeProviderId': 'provider'}
                if operation == 'pending-actions':
                    return {'actions': [{**request, 'status': 'awaiting_product_result', 'approval': 'not_required'}]}
                if operation == 'events':
                    return {'events': [{'sequence': 1, 'eventType': 'run.tool_call.requested', 'payload': request}], 'nextAfter': 1} if query['after'] == 0 else {'events': [{'sequence': 2, 'eventType': 'run.completed', 'payload': {}}], 'nextAfter': 2}
                if operation == 'submit-tool-result':
                    self.receipts.append(body)
                    if len(self.receipts) == 1:
                        raise TimeoutError('uncertain acknowledgement')
        journal = compute.Journal(self.root / 'journal')
        client = FakeClient()
        with patch.object(self.executor, 'execute', wraps=self.executor.execute) as execute:
            await compute.serve(client, self.executor, journal, 'provider', poll_seconds=.01)
            self.assertEqual(execute.call_count, 1)
        self.assertEqual(client.receipts[0], client.receipts[1])
        journal.close()

    async def test_other_providers_and_later_approval(self):
        request = {'tool_call_id': 'tool', 'tool_provider_id': 'provider',
                   'capability_name': 'compute.execute',
                   'arguments': {'method': 'workspace.write_text', 'arguments': {'path': 'approved', 'text': 'done'}}}
        class FakeClient:
            polls = 0
            done = False
            async def call(self, operation, body=None, **query):
                if operation == 'get':
                    return {'status': 'completed' if self.done else 'waiting_for_tool_results',
                            'executionEnvironment': 'local', 'computeProviderId': 'provider'}
                if operation == 'events':
                    return {'events': [], 'nextAfter': query['after']}
                if operation == 'pending-actions':
                    self.polls += 1
                    return {'actions': [{'tool_provider_id': 'connector', 'capability_name': 'mail.send'},
                        {**request, 'status': 'awaiting_product_result',
                         'approval': 'pending' if self.polls == 1 else 'approved'}]}
                if operation == 'submit-tool-result':
                    self.done = True
                if operation == 'cancel':
                    raise AssertionError('Completed run must not be cancelled')
        journal = compute.Journal(self.root / 'journal')
        client = FakeClient()
        with patch.object(self.executor, 'execute', wraps=self.executor.execute) as execute:
            result = await compute.serve(client, self.executor, journal, 'provider', poll_seconds=.01)
            self.assertEqual(execute.call_count, 1)
        self.assertEqual(result['status'], 'completed')
        self.assertGreaterEqual(client.polls, 2)
        journal.close()

    async def test_terminal_run_does_not_execute(self):
        class FakeClient:
            async def call(self, operation, **query):
                return {'status': 'cancelled'}
        journal = compute.Journal(self.root / 'journal')
        with patch.object(self.executor, 'execute', wraps=self.executor.execute) as execute:
            await compute.serve(FakeClient(), self.executor, journal, 'provider')
            execute.assert_not_called()
        journal.close()



@unittest.skipUnless(os.environ.get('SIKARU_TEST_CLI'), 'Set SIKARU_TEST_CLI to the built generated binary')
class GeneratedCliRoundTrip(unittest.IsolatedAsyncioTestCase):
    async def test_real_cli_transport_and_local_cleanup(self):
        import argparse
        import http.server
        import json
        import threading
        receipts = []
        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, *args):
                pass
            def respond(self, value):
                data = json.dumps(value).encode()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(data)))
                self.end_headers()
                self.wfile.write(data)
            def do_GET(self):
                if '/events' in self.path:
                    self.respond({'events': [], 'nextAfter': 0})
                elif '/actions' in self.path:
                    index = len(receipts)
                    operations = [('workspace.write_text', {'path': 'transport-proof', 'text': 'written'}),
                                  ('bash.start', {'command': 'sleep 30'})]
                    actions = [] if index >= 2 else [{
                        'tool_call_id': str(index), 'tool_provider_id': 'provider',
                        'capability_name': 'compute.execute', 'status': 'awaiting_product_result',
                        'approval': 'not_required',
                        'arguments': {'method': operations[index][0], 'arguments': operations[index][1]}}]
                    self.respond({'actions': actions})
                else:
                    self.respond({'status': 'completed' if len(receipts) >= 2 else 'waiting_for_tool_results',
                                  'executionEnvironment': 'local', 'computeProviderId': 'provider'})
            def do_POST(self):
                if self.path.endswith('/tool-results'):
                    receipts.append(json.loads(self.rfile.read(int(self.headers['Content-Length']))))
                    self.respond({'accepted': True})
                else:
                    self.send_error(400, 'Unexpected mutation')
        server = http.server.ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            with tempfile.TemporaryDirectory() as directory, patch.dict(os.environ, {'SIKARU_API_KEY': 'offline-only'}):
                root = Path(directory)
                options = argparse.Namespace(cli=os.environ['SIKARU_TEST_CLI'], project_id='project', run_id='run',
                    base_url=f'http://127.0.0.1:{server.server_port}')
                executor = compute.Executor(root, root / 'output')
                journal = compute.Journal(root / 'journal')
                try:
                    result = await compute.serve(compute.Client(options), executor, journal, 'provider', poll_seconds=.05, timeout=20)
                    self.assertEqual(result['status'], 'completed')
                    self.assertEqual((root / 'transport-proof').read_text(), 'written')
                    self.assertEqual(len(receipts), 2)
                    self.assertTrue(all(job['process'].returncode is not None for job in executor.handles.values()))
                    # Generated CLI interpolates @ strings unless the adapter preserves them.
                    sensitive = root / 'must-not-be-read'
                    sensitive.write_text('not-the-requested-output')
                    for output in ('@' + str(sensitive), '\\@literal', '\x00\x01\n' * 65536):
                        receipt = {'tool_call_id': 'literal', 'tool_provider_id': 'provider',
                            'capability_name': 'compute.execute', 'idempotency_key': 'literal',
                            'status': 'completed', 'payload': {'output': output,
                            'nested': ['@missing-file', '\\@also-literal']}}
                        await compute.Client(options).call('submit-tool-result', body=receipt)
                        self.assertEqual(receipts[-1], receipt)

                finally:
                    journal.close()
        finally:
            await asyncio.to_thread(server.shutdown)
            server.server_close()
            thread.join()

if __name__ == '__main__':
    unittest.main()
