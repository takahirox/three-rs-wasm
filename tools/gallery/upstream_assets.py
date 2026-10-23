#!/usr/bin/env python3
"""The demo's files that are the pinned three.js r186's own example assets
( web/examples/upstream-assets.json ).

They stay in the repository, for the tests and tools that read them, but the
published site leaves them out and loads them from the three.js repository at
the pinned commit on jsDelivr ( tools/build_site.py, web/examples/upstream.js ),
which keeps the site under GitHub Pages' 1 GB. A file is listed only when its
bytes equal an r186 file's under examples/ ( .cache/three-r186, from
tools/compat/prepare_reference.py ). JavaScript modules stay on the site, since
an import() bypasses upstream.js's fetch redirect, and so do files over
jsDelivr's 20 MB limit for GitHub files.

  upstream_assets.py           write upstream-assets.json
  upstream_assets.py --check   fail when it is stale
"""
import hashlib, json, subprocess, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
UPSTREAM = ROOT / '.cache/three-r186'
OUT = ROOT / 'web/examples/upstream-assets.json'
# The examples' assets, which the pages load through upstream.js; the gallery's
# own pages, styles and thumbnails stay on the site.
ASSETS = ('web/examples/assets/', 'web/examples/textures/', 'web/examples/models/', 'web/examples/environments/')
LIMIT = 20 * 1024 * 1024


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def mapping():
    commit = json.loads((ROOT / 'compat/three-r186/api.json').read_text())['baseline']['upstream_commit']
    if (UPSTREAM / '.revision').read_text().strip() != commit:
        sys.exit('.cache/three-r186 is not the pinned revision; run tools/compat/prepare_reference.py')
    by_size = {}
    for path in (UPSTREAM / 'examples').rglob('*'):
        if path.is_file():
            by_size.setdefault(path.stat().st_size, []).append(path)
    files = {}
    tracked = subprocess.check_output(['git', 'ls-files', '-z', 'web'], cwd=ROOT).decode().split('\0')
    for name in sorted(tracked):
        path = ROOT / name
        if not name.startswith(ASSETS) or not path.is_file():
            continue
        if name.endswith(('.js', '.mjs')) or path.stat().st_size > LIMIT:
            continue
        candidates = by_size.get(path.stat().st_size, [])
        if not candidates:
            continue
        own = digest(path)
        # The same name first, then the shortest path.
        for candidate in sorted(candidates, key=lambda c: (c.name != path.name, len(str(c)), str(c))):
            if digest(candidate) == own:
                # By its path from the site's root ( web/ ), as the pages see it.
                files[name.removeprefix('web/')] = candidate.relative_to(UPSTREAM).as_posix()
                break
    return {'commit': commit, 'base': None, 'files': files}


def main():
    data = mapping()
    text = json.dumps(data, indent=1, ensure_ascii=False) + '\n'
    if '--check' in sys.argv:
        if not OUT.exists() or OUT.read_text() != text:
            sys.exit(f'{OUT.relative_to(ROOT)} is stale; run tools/gallery/upstream_assets.py')
    else:
        OUT.write_text(text)
        size = sum((ROOT / 'web' / name).stat().st_size for name in data['files'])
        print(f"{len(data['files'])} files, {size / 1e6:.0f} MB")


if __name__ == '__main__':
    main()
