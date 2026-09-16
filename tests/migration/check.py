import json
import subprocess
from pathlib import Path

manifest=json.loads(Path('migration/manifest.json').read_text())
assert manifest['frozen'] is True
assert len(manifest['pairs'])>=2
for pair in manifest['pairs']:
    for key in ('reference','rust','test'):
        assert Path(pair[key]).is_file(),(pair['name'],key)
subprocess.run(['npx','--no-install','playwright','test','migration.spec.js'],check=True)
