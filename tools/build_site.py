"""Package tracked demo assets and the release Wasm bundle for GitHub Pages."""
import html
import json
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / ".cache/pages/three-rs-wasm"


def examples(site):
    """threejs.org's URLs: examples/#<id> for the gallery and examples/<id>.html
    for each example. The pages are web/gallery's, with a <base> there, so their
    relative URLs and the Wasm's asset paths stay those of web/gallery/."""
    gallery = site / "web/gallery"
    out = site / "examples"
    out.mkdir()
    base = '<base href="../web/gallery/">'
    index = (gallery / "index.html").read_text()
    index = index.replace("<head>", f'<head>\n\t\t{base}<meta name="three-rs-layout" content="examples">', 1)
    (out / "index.html").write_text(index)
    page = (gallery / "example.html").read_text()
    catalog = json.loads((gallery / "catalog.json").read_text())
    for entry in catalog["examples"]:
        meta = f'{base}<meta name="three-rs-example" content="{html.escape(entry["id"])}">'
        (out / f'{entry["id"]}.html').write_text(page.replace("<head>", "<head>" + meta, 1))


def main():
    bundle = [ROOT / "web/pkg" / name for name in
              ("three_rs_wasm.js", "three_rs_wasm_bg.wasm")]
    if not all(path.is_file() for path in bundle):
        raise SystemExit("Build web/pkg with wasm-pack --release first")
    # wasm-bindgen's inline_js modules, which three_rs_wasm.js imports.
    bundle += sorted(path for path in (ROOT / "web/pkg/snippets").rglob("*") if path.is_file())
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
    for source in [ROOT / name for name in files if name] + bundle:
        target = SITE / source.relative_to(ROOT)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
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
