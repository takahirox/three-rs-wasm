"""Package the site for GitHub Pages: web/, as three.js lays its repository
out ( build/ the library, examples/ the pages, files/ their chrome ), with the
release Wasm bundle and the example pages the build writes."""
import json
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
WEB = ROOT / "web"
SITE = ROOT / ".cache/pages/three-rs-wasm"


def main():
    # The gallery's Wasm ( scripts/build-gallery ): the glue and its inline_js
    # modules, the primary module, the parts and their listing.
    build = WEB / "build"
    required = [build / name for name in ("three_rs_wasm.js", "three_rs_wasm_bg.wasm", "parts.json")]
    if not all(path.is_file() for path in required):
        raise SystemExit("Build web/build with scripts/build-gallery first")
    # webgpu_tsl_editor's own Wasm: the three-tsl crate ( three r187 transpiled to Rust ).
    if not all((build / "tsl" / name).is_file() for name in ("three_tsl.js", "three_tsl_bg.wasm")):
        raise SystemExit("Build web/build/tsl with wasm-pack ( tsl/, --profile web ) first")
    bundle = sorted(path for path in build.rglob("*") if path.is_file())
    # Every example's page, which the build writes with its parts.
    catalog = json.loads((WEB / "examples/catalog.json").read_text())
    pages = [WEB / "examples" / f"{entry['id']}.html" for entry in catalog["examples"]]
    if not all(page.is_file() for page in pages):
        raise SystemExit("Write the example pages with scripts/build-gallery first")
    if SITE.exists():
        shutil.rmtree(SITE)
    files = subprocess.check_output(["git", "ls-files", "-z", "web"], cwd=ROOT).decode().split("\0")
    # three.js's own example assets load from its repository at the listing's
    # commit ( tools/gallery/upstream_assets.py, web/examples/upstream.js ).
    upstream = json.loads((WEB / "examples/upstream-assets.json").read_text())
    upstream["base"] = f"https://cdn.jsdelivr.net/gh/mrdoob/three.js@{upstream['commit']}/"
    for source in [ROOT / name for name in files if name and name.removeprefix("web/") not in upstream["files"]] + bundle + pages:
        target = SITE / source.relative_to(WEB)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    for name in ("LICENSE", "LICENSE-THREE", "LICENSE-MATERIALX", "LICENSE-RAPIER"):
        shutil.copyfile(ROOT / name, SITE / name)
    (SITE / "examples/upstream-assets.json").write_text(json.dumps(upstream, indent=1, ensure_ascii=False) + "\n")
    # The pages' parts of the listing load from the CDN too.
    for page in pages:
        target = SITE / page.relative_to(WEB)
        target.write_text(target.read_text().replace(
            '<script type="application/json" id="upstream-assets">{"base":null,',
            '<script type="application/json" id="upstream-assets">{"base":' + json.dumps(upstream["base"]) + ',', 1))
    print(SITE.relative_to(ROOT))


if __name__ == "__main__":
    main()
