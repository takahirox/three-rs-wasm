#!/usr/bin/env python3
"""Which gallery module each ported example runs ( web/gallery/modules.json ).

An example's module is the feature on its `create` arm in
gallery/src/browser/expanded.rs; the transpiler and the WebXR examples, which
the shell starts itself, use SHELL's. Examples the shell dispatches itself run
`core`. The page loads the gallery's primary Wasm module and its module's parts
( scripts/build-gallery, tools/gallery/split/ ).

  modules.py            write modules.json
  modules.py --check    fail when modules.json is stale
"""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'web/gallery/modules.json'
BROWSER = ROOT / 'gallery/src/browser'
# Examples the gallery's shell starts itself, with the module their page uses:
# the transpiler, the WebXR examples, and webgl_loader_gltf_avif's DRACOLoader
# ( gallery/src/browser/draco_loader.rs ).
SHELL = {28: 'draco_loader', 483: 'tsl_transpiler', 498: 'xr_app', 499: 'xr_app', 500: 'xr_app'}


def numbers(pattern):
    found = set()
    for part in pattern.split('|'):
        part = part.strip()
        if m := re.fullmatch(r'(\d+)\.\.=(\d+)', part):
            found.update(range(int(m[1]), int(m[2]) + 1))
        elif m := re.fullmatch(r'(\d+)\.\.(\d+)', part):
            found.update(range(int(m[1]), int(m[2])))
        elif part.isdigit():
            found.add(int(part))
        else:
            return None
    return found


def arms():
    """( example numbers, module ) for each `create` arm of expanded.rs."""
    source = (BROWSER / 'expanded.rs').read_text()
    start = source.index('    pub async fn create(')
    body = source[start:source.index('\n    pub fn update(', start)]
    feature = None
    for line in body.split('\n'):
        if m := re.fullmatch(r' {12}#\[cfg\(feature = "example_(\w+)"\)\]', line):
            feature = m[1]
            continue
        if re.match(r' {12}\S', line) and '=>' in line:
            found = numbers(line.strip().split('=>')[0])
            if found is not None:
                yield found, feature or 'core'
        if re.match(r' {12}\S', line):
            feature = None


def modules():
    """The modules that run examples."""
    return ({m for _, m in arms()} | set(SHELL.values())) - {'core'}


def mapping():
    by_number = dict(SHELL)
    for found, module in arms():
        for n in found:
            by_number.setdefault(n, module)
    catalog = json.loads((ROOT / 'web/gallery/catalog.json').read_text())
    return {'examples': {e['id']: by_number.get(e['port']['example'], 'core')
                         for e in sorted(catalog['examples'], key=lambda e: e['id']) if e.get('port')}}


def main():
    text = json.dumps(mapping(), indent=1) + '\n'
    if '--check' in sys.argv:
        if not OUT.exists() or OUT.read_text() != text:
            sys.exit(f'{OUT.relative_to(ROOT)} is stale; run tools/gallery/modules.py')
    else:
        OUT.write_text(text)


if __name__ == '__main__':
    main()
