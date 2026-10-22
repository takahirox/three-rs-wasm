"""Package tracked demo assets and the release Wasm bundle for GitHub Pages."""
import json
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / ".cache/pages/three-rs-wasm"


def examples(site):
    """threejs.org's URLs: examples/#<id> for the gallery and examples/<id>.html
    for each example. The pages are web/gallery's, with their <base> there, so
    their relative URLs and the Wasm's asset paths stay those of web/gallery/."""
    gallery = site / "web/gallery"
    out = site / "examples"
    out.mkdir()
    base = '<base href="../web/gallery/">'
    index = (gallery / "index.html").read_text()
    index = index.replace("<head>", f'<head>\n\t\t{base}<meta name="three-rs-layout" content="examples">', 1)
    (out / "index.html").write_text(index)
    for page in sorted((gallery / "examples").glob("*.html")):
        (out / page.name).write_text(page.read_text().replace('<base href="../">', base, 1))


def upstream_base(site, value):
    """The published pages' part of the upstream listing loads from `value`
    ( web/gallery/upstream.js ), as the whole listing does."""
    for page in sorted((site / "web/gallery/examples").glob("*.html")):
        page.write_text(page.read_text().replace(
            '<script type="application/json" id="upstream-assets">{"base":null,',
            '<script type="application/json" id="upstream-assets">{"base":' + json.dumps(value) + ',', 1))


def main():
    # The gallery's Wasm ( scripts/build-gallery ): the glue and its inline_js
    # modules, the primary module, the parts and their listing.
    gallery = ROOT / "web/pkg/gallery"
    required = [gallery / name for name in ("three_rs_wasm.js", "three_rs_wasm_bg.wasm", "parts.json")]
    if not all(path.is_file() for path in required):
        raise SystemExit("Build web/pkg/gallery with scripts/build-gallery first")
    bundle = sorted(path for path in gallery.rglob("*") if path.is_file())
    # Every example's page, which the build writes with its parts.
    pages = sorted((ROOT / "web/gallery/examples").glob("*.html"))
    if not pages:
        raise SystemExit("Build web/gallery/examples with scripts/build-gallery first")
    bundle += pages
    # webgpu_tsl_editor's own Wasm: the three-tsl crate ( three r186 transpiled to Rust ).
    tsl = [ROOT / "web/tsl-pkg" / name for name in ("three_tsl.js", "three_tsl_bg.wasm")]
    if not all(path.is_file() for path in tsl):
        raise SystemExit("Build web/tsl-pkg with wasm-pack ( tsl/, --profile web ) first")
    bundle += tsl + sorted(path for path in (ROOT / "web/tsl-pkg/snippets").rglob("*") if path.is_file())
    if SITE.exists():
        shutil.rmtree(SITE)
    files = subprocess.check_output(
        ["git", "ls-files", "-z", "web", "LICENSE", "LICENSE-THREE", "LICENSE-MATERIALX", "LICENSE-RAPIER"], cwd=ROOT
    ).decode().split("\0")
    # The pinned three.js r186's own example assets load from its repository at that
    # commit ( tools/gallery/upstream_assets.py, web/gallery/upstream.js ).
    upstream = json.loads((ROOT / "web/gallery/upstream-assets.json").read_text())
    for source in [ROOT / name for name in files if name and name not in upstream["files"]] + bundle:
        target = SITE / source.relative_to(ROOT)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    upstream["base"] = f"https://cdn.jsdelivr.net/gh/mrdoob/three.js@{upstream['commit']}/"
    (SITE / "web/gallery/upstream-assets.json").write_text(json.dumps(upstream, indent=1, ensure_ascii=False) + "\n")
    upstream_base(SITE, upstream["base"])
    examples(SITE)
    (SITE / "index.html").write_text(
        '<!doctype html><html lang="en"><meta charset="utf-8">'
        '<title>three-rs-wasm examples</title>'
        '<meta http-equiv="refresh" content="0;url=examples/">'
        '<a href="examples/">Open the examples gallery</a></html>\n'
    )
    print(SITE.relative_to(ROOT))


if __name__ == "__main__":
    main()
