#!/usr/bin/env python3
"""Moves the ports' captured WGSL from one release to the next.

Each port runs, from gallery/src/browser/<dir>/*.wgsl, shaders its original page
generated. Given both releases' captures ( tools/compat/capture_wgsl.mjs ), a
port's file is found among the old capture: by its text, or, for a shader the
page builds in a state the capture did not reach ( a GUI setting ), as the most
similar shader of the same stage. Its counterpart in the new capture of the same
page is the shader most similar to that one. The file moves by a three-way merge
( the old shader to the new, applied to the file ). When the result keeps the
file's interface ( bindings, uniform and storage structs, stage inputs and
outputs ), it replaces the file; otherwise the file is listed, since the port's
Rust side binds and fills that interface.

  rebase_wgsl.py <old-capture.json>… -- <new-capture.json>… [--write] [--why]

--why prints, for each file left with a changed interface, the interface lines
the new shader removes ( - ) and adds ( + ).
"""
import difflib
import json
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
# The release the new capture is of: the generated shaders' first line.
RELEASE = json.loads((ROOT / 'compat/three-r187/api.json').read_text())['baseline']['release'][1:]
HEADER = re.compile(r'^// Three\.js r(\d+) - Node System\n')


def load(paths):
    out = {}
    for p in paths:
        for k, v in json.loads(pathlib.Path(p).read_text()).items():
            if isinstance(v, list) and v:
                out[k] = v
    return out


def stage(code):
    return 'compute' if '@compute' in code else 'vertex' if '@vertex' in code else 'fragment'


def interface(code):
    """The parts the Rust side depends on: declarations with @group/@binding,
    the structs they and the entry points use, and the entry points' signatures."""
    decls = re.findall(r'@binding\(\s*\d+\s*\)\s*@group\(\s*\d+\s*\)[^;]*;', code)
    structs = dict(re.findall(r'struct\s+(\w+)\s*\{([^}]*)\}', code))
    used = set()
    for d in decls:
        used.update(re.findall(r'\b(\w+)\b', d))
    entries = re.findall(r'@(?:vertex|fragment|compute)[^{]*\{', code)
    for e in entries:
        used.update(re.findall(r'\b(\w+)\b', e))
    # Structs reached from the used ones ( nested structs, output structs ).
    while True:
        more = {w for name in used & structs.keys() for w in re.findall(r'\b(\w+)\b', structs[name])} - used
        if not more & structs.keys():
            break
        used |= more
    norm = lambda s: re.sub(r'\s+', ' ', s).strip()
    return (tuple(norm(d) for d in decls),
            tuple(sorted((n, norm(structs[n])) for n in used & structs.keys())),
            tuple(norm(e) for e in entries))


def similar(a, b):
    """By lines: shaders run to tens of thousands of characters."""
    return difflib.SequenceMatcher(None, a.splitlines(), b.splitlines(), autojunk=False).ratio()


def closest(body, shaders):
    """The most similar shader, by the cheap bound first."""
    lines = body.splitlines()
    bound = lambda s: difflib.SequenceMatcher(None, lines, s.splitlines(), autojunk=False).quick_ratio()
    shaders = sorted(shaders, key=lambda s: -bound(s))[:6]
    return max(shaders, key=lambda s: similar(body, s), default=None)


def merge(ours, base, theirs):
    """git merge-file: base -> theirs applied to ours, or None on a conflict."""
    with tempfile.TemporaryDirectory() as d:
        paths = []
        for name, text in (('ours', ours), ('base', base), ('theirs', theirs)):
            paths.append(pathlib.Path(d, name))
            paths[-1].write_text(text)
        r = subprocess.run(['git', 'merge-file', '-p', *map(str, paths)], capture_output=True, text=True)
        return r.stdout if r.returncode == 0 else None


def main():
    args = sys.argv[1:]
    write = '--write' in args
    why = '--why' in args
    args = [a for a in args if a not in ('--write', '--why')]
    split = args.index('--')
    old, new = load(args[:split]), load(args[split + 1:])
    # Old shader text -> pages that compiled it.
    pages = {}
    for page, shaders in old.items():
        for s in shaders:
            pages.setdefault(s, []).append(page)
    replaced, differ, conflict, unknown, absent = [], [], [], [], []
    files = [(f, HEADER.match(f.read_text())) for f in sorted((ROOT / 'gallery/src/browser').rglob('*.wgsl'))]
    # Files already on the new release are left alone.
    files = [(f, f.read_text()[m.end():]) for f, m in files if m and m.group(1) != RELEASE]
    # A port's pages: those its other files were found in by their text.
    ported = {}
    for f, body in files:
        ported.setdefault(f.parent, set()).update(pages.get(body, []))
    for f, body in files:
        rel = f.relative_to(ROOT)
        found = [p for p in pages.get(body, []) if p in new]
        if found:
            base = body
        else:
            among = ported[f.parent] or old.keys()
            base = closest(body, [s for p in among for s in old[p] if stage(s) == stage(body)])
            if base is None or similar(body, base) < 0.9:
                unknown.append(rel)
                continue
            found = [p for p in pages[base] if p in new]
            if not found:
                absent.append(rel)
                continue
        theirs = closest(base, [s for p in found for s in new[p] if stage(s) == stage(body)])
        if theirs is None:
            absent.append(rel)
            continue
        result = theirs if base == body else merge(body, base, theirs)
        if result is None:
            conflict.append((rel, found[0]))
        elif interface(result) == interface(body):
            replaced.append(rel)
            if write:
                f.write_text(f'// Three.js r{RELEASE} - Node System\n' + result)
        else:
            differ.append((rel, found[0], interface(body), interface(result)))
    print(f'{len(replaced)} replaced ( same interface ), {len(differ)} with a changed interface, '
          f'{len(conflict)} conflicting with the file\'s edits, {len(unknown)} not in the old capture, '
          f'{len(absent)} whose pages were not captured')
    for rel, page, was, now in differ:
        print('interface', rel, page)
        if why:
            flat = lambda i: [str(x) for part in i for x in part]
            for line in difflib.unified_diff(flat(was), flat(now), lineterm='', n=0):
                if line[:1] in '+-' and line[:3] not in ('---', '+++'):
                    print('   ', line[:300])
    for rel, page in conflict:
        print('conflict', rel, page)
    for rel in unknown:
        print('unknown', rel)
    for rel in absent:
        print('uncaptured', rel)


main()
