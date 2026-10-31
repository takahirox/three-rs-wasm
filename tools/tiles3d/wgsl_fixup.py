#!/usr/bin/env python3
"""Rewrites naga's WGSL where WGSL is stricter than GLSL, keeping GLSL's
semantics: a call that passes two pointers into the same variable
( GLSL `inout result.a, inout result.b` ) violates WGSL's alias analysis, so
those arguments become temporaries copied in before and out after the call,
which is GLSL's copy-in / copy-out of inout parameters.

Usage: wgsl_fixup.py < naga.wgsl > fixed.wgsl
"""
import re
import sys

CALL = re.compile(r'^(\s*)((?:let\s+\w+\s*=\s*)?)(\w+)\((.*)\);\s*$')


def split_arguments(text):
    out, depth, start = [], 0, 0
    for i, c in enumerate(text):
        if c in '([':
            depth += 1
        elif c in ')]':
            depth -= 1
        elif c == ',' and depth == 0:
            out.append(text[start:i].strip())
            start = i + 1
    out.append(text[start:].strip())
    return out


def main():
    lines = sys.stdin.read().split('\n')
    out = []
    temporary = 0
    for line in lines:
        m = CALL.match(line)
        if not m:
            out.append(line)
            continue
        indent, binding, name, args = m.groups()
        arguments = split_arguments(args)
        pointers = {}
        for k, a in enumerate(arguments):
            p = re.fullmatch(r'\(&(?:\(\*)?(\w+)\)?([.\[].*)?\)', a)
            if p:
                pointers.setdefault(p.group(1), []).append(k)
        aliased = [k for ks in pointers.values() if len(ks) > 1 for k in ks]
        if not aliased:
            out.append(line)
            continue
        copies = []
        for k in aliased:
            place = arguments[k][2:-1]
            t = f'pt_inout_{temporary}'
            temporary += 1
            out.append(f'{indent}var {t} = {place};')
            copies.append(f'{indent}{place} = {t};')
            arguments[k] = f'(&{t})'
        out.append(f'{indent}{binding}{name}({", ".join(arguments)});')
        out.extend(copies)
    sys.stdout.write('\n'.join(out))


if __name__ == '__main__':
    main()
