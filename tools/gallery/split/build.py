#!/usr/bin/env python3
"""Split the whole gallery into one primary module and parts.

  build.py RAW OUT

RAW is the gallery linked with `--emit-relocs` ( scripts/build-gallery ). OUT
gets wasm-bindgen's glue and the primary module ( three_rs_wasm.js,
three_rs_wasm_bg.wasm ), a part per module and per shared chunk
( parts/<name>.wasm ), the glue of the parts that need their own
( parts/<name>.js, tools/gallery/split/glue.mjs ) and parts.json: the parts
each module's page loads ( tools/gallery/split/assign.py chooses them ) and
those with glue.

The functions move with Binaryen's wasm-split --multi-split; the read-only data
only a part references moves into it as active segments at the same addresses,
so the primary module's memory holds it once the part is instantiated."""
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import assign  # noqa: E402
import modules  # noqa: E402
import wasm  # noqa: E402

FEATURES = ['--enable-bulk-memory', '--enable-nontrapping-float-to-int', '--enable-sign-ext',
            '--enable-mutable-globals', '--enable-reference-types', '--enable-multivalue']


def bindgen():
    """The wasm-bindgen CLI of the version Cargo.lock pins: $WASM_BINDGEN, or the
    one wasm-pack installed ( it builds the gallery first, scripts/build-gallery )."""
    found = os.environ.get('WASM_BINDGEN')
    if found:
        return found
    lock = (Path(__file__).resolve().parents[3] / 'Cargo.lock').read_text()
    version = lock.split('name = "wasm-bindgen"\nversion = "')[1].split('"')[0]
    for cache in (Path.home() / 'Library/Caches/.wasm-pack', Path.home() / '.cache/.wasm-pack'):
        for path in sorted(cache.glob('*/wasm-bindgen')) if cache.exists() else []:
            out = subprocess.run([str(path), '--version'], capture_output=True, text=True).stdout
            if out.split()[-1:] == [version]:
                return str(path)
    raise SystemExit(f'wasm-bindgen {version} not found: build with wasm-pack first or set WASM_BINDGEN')


def carve(segments, moved):
    """The segments without the moved intervals."""
    moved = sorted(moved)
    out = []
    for addr, data in segments:
        start, end = addr, addr + len(data)
        cursor = start
        for a, n in moved:
            if a + n <= cursor or a >= end:
                continue
            if a > cursor:
                out.append((cursor, data[cursor - start:a - start]))
            cursor = max(cursor, a + n)
        if cursor < end:
            out.append((cursor, data[cursor - start:]))
    return out



def trampoline_slot(body):
    """A wasm-split trampoline's table and slot: its body is local.gets, an
    i32.const slot and a call_indirect through the parts' table."""
    i = 0
    _, i = wasm.leb(body, i)  # body size
    groups, i = wasm.leb(body, i)
    for _ in range(groups):
        _, i = wasm.leb(body, i)
        i += 1
    while body[i] == 0x20:
        _, i = wasm.leb(body, i + 1)
    if body[i] != 0x41:
        return None
    slot, i = wasm.sleb(body, i + 1)
    if body[i] != 0x11:
        return None
    _, i = wasm.leb(body, i + 1)
    table, i = wasm.leb(body, i)
    return (table, slot) if body[i:] == b'\x0b' else None


def rehome(primary, parts, targets):
    """What only a part uses, the part holds itself: the JavaScript functions
    it calls it imports from wasm-bindgen's glue instead of through the
    primary module, and the exports whose functions it holds ( its programs,
    its classes' methods ) it exports itself, instead of the primary
    module's trampolines to them. targets: trampoline -> ( part, function ).
    The glue follows ( tools/gallery/split/glue.mjs )."""
    ps = wasm.sections(primary)
    nimp = wasm.imported_functions(primary, ps)
    js = [(mod, field) for mod, field, kind, _ in wasm.imports(primary, ps) if kind == 0]
    exports = wasm.exports(primary, ps)
    by_name = {nm: idx for nm, kind, idx in exports if kind == 0}
    moved = {}
    keep = []
    for nm, kind, idx in exports:
        if kind == 0 and not nm.startswith('%') and idx in targets:
            part, f = targets[idx]
            moved.setdefault(part, []).append((nm, 0, f))
        else:
            keep.append((nm, kind, idx))
    for part, b in parts.items():
        s = wasm.sections(b)
        imps = []
        for mod, field, kind, payload in wasm.imports(b, s):
            idx = by_name.get(field) if mod == 'primary' and kind == 0 else None
            if idx is not None and idx < nimp:
                mod, field = js[idx]
            imps.append((mod, field, kind, payload))
        new = {2: wasm.encode_imports(imps)}
        if part in moved:
            new[7] = wasm.encode_exports(wasm.exports(b, s) + moved[part])
        parts[part] = wasm.replace_sections(b, new)
    # The primary module's exports of the imports the parts now import themselves.
    keep = [(nm, kind, idx) for nm, kind, idx in keep
            if not (nm.startswith('%') and kind == 0 and idx < nimp)]
    primary = wasm.replace_sections(primary, {7: wasm.encode_exports(keep)})
    return primary, parts


