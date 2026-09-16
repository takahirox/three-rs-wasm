# Three.js r186 compatibility baseline

This directory defines the frozen MVP compatibility reference for `three-rs-wasm`.

## Files

- `api.json` — exact upstream revision and Core export inventory scope.
- `capabilities.json` — Rust mapping and default classification for each exported Core type, plus member overrides and must-support items.
- `unsupported.json` — frozen allowlist for WebGL-specific, JavaScript-specific, and explicitly deferred items.

The complete member inventory is generated deterministically from the pinned Three.js source rather than edited by hand:

```bash
python3 tools/compat/generate_core_inventory.py --output /tmp/three-rs-wasm-members.json
python3 tools/compat/check_manifest.py --members /tmp/three-rs-wasm-members.json
```

or simply:

```bash
make compat-check
```

The generator reads the exact `upstream_commit` recorded in `api.json`. An offline checkout of the same revision can be supplied with `--source-root`.

## Classification rules

Every generated Core type/member receives exactly one status:

- `implemented-equivalent`
- `implemented-adapted`
- `unsupported-webgl-specific`
- `unsupported-js-specific`
- `deferred`

A member inherits the containing type's default classification unless an exact override exists in `capabilities.json`.

Difficulty is never a valid exclusion reason. Autonomous implementation agents must not add entries to `unsupported.json` merely to make checks pass.

## Changing the baseline

Changes to `api.json`, `capabilities.json`, or `unsupported.json` are design changes. They are not ordinary implementation fixes and should be reviewed explicitly.
