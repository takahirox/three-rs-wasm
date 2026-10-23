"""Extract the new texture assets from the repository's pinned Three.js archive."""
import hashlib
import json
import tarfile
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2]
REV='148ef33ecb6d2502ff796d4554abd1549c95d519'
OUT=ROOT/'web/examples/assets/material-textures'
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
(OUT/'manifest.json').write_text(json.dumps({'revision':REV,'files':manifest},indent=2)+'\n')
