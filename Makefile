.PHONY: compat-generate compat-check check-mvp

compat-generate:
	python3 tools/compat/generate_core_inventory.py

compat-check:
	python3 tools/compat/generate_core_inventory.py --output /tmp/three-rs-wasm-members.json
	python3 tools/compat/check_manifest.py --members /tmp/three-rs-wasm-members.json

check-mvp:
	bash scripts/check-mvp
