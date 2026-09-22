"""Extract the new texture assets from the repository's pinned Three.js archive."""
import hashlib
import json
import subprocess
import tarfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
REV='148ef33ecb6d2502ff796d4554abd1549c95d519'
OUT=ROOT/'web/gallery/assets/material-textures'
OUT.mkdir(parents=True,exist_ok=True)
manifest=[]
with tarfile.open(ROOT/f'.cache/three-{REV}.tar.gz') as tar:
    for name in ['textures/carbon/Carbon.png','textures/crate.gif','textures/758px-Canestra_di_frutta_(Caravaggio).jpg']:
        member=next(m for m in tar.getmembers() if m.name.endswith('/examples/'+name))
        data=tar.extractfile(member).read()
        path=ROOT/'.cache/three-r186/examples'/name
        path.parent.mkdir(parents=True,exist_ok=True)
        path.write_bytes(data)
        (OUT/Path(name).name).write_bytes(data)
        manifest.append({'file':Path(name).name,'source':'examples/'+name,'sha256':hashlib.sha256(data).hexdigest()})
subprocess.run(['node',str(ROOT/'tools/gallery/prepare-materials.mjs')],check=True)
manifest.append({'file':'paper-models.json','source':'examples/webgpu_materials_arrays.html','sha256':hashlib.sha256((OUT/'paper-models.json').read_bytes()).hexdigest()})
manifest.append({'file':'manual-mips.rgba','source':'examples/webgpu_materials_texture_manualmipmap.html','sha256':hashlib.sha256((OUT/'manual-mips.rgba').read_bytes()).hexdigest()})
(OUT/'manifest.json').write_text(json.dumps({'revision':REV,'files':manifest},indent=2)+'\n')
