"""Package tracked demo assets and the release Wasm bundle for GitHub Pages."""
from pathlib import Path
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[1]
SITE = ROOT / ".cache/pages/three-rs-wasm"


def main():
    bundle = [ROOT / "web/pkg" / name for name in
              ("three_rs_wasm.js", "three_rs_wasm_bg.wasm")]
    if not all(path.is_file() for path in bundle):
        raise SystemExit("Build web/pkg with wasm-pack --release first")
    if SITE.exists():
        shutil.rmtree(SITE)
    files = subprocess.check_output(
        ["git", "ls-files", "-z", "web", "LICENSE", "LICENSE-THREE", "LICENSE-MATERIALX"], cwd=ROOT
    ).decode().split("\0")
    for source in [ROOT / name for name in files if name] + bundle:
        target = SITE / source.relative_to(ROOT)
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(source, target)
    (SITE / "index.html").write_text(
        '<!doctype html><html lang="en"><meta charset="utf-8">'
        '<title>three-rs-wasm examples</title>'
        '<meta http-equiv="refresh" content="0;url=web/gallery/">'
        '<a href="web/gallery/">Open the examples gallery</a></html>\n'
    )
    print(SITE.relative_to(ROOT))


if __name__ == "__main__":
    main()
