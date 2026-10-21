#!/usr/bin/env python3
"""Which gallery module each ported example runs ( web/gallery/modules.json ).

Every example has its own program, a Rust function named after it
( `webgl_loader_texture_ktx`, `-` read as `_` ); its module is the module
whose file defines that program. Programs in the shell's files
( browser.rs, expanded.rs, gallery_scenes.rs ) run `core`, except where SHELL
names the module the page loads beside the shell: webgl_loader_gltf_avif's
DRACOLoader ( gallery/src/browser/draco_loader.rs ). The transpiler's page
calls its module's tsl_transpile instead of a program. The page loads the gallery's
primary Wasm module and its module's parts ( scripts/build-gallery,
tools/gallery/split/ ).

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
SHELL_FILES = {'browser', 'expanded', 'gallery_scenes'}
SHELL = {
    'webgl_loader_gltf_avif': 'draco_loader',
    'webgpu_tsl_transpiler': 'tsl_transpiler',
}
PROGRAM = re.compile(r'#\[(?:wasm_bindgen::prelude::)?wasm_bindgen\]\npub async fn (\w+)\(\n?\s*canvas: web_sys::HtmlCanvasElement')


def programs():
    """Program name -> the module whose file defines it."""
    found = {}
    for path in [BROWSER.parent / 'browser.rs', *sorted(BROWSER.rglob('*.rs'))]:
        rel = path.relative_to(BROWSER.parent)
        module = 'browser' if rel.name == 'browser.rs' and len(rel.parts) == 1 else rel.parts[1].removesuffix('.rs')
        for name in PROGRAM.findall(path.read_text()):
            found[name] = 'core' if module in SHELL_FILES else module
    return found


def mapping():
    catalog = json.loads((ROOT / 'web/gallery/catalog.json').read_text())
    found = programs()
    out = {}
    for e in sorted(catalog['examples'], key=lambda e: e['id']):
        if not e.get('port'):
            continue
        out[e['id']] = SHELL.get(e['id']) or found.get(e['id'].replace('-', '_'), 'core')
    return {'examples': out}


def modules():
    """The modules that run examples."""
    return set(mapping()['examples'].values()) - {'core'}


def main():
    text = json.dumps(mapping(), indent=1) + '\n'
    if '--check' in sys.argv:
        if not OUT.exists() or OUT.read_text() != text:
            sys.exit(f'{OUT.relative_to(ROOT)} is stale; run tools/gallery/modules.py')
    else:
        OUT.write_text(text)


if __name__ == '__main__':
    main()
