#!/usr/bin/env python3
"""Validate the frozen r186 compatibility manifest and generated inventory."""
from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
BASE = ROOT / "compat" / "three-r186"
ALLOWED = {
    "implemented-equivalent",
    "implemented-adapted",
    "unsupported-webgl-specific",
    "unsupported-js-specific",
    "deferred",
}


def load(path: Path) -> dict:
    if not path.is_file():
        raise RuntimeError(f"missing {path}")
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--members", type=Path, default=BASE / "members.json")
    args = parser.parse_args()

    members_path = args.members if args.members.is_absolute() else ROOT / args.members
    errors: list[str] = []
    try:
        api = load(BASE / "api.json")
        caps = load(BASE / "capabilities.json")
        unsupported = load(BASE / "unsupported.json")
        members = load(members_path)
    except Exception as exc:
        print(f"Compatibility manifest: FAIL: {exc}", file=sys.stderr)
        return 1

    baselines = [api["baseline"], caps["baseline"], unsupported["baseline"]]
    for key in ("release", "npm", "git_tag", "upstream_commit", "core_path"):
        values = {x.get(key) for x in baselines}
        if len(values) != 1:
            errors.append(f"baseline mismatch for {key}: {sorted(values)}")

    if not all(doc.get("frozen") is True for doc in (api, caps, unsupported)):
        errors.append("api.json, capabilities.json, and unsupported.json must all be frozen")

    api_types = set(api["inventory_scope"]["exported_types"])
    cap_types = {x["three_type"] for x in caps["types"]}
    if api_types != cap_types:
        errors.append(
            f"type sets differ: api-only={sorted(api_types-cap_types)}, "
            f"capabilities-only={sorted(cap_types-api_types)}"
        )

    for spec in caps["types"]:
        status = spec["default_status"]
        if status not in ALLOWED:
            errors.append(f"invalid type status: {spec['three_type']}={status}")
        if status.startswith("implemented-") and not spec.get("rust_mapping"):
            errors.append(f"implemented type lacks rust_mapping: {spec['three_type']}")

    overrides = {x["api"]: x for x in caps.get("member_overrides", [])}
    unsupported_entries = {x["api"]: x for x in unsupported["entries"]}
    for name, item in overrides.items():
        if item["status"] not in ALLOWED:
            errors.append(f"invalid member override status: {name}={item['status']}")
        if item["status"] in {"unsupported-webgl-specific", "unsupported-js-specific", "deferred"}:
            u = unsupported_entries.get(name)
            if u is None:
                errors.append(f"override missing from unsupported.json: {name}")
            elif u["category"] != item["status"]:
                errors.append(f"override category mismatch for {name}")

    rules = unsupported.get("rules", {})
    if rules.get("autonomous_agent_may_add_entries") is not False:
        errors.append("unsupported allowlist must forbid autonomous additions")
    if rules.get("difficulty_is_never_a_valid_reason") is not True:
        errors.append("unsupported allowlist must reject difficulty as a reason")

    items = members.get("items", [])
    specifications = {spec['three_type']: spec for spec in caps['types']}
    keys = set()
    for item in items:
        key = (item.get('api'), item.get('kind'))
        if key in keys:
            errors.append(f'duplicate inventory item: {key}')
        keys.add(key)
        owner = specifications.get(item.get('owner'))
        if owner is None:
            errors.append(f'unknown inventory owner: {item.get("owner")}')
            continue
        expected = overrides.get(item.get('api'), {}).get('status', owner['default_status'])
        if item.get('status') != expected:
            errors.append(f'inventory classification differs from frozen manifest: {item.get("api")}')
    for name in api_types:
        if (name, 'type') not in keys:
            errors.append(f'exported type missing from inventory: {name}')
    item_apis = {item["api"] for item in items}
    for item in items:
        if item.get("status") not in ALLOWED:
            errors.append(f"unclassified inventory item: {item.get('api')}={item.get('status')}")

    for requirement in caps.get("must_support", []):
        name = requirement["api"]
        matching = [x for x in items if x["api"] == name]
        if not matching:
            errors.append(f"must_support API missing from inventory: {name}")
        elif any(x["status"] in {"unsupported-webgl-specific", "unsupported-js-specific", "deferred"} for x in matching):
            errors.append(f"must_support API is excluded/deferred: {name}")

    for name in unsupported_entries:
        if name not in item_apis and ".constructor.options." not in name:
            errors.append(f"unsupported entry does not resolve to inventory: {name}")

    if members.get("item_count") != len(items):
        errors.append("generated inventory item_count is inconsistent")
    if members.get("generated_from", {}).get("upstream_commit") != api["baseline"]["upstream_commit"]:
        errors.append("generated inventory was created from a different upstream commit")

    if errors:
        print("Compatibility manifest: FAIL", file=sys.stderr)
        for error in errors:
            print(f"  - {error}", file=sys.stderr)
        return 1

    counts: dict[str, int] = {}
    for item in items:
        counts[item["status"]] = counts.get(item["status"], 0) + 1
    print("Compatibility manifest: PASS")
    print(f"Core API accounting: 100% ({len(items)} items)")
    for status, count in sorted(counts.items()):
        print(f"  {status}: {count}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
