#!/usr/bin/env python3
"""Generate the frozen Three.js r186 Core API member inventory.

No third-party dependencies. By default this reads the exact upstream commit
recorded in compat/three-r186/api.json. For offline/reproducible use, pass
--source-root /path/to/three.js.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.request
from dataclasses import dataclass
from pathlib import Path
from typing import Iterable

ROOT = Path(__file__).resolve().parents[2]
BASELINE_DIR = ROOT / "compat" / "three-r186"
API_PATH = BASELINE_DIR / "api.json"
CAPABILITIES_PATH = BASELINE_DIR / "capabilities.json"
DEFAULT_OUTPUT = BASELINE_DIR / "members.json"
IDENT = r"[A-Za-z_$][A-Za-z0-9_$]*"


@dataclass(frozen=True)
class Method:
    name: str
    kind: str
    body: str


def load_json(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def fetch_text(path: str, commit: str, source_root: Path | None) -> str:
    if source_root is not None:
        p = source_root / path
        if not p.is_file():
            raise FileNotFoundError(f"Missing upstream source: {p}")
        return p.read_text(encoding="utf-8")
    url = f"https://raw.githubusercontent.com/mrdoob/three.js/{commit}/{path}"
    req = urllib.request.Request(url, headers={"User-Agent": "three-rs-wasm-compat-generator/1"})
    with urllib.request.urlopen(req, timeout=30) as response:
        return response.read().decode("utf-8")


def find_matching_brace(source: str, open_index: int) -> int:
    depth = 0
    i = open_index
    state = "code"
    quote = ""
    while i < len(source):
        c = source[i]
        n = source[i + 1] if i + 1 < len(source) else ""
        if state == "code":
            if c == "/" and n == "/":
                state = "line_comment"; i += 2; continue
            if c == "/" and n == "*":
                state = "block_comment"; i += 2; continue
            if c in ("'", '"', "`"):
                state = "string"; quote = c; i += 1; continue
            if c == "{": depth += 1
            elif c == "}":
                depth -= 1
                if depth == 0: return i
            i += 1
        elif state == "line_comment":
            if c == "\n": state = "code"
            i += 1
        elif state == "block_comment":
            if c == "*" and n == "/": state = "code"; i += 2
            else: i += 1
        else:
            if c == "\\": i += 2
            elif c == quote: state = "code"; quote = ""; i += 1
            else: i += 1
    raise ValueError("Unmatched brace")


def class_blocks(source: str) -> dict[str, tuple[str | None, str]]:
    out = {}
    pattern = re.compile(rf"\bclass\s+({IDENT})(?:\s+extends\s+({IDENT}))?\s*\{{")
    for m in pattern.finditer(source):
        start = source.find("{", m.start())
        end = find_matching_brace(source, start)
        out[m.group(1)] = (m.group(2), source[start + 1:end])
    return out


def top_level_methods(body: str) -> list[Method]:
    methods = []
    i = 0
    depth = 0
    state = "code"
    quote = ""
    sig = re.compile(rf"(?:(get|set|async|static)\s+)?({IDENT})\s*\([^)]*\)\s*\{{")
    while i < len(body):
        c = body[i]
        n = body[i + 1] if i + 1 < len(body) else ""
        if state == "code":
            if c == "/" and n == "/": state = "line_comment"; i += 2; continue
            if c == "/" and n == "*": state = "block_comment"; i += 2; continue
            if c in ("'", '"', "`"): state = "string"; quote = c; i += 1; continue
            if c == "{": depth += 1; i += 1; continue
            if c == "}": depth -= 1; i += 1; continue
            if depth == 0:
                m = sig.match(body, i)
                if m:
                    prefix, name = m.group(1), m.group(2)
                    start = body.find("{", m.start(), m.end())
                    end = find_matching_brace(body, start)
                    kind = "constructor" if name == "constructor" else "method"
                    if prefix == "get": kind = "getter"
                    elif prefix == "set": kind = "setter"
                    elif prefix == "static": kind = "static-method"
                    methods.append(Method(name, kind, body[start + 1:end]))
                    i = end + 1; continue
            i += 1
        elif state == "line_comment":
            if c == "\n": state = "code"
            i += 1
        elif state == "block_comment":
            if c == "*" and n == "/": state = "code"; i += 2
            else: i += 1
        else:
            if c == "\\": i += 2
            elif c == quote: state = "code"; quote = ""; i += 1
            else: i += 1
    return methods


def constructor_properties(class_name: str, source: str, methods: Iterable[Method]) -> set[str]:
    props = set()
    ctor = next((m for m in methods if m.kind == "constructor"), None)
    if ctor:
        props.update(re.findall(rf"\bthis\.({IDENT})\s*=", ctor.body))
        props.update(re.findall(rf"Object\.defineProperty\(\s*this\s*,\s*['\"]({IDENT})['\"]", ctor.body))
    # Covers Object3D.defineProperties fields and other documented immutable fields.
    props.update(re.findall(rf"@name\s+{re.escape(class_name)}#({IDENT})\b", source))
    return {p for p in props if not p.startswith("_")}


def static_properties(class_name: str, source: str) -> set[str]:
    return {x for x in re.findall(rf"\b{re.escape(class_name)}\.({IDENT})\s*=", source) if not x.startswith("_")}


def make_item(api: str, owner: str, kind: str, source: str, spec: dict,
              overrides: dict[str, dict], member: str | None = None,
              extends: str | None = None) -> dict:
    override = overrides.get(api)
    item = {
        "api": api,
        "owner": owner,
        "kind": kind,
        "source": source,
        "status": override["status"] if override else spec["default_status"],
        "rust_mapping": spec.get("rust_mapping"),
    }
    if member is not None: item["member"] = member
    if extends is not None: item["extends"] = extends
    if override: item["reason"] = override["reason"]
    elif spec.get("notes"): item["notes"] = spec["notes"]
    return item


def generate(source_root: Path | None) -> dict:
    api = load_json(API_PATH)
    caps = load_json(CAPABILITIES_PATH)
    commit = api["baseline"]["upstream_commit"]
    type_map = {x["three_type"]: x for x in caps["types"]}
    overrides = {x["api"]: x for x in caps.get("member_overrides", [])}
    items, seen = [], set()

    def add(item: dict) -> None:
        key = (item["api"], item["kind"])
        if key not in seen:
            seen.add(key); items.append(item)

    for source_path, exported_types in api["source_exports"].items():
        text = fetch_text(source_path, commit, source_root)
        blocks = class_blocks(text)
        for class_name in exported_types:
            if class_name not in blocks: raise RuntimeError(f"Class {class_name} not found in {source_path}")
            if class_name not in type_map: raise RuntimeError(f"No classification for {class_name}")
            extends, body = blocks[class_name]
            spec = type_map[class_name]
            add(make_item(class_name, class_name, "type", source_path, spec, overrides, extends=extends))
            methods = top_level_methods(body)
            for method in methods:
                add(make_item(f"{class_name}.{method.name}", class_name, method.kind, source_path, spec, overrides, member=method.name))
            for prop in sorted(constructor_properties(class_name, text, methods)):
                add(make_item(f"{class_name}.{prop}", class_name, "property", source_path, spec, overrides, member=prop))
            for prop in sorted(static_properties(class_name, text)):
                add(make_item(f"{class_name}.{prop}", class_name, "static-property", source_path, spec, overrides, member=prop))

    extracted = {x["api"] for x in items}
    unresolved = [x for x in overrides if x not in extracted and ".constructor.options." not in x]
    if unresolved: raise RuntimeError(f"Overrides not found in inventory: {unresolved}")

    items.sort(key=lambda x: (x["owner"], x["api"], x["kind"]))
    counts = {}
    for item in items: counts[item["status"]] = counts.get(item["status"], 0) + 1
    return {
        "schema_version": 1,
        "generated_from": {"release": api["baseline"]["release"], "upstream_commit": commit, "core_path": api["baseline"]["core_path"]},
        "generator": "tools/compat/generate_core_inventory.py",
        "item_count": len(items),
        "status_counts": dict(sorted(counts.items())),
        "items": items,
    }


def canonical(data: dict) -> str:
    return json.dumps(data, indent=2, ensure_ascii=False) + "\n"


def main() -> int:
    p = argparse.ArgumentParser()
    p.add_argument("--source-root", type=Path, help="three.js checkout at the pinned commit")
    p.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    p.add_argument("--check", action="store_true")
    args = p.parse_args()
    generated = canonical(generate(args.source_root))
    output = args.output if args.output.is_absolute() else ROOT / args.output
    if args.check:
        if not output.is_file() or output.read_text(encoding="utf-8") != generated:
            print("ERROR: Core inventory missing or stale. Run: python3 tools/compat/generate_core_inventory.py", file=sys.stderr)
            return 1
        print(f"Core API inventory: PASS ({json.loads(generated)['item_count']} items)")
        return 0
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(generated, encoding="utf-8")
    print(f"Wrote {output} ({json.loads(generated)['item_count']} items)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
