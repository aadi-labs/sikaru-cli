"""Bounded local wire oracle for all generated client lifecycle suites.

This tests serialization, credentials, polling models and transport retry policy;
it is deliberately not a substitute for gateway authorization/lease acceptance.
"""
import json
import os
import subprocess
import sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from threading import Thread

PROVENANCE = {'kind': 'existing_directory', 'identity': 'workspace'}
ATTACHMENT = dict(id='attachment', session_id='session', project_id='project',
    environment_id='environment', provider_id='provider', workspace_generation='generation',
    workspace_provenance=PROVENANCE, journal_id='journal', status='pending',
    owner_id=None, owner_epoch=0, lease_until=0.0, lease_ttl_seconds=60,
    startup_deadline=2000000180.0, capabilities=['compute.execute'],
    protocol_version='sikaru-compute-v1', cleanup_at=None, cleanup_status='unconfirmed',
    uncertain_operations=[], processes=[])
RECEIPT = dict(run_id='run', tool_call_id='tool', tool_provider_id='provider',
    capability_name='compute.execute', idempotency_key='receipt', request_digest='a'*64,
    status='completed', payload={'output': 'done'})
BASE = '/v1/projects/project/'
PATH = BASE + 'compute-attachments/attachment'


def steps():
    ready = dict(executor_instance_id='instance', journal_id='journal',
        workspace_provenance=PROVENANCE, protocol_version='sikaru-compute-v1',
        capabilities=['compute.execute'])
    active = dict(ATTACHMENT, owner_id='worker', owner_epoch=1, lease_until=2000000000.0)
    recovery = dict(active, status='recovery_required', processes=[
        {'handle_id': 'handle', 'status': 'lost', 'evidence': 'ownership unproven'}])
    work = dict(attachment=dict(active, status='ready'), execution_phase='running',
        live_handles=[], issued_operations=[], operations=[dict(run_id='run', tool_call_id='tool',
        tool_provider_id='provider', capability_name='compute.execute', method='workspace.write_text',
        arguments={'path':'out.txt','text':'done'}, request_digest='a'*64, owner_epoch=1,
        workspace_generation='generation')], execution=dict(run_id='run', status='running',
        approval_required=False, terminal=False), poll_after_seconds=1)
    return [
        ('POST', BASE+'execution-sessions/session/compute-attachments', 'controller',
         dict(environment_id='environment', workspace_provenance=PROVENANCE, idempotency_key='create'), 201, ATTACHMENT),
        ('GET', PATH, 'controller', None, 200, ATTACHMENT),
        ('GET', BASE+'compute-environments/environment/queue', 'sk_compute_worker', None, 200,
         dict(attachments=[ATTACHMENT], poll_after_seconds=1)),
        ('POST', PATH+'/ready', 'sk_compute_executor', ready, 200, dict(active,status='ready')),
        ('GET', PATH+'/work', 'sk_compute_executor', None, 200, work),
        ('POST', PATH+'/receipts', 'sk_compute_executor', RECEIPT, 200,
         dict(tool_call_id='tool', status='accepted', created=True)),
        ('POST', PATH+'/receipts', 'sk_compute_executor', RECEIPT, 200,
         dict(tool_call_id='tool', status='accepted', created=False)),
        ('POST', PATH+'/reconcile', 'sk_compute_executor',
         dict(executor_instance_id='instance', journal_id='journal', workspace_provenance=PROVENANCE,
              processes=recovery['processes']), 200, dict(attachment=recovery, receipts=[])),
        ('GET', PATH+'/status', 'sk_compute_executor', None, 200, recovery),
        ('GET', PATH+'/work', 'sk_compute_executor', None, 200,
         dict(work, execution_phase='terminal', operations=[], execution=dict(run_id='run',status='completed',approval_required=False,terminal=True))),
        ('GET', PATH+'/work', 'wrong-scope', None, 403, {'detail':'Outside attachment authority'}),
        ('POST', PATH+'/receipts', 'sk_compute_executor', RECEIPT, 503, {'detail':'Response lost after commit'}),
    ]


def check_request(handler, expected):
    method, path, token, body, _, _ = expected
    assert handler.command == method, (handler.command, method)
    assert handler.path.split('?')[0] == path, handler.path
    assert handler.headers.get('Authorization') == 'Bearer '+token
    actual = json.loads(handler.rfile.read(int(handler.headers.get('Content-Length', 0))) or b'{}')
    for key, value in (body or {}).items():
        assert actual.get(key) == value, (key, actual)


def run(command):
    pending = steps()
    failures = []
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, *args):
            pass
        def do_GET(self):
            self.respond()
        def do_POST(self):
            self.respond()
        def respond(self):
            try:
                expected = pending.pop(0)
                check_request(self, expected)
                code, result = expected[-2:]
            except Exception as exc:
                failures.append(str(exc))
                code, result = 500, {'detail': str(exc)}
            payload = json.dumps(result).encode()
            self.send_response(code)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
    server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
    thread = Thread(target=server.serve_forever, daemon=True)
    thread.start()
    try:
        result = subprocess.run(command, env=dict(os.environ, CONTRACT_URL=f'http://127.0.0.1:{server.server_port}'), timeout=600)
        assert result.returncode == 0, result.returncode
        assert not failures, failures
        assert not pending, f'{len(pending)} missing wire operations'
    finally:
        server.shutdown()
        server.server_close()
        thread.join()
    print('PASS: 12 scoped lifecycle requests, typed recovery/terminal polling, immutable replay, no automatic mutation retry')


if __name__ == '__main__':
    run(sys.argv[1:])
