#!/usr/bin/env python3
"""Which browser package each ported example loads ( web/gallery/packages.json ).

Every package is the library, the gallery's shell and shared scenes ( the core
package ) and a group of example modules, features of gallery/. One package
per module would repeat the core about 220 times, past the published site's
size; modules are grouped instead, those sharing modules together, into
about GROUPS packages of similar size ( a module's source size predicts its
code size ). An example's module is the feature on its `create` arm in
gallery/src/browser/expanded.rs; examples the core dispatches load `core`.

  packages.py            write packages.json
  packages.py --check    fail when packages.json is stale
  packages.py --list     print the packages
  packages.py --features PACKAGE   print a package's features
"""
import json, re, sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / 'web/gallery/packages.json'
BROWSER = ROOT / 'gallery/src/browser'
GROUPS = 40
# Examples the gallery's shell starts itself, with the feature their page uses.
SHELL = {483: 'tsl_transpiler', 498: 'xr_app', 499: 'xr_app', 500: 'xr_app'}


def numbers(pattern):
    found = set()
    for part in pattern.split('|'):
        part = part.strip()
        if m := re.fullmatch(r'(\d+)\.\.=(\d+)', part):
            found.update(range(int(m[1]), int(m[2]) + 1))
        elif m := re.fullmatch(r'(\d+)\.\.(\d+)', part):
            found.update(range(int(m[1]), int(m[2])))
        elif part.isdigit():
            found.add(int(part))
        else:
            return None
    return found


def arms():
    source = (BROWSER / 'expanded.rs').read_text()
    start = source.index('    pub async fn create(')
    body = source[start:source.index('\n    pub fn update(', start)]
    feature = None
    for line in body.split('\n'):
        if m := re.fullmatch(r' {12}#\[cfg\(feature = "example_(\w+)"\)\]', line):
            feature = m[1]
            continue
        if re.match(r' {12}\S', line) and '=>' in line:
            found = numbers(line.strip().split('=>')[0])
            if found is not None:
                yield found, feature or 'core'
        if re.match(r' {12}\S', line):
            feature = None


def features():
    """gallery/Cargo.toml's module features and the modules each enables."""
    text = (ROOT / 'gallery/Cargo.toml').read_text()
    table = text[text.index('[features]'):text.index('[dependencies]')]
    modules = {m[1]: re.findall(r'"(\w+)"', m[2])
               for m in re.finditer(r'^(\w+) = \[(.*?)\]$', table, re.M)
               if m[1] not in ('default', 'all') and not m[1].startswith('example_')}
    # Library features ( basisu ) are not modules.
    return {k: [d for d in v if d in modules] for k, v in modules.items() if (BROWSER / f'{k}.rs').exists()}


def closure(feature, table, seen=None):
    seen = set() if seen is None else seen
    if feature not in seen:
        seen.add(feature)
        for d in table.get(feature, []):
            closure(d, table, seen)
    return seen


def weight(module):
    paths = [BROWSER / f'{module}.rs', *(BROWSER / module).rglob('*')]
    return sum(p.stat().st_size for p in paths if p.is_file())


def group(units, table):
    """Greedy packing: the largest unit first, each into the group it adds the
    least to while the group stays under the capacity, the smallest capacity
    that makes at most GROUPS groups."""
    sizes = {}
    def size(modules):
        return sum(sizes.setdefault(m, weight(m)) for m in modules)
    closures = {u: closure(u, table) for u in units}
    union = set().union(*closures.values())
    capacity = size(union) / GROUPS
    while len(groups := pack(units, closures, size, capacity)) > GROUPS:
        capacity *= 1.05
    return {g[0]: sorted(g[1]) for g in groups}


def pack(units, closures, size, capacity):
    groups = []  # [name, units, modules, size]
    for unit in sorted(units, key=lambda u: (-size(closures[u]), u)):
        best = None
        for g in groups:
            added = size(closures[unit] - g[2])
            if g[3] + added <= capacity and (best is None or added < best[0]):
                best = (added, g)
        if best is None:
            groups.append([unit, [unit], set(closures[unit]), size(closures[unit])])
        else:
            best[1][1].append(unit)
            best[1][2] |= closures[unit]
            best[1][3] += best[0]
    return groups


def mapping():
    by_number = dict(SHELL)
    for found, package in arms():
        for n in found:
            by_number.setdefault(n, package)
    catalog = json.loads((ROOT / 'web/gallery/catalog.json').read_text())
    modules = {e['id']: by_number.get(e['port']['example'], 'core')
               for e in catalog['examples'] if e.get('port')}
    table = features()
    packages = group(sorted(set(modules.values()) - {'core'}), table)
    package_of = {u: name for name, units in packages.items() for u in units}
    packages['core'] = []
    return {'packages': dict(sorted(packages.items())),
            'examples': {k: package_of.get(v, 'core') for k, v in sorted(modules.items())}}


def main():
    data = mapping()
    text = json.dumps(data, indent=1) + '\n'
    if '--list' in sys.argv:
        print('\n'.join(data['packages']))
    elif '--features' in sys.argv:
        # The package's examples' dispatch features ( which enable their modules ).
        print(','.join(f'example_{u}' for u in data['packages'][sys.argv[sys.argv.index('--features') + 1]]))
    elif '--check' in sys.argv:
        if not OUT.exists() or OUT.read_text() != text:
            sys.exit(f'{OUT.relative_to(ROOT)} is stale; run tools/gallery/packages.py')
    else:
        OUT.write_text(text)
        print(f"{len(data['examples'])} examples, {len(data['packages'])} packages")


if __name__ == '__main__':
    main()
