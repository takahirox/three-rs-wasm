#!/usr/bin/env python3
"""Record actual browser/Rust attempts without promoting snapshots to ports."""
import collections
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
CACHE = ROOT / '.cache/gallery-runtime'
OUTPUT = ROOT / 'docs/gallery-runtime-results.json'
catalog = json.loads((ROOT / 'web/gallery/catalog.json').read_text())


def sanitize(value):
    if isinstance(value, str):
        value = re.sub(r'(https?://[^\s?"\)]+)\?[^\s"\)]*', r'\1?[query omitted]', value)
        return value.replace(str(ROOT), '<repository>')[:700]
    if isinstance(value, list):
        return [sanitize(v) for v in value]
    if isinstance(value, dict):
        return {k: sanitize(v) for k, v in value.items()}
    return value


def validate(report):
    expected = {e['id']: e for e in catalog['examples']}
    assert len(report['examples']) == len(expected) == 607
    assert {r['id'] for r in report['examples']} == expected.keys()
    for row in report['examples']:
        entry = expected[row['id']]
        assert row['source_sha256'] == entry['source_sha256'], row['id']
        assert row['product_status'] == entry['status'], row['id']
        if entry['status'] == 'excluded':
            assert row['outcome'] == 'excluded'
        else:
            assert row['browser']['id'] == row['id']
            if row['browser']['stage'] == 'captured-scene':
                assert row['rust']['stage'].startswith('rust-'), row['id']
    assert report['counts'] == dict(collections.Counter(r['outcome'] for r in report['examples']))


if '--check' in sys.argv:
    report = json.loads(OUTPUT.read_text())
    validate(report)
    print('All 607 recorded runtime outcomes match the pinned catalog')
    sys.exit(0)

rows = []
for entry in catalog['examples']:
    name = entry['id']
    row = {k: entry[k] for k in ('id', 'source', 'source_sha256')}
    row['product_status'] = entry['status']
    if entry['status'] == 'excluded':
        row['outcome'] = 'excluded'
    else:
        browser = json.loads((CACHE / f'{name}.result.json').read_text())
        assert browser['source_sha256'] == entry['source_sha256'], name
        browser['blockers'] = list(dict.fromkeys(browser.get('blockers', [])))
        row['browser'] = sanitize(browser)
        if browser['stage'] == 'captured-scene':
            rust = json.loads((CACHE / f'{name}.rust.json').read_text())
            row['rust'] = sanitize(rust)
            row['outcome'] = rust['stage']
            row['local_image'] = f'.cache/gallery-runtime/{name}.png'
        else:
            row['outcome'] = browser['stage']
        row['probe_limit'] = any(b.startswith('Probe resource limit:') for b in browser.get('blockers', []))
    rows.append(row)
report = {
    'revision': catalog.get('revision', '148ef33ecb6d2502ff796d4554abd1549c95d519'),
    'method': 'Baseline recorded before Core expansion. Instrumented pinned upstream HTML executed in isolated Chrome contexts. Actual main render calls and unsupported dispatches were intercepted. Eligible observed scene data was rendered separately by the native Rust/WebGPU renderer. This does not execute the example behavior in Rust or establish pixel parity.',
    'limits': ['A sampled frame cannot validate every GUI branch, animation, interaction, or late asset load.', 'Upstream JavaScript constructs geometry and decodes assets for the probe; this does not prove native loader support.', '2000 objects, 1M vertices, and 8M total texture pixels are diagnostic transfer limits, not engine limits.', 'Only independently implemented and tested product ports appear in the gallery.', 'The runtime bridge predates Core expansion: its missing-feature labels are historical, not current Core capability claims. See core-expansion.md.'],
    'counts': dict(collections.Counter(r['outcome'] for r in rows)),
    'examples': rows,
}
validate(report)
OUTPUT.write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n')
blockers = collections.Counter(b for r in rows for b in set(r.get('browser', {}).get('blockers', [])))
lines = ['# All-example runtime reproduction attempts', '', report['method'], '',
         '**These are attempts, not 605 completed ports.** The gallery contains 16 partial behavioral ports. Captured frame renders are diagnostic artifacts and are not listed as working examples.', '',
         '| Outcome | Examples |', '| --- | ---: |']
lines += [f'| {k} | {v} |' for k, v in report['counts'].items()]
lines += ['', '## Limits', ''] + [f'- {v}' for v in report['limits']]
lines += ['', 'A runtime prerequisite is a requirement observed at an intercepted call or scene inspection. It can be a missing engine feature, an untranslated addon, or a limitation of the diagnostic bridge. It does not prove that a dedicated Rust implementation is impossible. Blank/background-only frames are not counted as successful scene reproduction.', '',
          '## Common observed prerequisites', '', 'Counts overlap and describe the sampled execution, not all possible branches.', '', '| Prerequisite | Examples |', '| --- | ---: |']
lines += [f'| {b} | {n} |' for b, n in blockers.most_common(25)]
lines += ['', '## Repeat the experiment', '',
          'Run from the repository root. The pinned archive is prepared by `tools/gallery/build.py`; Playwright uses the Chrome configuration in this repository. Native rendering requires a working GPU backend. The extracted assets occupy about 400 MiB and snapshots can occupy several GiB.', '',
          '```sh', 'python3 tools/gallery/build.py', 'python3 tools/gallery/prepare_runtime.py', 'npm ci --ignore-scripts',
          '# Keep this loopback server running in another terminal:', 'python3 -m http.server 8175 --bind 127.0.0.1',
          '# Execute in the original terminal:', 'node tools/gallery/runtime/run.mjs',
          'cargo run --locked --example gallery_render_probe -- .cache/gallery-runtime',
          'python3 tools/gallery/runtime/report.py', '```', '',
          'The browser runner resumes existing results. Pass explicit example IDs to repeat cases. Results are stored under `.cache/gallery-runtime/`; only bounded, sanitized outcome records are checked in. Geometry, images, random seeds and captured time vary between runs. The separate gallery tests compare controlled scenes with the pinned reference.', '',
          '[Machine-readable results](gallery-runtime-results.json) · [Runnable ports and remaining differences](examples-coverage.md) · [Source and native glTF importer review](gallery-port-attempts.md)', '',
          '## Every example', '', '| Example | Observed outcome | Prerequisites / diagnostic note |', '| --- | --- | --- |']
for row in rows:
    note = '; '.join(row.get('browser', {}).get('blockers', [])) or row.get('rust', {}).get('error', '')
    if row.get('rust', {}).get('stage', '').endswith('frame-rendered'):
        note = 'One sampled frame only; no behavior or pixel-parity claim'
    if row.get('probe_limit'):
        note = 'PROBE LIMIT: ' + note
    lines.append(f'| [{row["id"]}]({row["source"]}) | {row["outcome"]} | {note.replace("|", "/").replace(chr(10), " ")} |')
(ROOT / 'docs/gallery-runtime-results.md').write_text('\n'.join(lines) + '\n')
print(json.dumps(report['counts']))