def untrampoline(primary, parts):
    """The primary module's function table holds a trampoline for each part
    function a function pointer names ( a vtable's methods, a closure's
    shim ). Leave those slots empty and let the part write its function into
    them as it is instantiated, as it writes its own table: no trampolines."""
    ps = wasm.sections(primary)
    nimp = wasm.imported_functions(primary, ps)
    names = wasm.function_names(primary, ps)
    code = wasm.section(primary, ps, 10)[2]
    bodies = wasm.bodies(primary, ps)
    slots = {}
    for idx, nm in names.items():
        if idx >= nimp and nm.startswith('trampoline_'):
            s, e = bodies[idx - nimp]
            found = trampoline_slot(primary[code + s:code + e])
            if found:
                slots[idx] = found
    exported_tables = {nm: idx for nm, kind, idx in wasm.exports(primary, ps) if kind == 1}
    # ( primary table, slot ) -> ( part, function ) from the parts' own segments.
    owner = {}
    for part, b in parts.items():
        s = wasm.sections(b)
        tables = [exported_tables[field] for mod, field, kind, _ in wasm.imports(b, s) if kind == 1]
        for seg in wasm.elem_segments(b, s):
            if seg['offset'] is None:
                continue
            base = wasm.offset_value(seg['offset'])
            for k, (kind, f) in enumerate(seg['items']):
                if kind == 'func':
                    owner[(tables[seg['table']], base + k)] = (part, f)
    table0 = {v: k for k, v in exported_tables.items()}.get(0)
    if table0 is None:
        raise SystemExit('split: the primary module does not export its function table')
    writes = {}
    segs = []
    for seg in wasm.elem_segments(primary, ps):
        if seg['flags'] != 0:
            segs.append(seg)
            continue
        # Runs of the slots that stay, each its own segment: a table's other
        # slots start empty.
        base = wasm.offset_value(seg['offset'])
        run = []
        for k, (kind, f) in enumerate(seg['items'] + [('end', None)]):
            target = owner.get(slots.get(f)) if kind == 'func' else None
            if target:
                writes.setdefault(target[0], []).append((base + k, target[1]))
            if kind == 'func' and not target:
                if not run:
                    start = base + k
                run.append(('func', f))
            elif run:
                segs.append(dict(flags=0, table=0, offset=b'\x41' + wasm.enc_s(start) + b'\x0b', kind=0,
                                 items=run))
                run = []
    primary = wasm.replace_sections(primary, {9: wasm.encode_elem(segs)})
    primary, parts = rehome(primary, parts, {f: owner[slot] for f, slot in slots.items() if slot in owner})
    for part, entries in writes.items():
        b = parts[part]
        s = wasm.sections(b)
        imps = wasm.imports(b, s)
        if wasm.section(b, s, 4):
            raise SystemExit(f'split: {part} defines a table')
        table = sum(1 for i in imps if i[2] == 1)
        imps.append(('primary', table0, 1, b'\x70\x00\x00'))
        segs = wasm.elem_segments(b, s)
        for slot, f in sorted(entries):
            segs.append(dict(flags=2, table=table, offset=b'\x41' + wasm.enc_s(slot) + b'\x0b', kind=0,
                             items=[('func', f)]))
        parts[part] = wasm.replace_sections(b, {2: wasm.encode_imports(imps), 9: wasm.encode_elem(segs)})
    return primary, parts, sum(len(v) for v in writes.values())

def main(raw, out):
    out = Path(out)
    # The split's own output; other builds share the directory ( web/build/tsl ).
    for name in ('parts', 'snippets'):
        shutil.rmtree(out / name, ignore_errors=True)
    for path in [*out.glob('three_rs_wasm*'), out / 'parts.json']:
        path.unlink(missing_ok=True)
    (out / 'parts').mkdir(parents=True)
    # $SPLIT_WORK keeps the intermediate modules ( and their names ) for inspection.
    keep = os.environ.get('SPLIT_WORK')
    with tempfile.TemporaryDirectory() as tmp:
        work = Path(keep) if keep else Path(tmp)
        work.mkdir(parents=True, exist_ok=True)
        split(raw, out, work)


