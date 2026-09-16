#!/usr/bin/env python3
"""Keep capability accounting separate from proof of implementation."""
import argparse
import json
from pathlib import Path

ROOT=Path(__file__).resolve().parents[2]
parser=argparse.ArgumentParser()
parser.add_argument('--members',type=Path,required=True)
args=parser.parse_args()
items=json.loads(args.members.read_text())['items']
path=ROOT/'compat/three-r186/implementation.json'
evidence=json.loads(path.read_text()) if path.is_file() else {}
required={item['api'] for item in items if item['status'].startswith('implemented-')}
missing=sorted(required-evidence.keys())
errors=[]
for name, entry in evidence.items():
    if name not in required:
        errors.append(f'Unknown or excluded entry: {name}')
    for field in ('rust','test'):
        value=entry.get(field,'')
        if not value or not (ROOT/value.split('#')[0]).is_file():
            errors.append(f'{name}: missing {field} evidence')
        elif '#' in value:
            file,anchor=value.split('#',1)
            if anchor not in (ROOT/file).read_text():
                errors.append(f'{name}: unresolved {field} anchor {anchor}')
    if not entry.get('mapping'):
        errors.append(f'{name}: missing capability mapping explanation')
if missing or errors:
    print(f'Implementation evidence: FAIL ({len(missing)} missing of {len(required)} required APIs)')
    for message in errors[:20]+missing[:30]: print('  '+message)
    raise SystemExit(1)
print(f'Implementation evidence: PASS ({len(required)} APIs)')
