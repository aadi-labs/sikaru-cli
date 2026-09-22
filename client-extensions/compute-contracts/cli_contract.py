"""Exercise the primary generated CLI operations, including scoped bearer auth."""
import json
import os
import subprocess


def call(token, group, operation, *, fail=False, **parameters):
    args = [os.environ['COMPUTE_CONTRACT_BINARY'], group.replace('_','-'), operation, '--project-id', 'project',
            '--base-url', os.environ['CONTRACT_URL'], '--format', 'json']
    for name, value in parameters.items():
        args.extend(['--'+name.replace('_','-'), json.dumps(value) if isinstance(value, (dict,list)) else str(value)])
    result = subprocess.run(args, env=dict(os.environ, SIKARU_API_KEY=token), capture_output=True, text=True, timeout=30)
    if fail:
        assert result.returncode != 0, result.stdout
        return json.loads(result.stdout)
    assert result.returncode == 0, (result.stdout, result.stderr)
    return json.loads(result.stdout)


def create_and_ready():
    provenance = {'kind':'existing_directory','identity':'workspace'}
    a = call('controller','compute_attachments','create',session_id='session',environment_id='environment',idempotency_key='create',workspace_provenance=provenance)
    assert a['owner_id'] is None
    assert a['workspace_generation']=='generation' and a['status']=='pending'
    assert call('controller','compute_attachments','get',attachment_id='attachment')['journal_id']=='journal'
    queue = call('sk_compute_worker','compute_workers','poll',environment_id='environment',wait_seconds=0)
    assert queue['attachments'][0]['id']=='attachment'
    ready = call('sk_compute_executor','compute_attachments','ready', attachment_id='attachment', executor_instance_id='instance',journal_id='journal',workspace_provenance=provenance,protocol_version='sikaru-compute-v1',capabilities=['compute.execute'])
    assert ready['status']=='ready'


def work_and_receipts():
    work = call('sk_compute_executor','compute_operations','poll',attachment_id='attachment',wait_seconds=0)
    assert work['execution_phase']=='running' and work['operations'][0]['request_digest']=='a'*64
    receipt = dict(attachment_id='attachment',run_id='run',tool_call_id='tool',tool_provider_id='provider',capability_name='compute.execute',idempotency_key='receipt',request_digest='a'*64,status='completed',payload={'output':'done'})
    assert call('sk_compute_executor','compute_operations','submit_receipt',**receipt)['created']
    assert not call('sk_compute_executor','compute_operations','submit_receipt',**receipt)['created']
    return receipt


def recovery_and_errors(receipt):
    recovered = call('sk_compute_executor','compute_attachments','reconcile',attachment_id='attachment',executor_instance_id='instance',journal_id='journal',workspace_provenance={'kind':'existing_directory','identity':'workspace'},processes=[{'handle_id':'handle','status':'lost','evidence':'ownership unproven'}])
    assert recovered['attachment']['status']=='recovery_required'
    assert call('sk_compute_executor','compute_attachments','status',attachment_id='attachment')['processes'][0]['status']=='lost'
    terminal = call('sk_compute_executor','compute_operations','poll',attachment_id='attachment')
    assert terminal['execution_phase']=='terminal' and terminal['execution']['terminal']
    denied = call('wrong-scope','compute_operations','poll',attachment_id='attachment',fail=True)
    assert denied['error']['code'] == 403
    failed = call('sk_compute_executor','compute_operations','submit_receipt',**receipt,fail=True)
    assert failed['error']['code'] == 503


if __name__ == '__main__':
    create_and_ready()
    recovery_and_errors(work_and_receipts())
