#!/usr/bin/env python3
"""Split the whole gallery into one primary module and parts.

  build.py RAW OUT

RAW is the gallery linked with `--emit-relocs` ( scripts/build-gallery ). OUT
gets wasm-bindgen's glue and the primary module ( three_rs_wasm.js,
three_rs_wasm_bg.wasm ), a part per module and per shared chunk
( parts/<name>.wasm ) and parts.json, the parts each module's page loads
( tools/gallery/split/assign.py chooses them ).

The functions move with Binaryen's wasm-split --multi-split; the read-only data
only a part references moves into it as active segments at the same addresses,
so the primary module's memory holds it once the part is instantiated."""
import json
import os
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


def main(raw, out):
    out = Path(out)
    if out.exists():
        shutil.rmtree(out)
    (out / 'parts').mkdir(parents=True)
    # $SPLIT_WORK keeps the intermediate modules ( and their names ) for inspection.
    keep = os.environ.get('SPLIT_WORK')
    with tempfile.TemporaryDirectory() as tmp:
        work = Path(keep) if keep else Path(tmp)
        work.mkdir(parents=True, exist_ok=True)
        split(raw, out, work)


def split(raw, out, work):
    parts, loads = assign.assign(raw, modules.modules())
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
    manifest = []
    for name, (functions, _) in sorted(parts.items()):
        keep = [f'F{i}' for i in functions if f'F{i}' in present]
        if keep:
            manifest.append('\n'.join([name] + keep))
    (work / 'manifest.txt').write_text('\n\n'.join(manifest) + '\n')
    subprocess.run(['wasm-split', '--multi-split', str(bg), '--manifest', str(work / 'manifest.txt'),
                    '--out-prefix', str(work) + '/', '-o', str(work / 'primary.wasm'),
                    '--no-placeholders', '--export-prefix', '%', '-g', *FEATURES], check=True)
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


if __name__ == '__main__':
    main(*sys.argv[1:])