def split(raw, out, work):
    parts, loads, imports = assign.assign(raw, modules.modules())
    b = Path(raw).read_bytes()
    stripped = work / 'gallery.wasm'
    # Functions travel by index: wasm-bindgen demangles names into strings that may collide.
    stripped.write_bytes(wasm.with_index_names(wasm.without_custom(b, {'linking', 'reloc.CODE', 'reloc.DATA'})))
    subprocess.run([bindgen(), '--target', 'web', '--out-dir', str(out), '--out-name', 'three_rs_wasm',
                    '--no-typescript', '--keep-debug', str(stripped)], check=True)
    bg = out / 'three_rs_wasm_bg.wasm'
    bb = bg.read_bytes()
    secs = wasm.sections(bb)
    present = set(wasm.function_names(bb, secs).values())
    # wasm-bindgen's adapters ( `F<i> externref shim` ) go with the function
    # they adapt: an export's into its part, so a part's programs are its own
    # exports, and an import's into the part that alone calls it.
    adapters = {}
    for nm in present:
        m = re.fullmatch(r'F(\d+) ((?:externref|multivalue) shim(?: multivalue shim)?)', nm)
        if m:
            adapters.setdefault(int(m[1]), []).append(nm)
    manifest = []
    for name, (functions, _) in sorted(parts.items()):
        keep = [f'F{i}' for i in functions if f'F{i}' in present]
        keep += [a for i in functions for a in adapters.get(i, [])]
        keep += [a for i, p in imports.items() if p == name for a in adapters.get(i, [])]
        if keep:
            manifest.append('\n'.join([name] + keep))
    # JavaScript functions several modules' parts call through the primary
    # module: each of those parts' glue hands the function to the primary
    # module's glue ( glue.mjs ), so the snippets they come from load with them.
    fields = [f for _, f, kind, _ in wasm.imports(b, wasm.sections(b)) if kind == 0]
    (work / 'shared-imports.json').write_text(json.dumps(
        {fields[i]: [m for m in p if m in parts] for i, p in imports.items() if isinstance(p, list)}))
    (work / 'manifest.txt').write_text('\n\n'.join(manifest) + '\n')
    subprocess.run(['wasm-split', '--multi-split', str(bg), '--manifest', str(work / 'manifest.txt'),
                    '--out-prefix', str(work) + '/', '-o', str(work / 'primary.wasm'),
                    '--no-placeholders', '--export-prefix', '%', '-g', *FEATURES], check=True)
    # Function pointers to part functions: written by the parts, not trampolines.
    names = [n for n in parts if (work / f'{n}.wasm').exists()]
    primary, written, moved = untrampoline((work / 'primary.wasm').read_bytes(),
                                           {n: (work / f'{n}.wasm').read_bytes() for n in names})
    (work / 'primary.wasm').write_bytes(primary)
    for n, b in written.items():
        (work / f'{n}.wasm').write_bytes(b)
    print(f'split: {moved} function table slots written by the parts')
    # The data: out of the primary module's segments, into the parts'.
    original = [(addr, bb[secs_data_start + off:secs_data_start + off + size])
                for off, addr, size in wasm.data_segments(bb, secs)
                for secs_data_start in [wasm.section(bb, secs, 11)[2]]]
    memory = bytearray(max(a + len(d) for a, d in original))
    for a, d in original:
        memory[a:a + len(d)] = d
    # A part whose functions wasm-bindgen dropped as unused keeps its data in the primary module.
    parts = {k: v for k, v in parts.items() if (work / f'{k}.wasm').exists()}
    moved = [iv for _, (_, data) in parts.items() for iv in data]
    primary = (work / 'primary.wasm').read_bytes()
    ps = wasm.sections(primary)
    kept = carve(original, moved)
    payload = wasm.data_payload(kept)
    new = {11: payload}
    if wasm.section(primary, ps, 12):
        new[12] = wasm.enc(len(kept))
    (work / 'primary.data.wasm').write_bytes(wasm.replace_sections(primary, new))
    loaded = {}
    for name, (functions, data) in sorted(parts.items()):
        path = work / f'{name}.wasm'
        part = path.read_bytes()
        segs = [(a, bytes(memory[a:a + n])) for a, n in sorted(data)]
        if segs:
            if not wasm.has_memory_import(part, wasm.sections(part)):
                raise SystemExit(f'{name}: no memory import for its data')
            part = wasm.replace_sections(part, {11: wasm.data_payload(segs)})
        (work / f'{name}.data.wasm').write_bytes(part)
        loaded[name] = True
    # Optimize each module on its own.
    debug = ['-g'] if os.environ.get('SPLIT_WORK') else []
    subprocess.run(['wasm-opt', '-O', *debug, *FEATURES, str(work / 'primary.data.wasm'), '-o', str(bg)], check=True)
    for name in loaded:
        subprocess.run(['wasm-opt', '-O', *debug, *FEATURES, str(work / f'{name}.data.wasm'),
                        '-o', str(out / 'parts' / f'{name}.wasm')], check=True)
    listing = {m: [p for p in ps_ if p in loaded] for m, ps_ in sorted(loads.items())}
    listing['core'] = []
    (out / 'parts.json').write_text(json.dumps(listing, indent=1) + '\n')
    # The glue, split as the modules are.
    subprocess.run(['node', str(Path(__file__).resolve().parent / 'glue.mjs'), str(out),
                    str(work / 'shared-imports.json')], check=True)


if __name__ == '__main__':
    main(*sys.argv[1:])
