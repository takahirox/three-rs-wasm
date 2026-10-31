#!/usr/bin/env python3
"""The demo's files that are three.js's own example assets
( web/examples/upstream-assets.json ).

They stay in the repository, for the tests and tools that read them, but the
published site leaves them out and loads them from the three.js repository on
jsDelivr ( tools/build_site.py, web/examples/upstream.js ), which keeps the site
under GitHub Pages' 1 GB. jsDelivr no longer serves the repository at a new
commit ( "Package size exceeded the configured limit of 50 MB" ), only the
files it already holds, so the assets load from r186's commit ( ASSETS_COMMIT )
whatever the pinned release: a file is listed only when its bytes equal a file's
under examples/ at that commit ( its archive, .cache/three-<commit>.tar.gz,
downloaded when missing ). Files r187 changed or added stay on the site.
JavaScript modules stay on the site, since an import() bypasses upstream.js's
fetch redirect, and so do files over jsDelivr's 20 MB limit for GitHub files.

  upstream_assets.py           write upstream-assets.json
  upstream_assets.py --check   fail when it is stale
"""
import hashlib, json, subprocess, sys, tarfile, urllib.request
from pathlib import Path, PurePosixPath

ROOT = Path(__file__).resolve().parents[2]
# r186, whose files jsDelivr serves.
ASSETS_COMMIT = '148ef33ecb6d2502ff796d4554abd1549c95d519'
OUT = ROOT / 'web/examples/upstream-assets.json'
# The examples' assets, which the pages load through upstream.js; the gallery's
# own pages, styles and thumbnails stay on the site.
ASSETS = ('web/examples/assets/', 'web/examples/textures/', 'web/examples/models/', 'web/examples/environments/')
LIMIT = 20 * 1024 * 1024


def digest(data):
    return hashlib.sha256(data).hexdigest()


def upstream_files():
    """{ size: [ ( path under the repository, digest ) ] } of examples/ at ASSETS_COMMIT."""
    archive = ROOT / '.cache' / f'three-{ASSETS_COMMIT}.tar.gz'
    if not archive.exists():
        archive.parent.mkdir(parents=True, exist_ok=True)
        request = urllib.request.Request(f'https://codeload.github.com/mrdoob/three.js/tar.gz/{ASSETS_COMMIT}', headers={'User-Agent': 'three-rs-wasm-tests'})
        with urllib.request.urlopen(request, timeout=300) as response:
            archive.write_bytes(response.read())
    by_size = {}
    with tarfile.open(archive) as tar:
        for entry in tar:
            parts = PurePosixPath(entry.name).parts
            if len(parts) < 3 or parts[1] != 'examples' or not entry.isfile():
                continue
            by_size.setdefault(entry.size, []).append((PurePosixPath(*parts[1:]), digest(tar.extractfile(entry).read())))
    return by_size


def mapping():
    by_size = upstream_files()
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
        own = digest(path.read_bytes())
        # The same name first, then the shortest path.
        for candidate, theirs in sorted(candidates, key=lambda c: (c[0].name != path.name, len(str(c[0])), str(c[0]))):
            if theirs == own:
                # By its path from the site's root ( web/ ), as the pages see it.
                files[name.removeprefix('web/')] = candidate.as_posix()
                break
    return {'commit': ASSETS_COMMIT, 'base': None, 'files': files}


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
