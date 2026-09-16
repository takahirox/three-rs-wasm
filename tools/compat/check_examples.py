#!/usr/bin/env python3
"""Validate the frozen application inventory; browser assertions run separately."""
import json
from pathlib import Path

required=set('object-transforms animation input perspective-camera box-geometry basic-material raycasting repeated-render scene-hierarchy multiple-meshes orthographic-camera sphere-geometry standard-material image-texture ambient-light directional-light point-light lines points buffer-geometry attribute-updates indexed-geometry groups draw-ranges plane-geometry object-add-remove resource-recreate'.split())
manifest=json.loads(Path('examples/manifest.json').read_text())
assert manifest['frozen'] is True
covered=set()
names=set()
for app in manifest['applications']:
    assert app['id'] not in names,'duplicate application id'
    names.add(app['id'])
    for key in ('rust','test'):
        assert Path(app[key]).is_file(),(app['id'],key)
    covered.update(app['covers'])
assert covered==required,{'missing':sorted(required-covered),'unexpected':sorted(covered-required)}
print(f'Frozen web applications: PASS ({len(names)} applications, {len(covered)} capabilities)')
