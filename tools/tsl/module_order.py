"""The ES module evaluation order of three r187's Three.WebGPU.js: a depth-first
post-order over each module's relative imports and re-exports in source order.
Writes .cache/tslsrc/module_order.txt ( paths relative to src/ ), and the
ported addons' order after it, as the pages import them, to
.cache/tslsrc/addon_order.txt ( ../examples/jsm/… ).

The examples load the build instead: build/three.webgpu.js imports
three.core.js, then three.tsl.js, and Rollup orders each chunk's modules its
own way. .cache/tslsrc/bundle_order.txt is the same modules in the order the
build evaluates them: each by where its declarations are in the build ( a
top-level declaration line the build has once ), and the modules without
code there ( the re-exporting barrels, the GLSL strings, the port's own )
just before their first importer, after their own imports. The order shows
where a counter names things: a TSL Fn's layout is named fnN by creation."""
import collections, os, re, sys

SRC = os.path.join(os.path.dirname(__file__), '../../.cache/three-r187/src')
BUILD = os.path.join(os.path.dirname(__file__), '../../.cache/three-r187/build')
CHUNKS = ['three.core.js', 'three.tsl.js', 'three.webgpu.js']
IMPORT = re.compile(r'''(?:^|\n)\s*(?:import|export)\s+(?:[^'";]*?\s+from\s+)?['"]([^'"]+)['"]''')

OVERRIDES = os.path.join(os.path.dirname(__file__), 'overrides')
BARE = {'three/webgpu': 'Three.WebGPU.js', 'three': 'Three.WebGPU.js', 'three/tsl': 'Three.TSL.js'}
# The addons the ported pages import, in their import order.
ADDONS = ['../examples/jsm/loaders/MaterialXLoader.js', '../examples/jsm/loaders/materialx/MaterialXInterfaceValidation.js']

def source(path):
    if path.endswith('.original.js'):
        return open(os.path.join(SRC, path.replace('.original.js', '.js'))).read()
    if os.path.exists(os.path.join(OVERRIDES, path)):
        return open(os.path.join(OVERRIDES, path)).read()
    return open(os.path.join(SRC, path)).read()

def deps(path):
    out = []
    for m in IMPORT.finditer(source(path)):
        spec = m.group(1)
        if spec.startswith('.'):
            out.append(os.path.normpath(os.path.join(os.path.dirname(path), spec)))
        elif spec in BARE:
            out.append(BARE[spec])
        elif spec.startswith('three/addons/'):
            out.append('../examples/jsm/' + spec[len('three/addons/'):])
    return out

def main():
    sys.setrecursionlimit(10000)
    order, seen = [], set()
    def visit(path):
        if path in seen:
            return
        seen.add(path)
        for d in deps(path):
            visit(d)
        order.append(path)
    visit('Three.WebGPU.js')
    # The editor imports three/tsl after three/webgpu.
    visit('Three.TSL.js')
    core = len(order)
    for path in ADDONS:
        visit(path)
    out = os.path.join(os.path.dirname(__file__), '../../.cache/tslsrc/')
    open(out + 'module_order.txt', 'w').write('\n'.join(order[:core]))
    open(out + 'addon_order.txt', 'w').write('\n'.join(order[core:]))
    bundled = bundle_order(order[:core])
    open(out + 'bundle_order.txt', 'w').write('\n'.join(bundled))
    print(core, 'modules,', len(order) - core, 'addons')

DECLARATION = re.compile(r'(?:export\s+)?(?:default\s+)?(?:async\s+)?(?:class|function\*?|const|let|var)\s+([\w$]+)')

def bundle_order(modules):
    """`modules` ( in module_order.txt's order ) in the build's."""
    # The build's top-level lines ( a renamed binding's $n suffix dropped ) and
    # declared names, by ( chunk, line ).
    lines, names = collections.defaultdict(list), collections.defaultdict(list)
    for c, chunk in enumerate(CHUNKS):
        for n, line in enumerate(open(os.path.join(BUILD, chunk)).read().split('\n')):
            if line and not line[0].isspace():
                lines[re.sub(r'\$\d+\b', '', line)].append((c, n))
                d = DECLARATION.match(line)
                if d:
                    names[re.sub(r'\$\d+$', '', d.group(1))].append((c, n))
    at = {}
    for path in modules:
        js = path.replace('.original.js', '.js')
        if not os.path.exists(os.path.join(SRC, js)):
            continue
        exact, named = [], []
        for line in open(os.path.join(SRC, js)).read().split('\n'):
            d = DECLARATION.match(line)
            if d:
                hits = lines.get(re.sub(r'^export\s+(default\s+)?', '', line), [])
                if len(hits) == 1:
                    exact.append(hits[0])
                named.append(names.get(d.group(1), []))
        if exact:
            at[path] = min(exact)
        else:
            # A declaration the build changed: by a name it declares once.
            once = [hits[0] for hits in named if len(hits) == 1]
            if once:
                at[path] = once[0]
    known = set(modules)
    imports = {path: [d for d in deps(path) if d in known] for path in modules}
    order, done = [], set()
    def emit(path):
        if path in done:
            return
        done.add(path)
        for d in imports[path]:
            if d not in at:
                emit(d)
        order.append(path)
    for path in sorted(at, key=at.get):
        emit(path)
    for path in modules:
        emit(path)
    position = {path: i for i, path in enumerate(order)}
    late = [(path, d) for path in order for d in imports[path] if position[d] > position[path]]
    if late:
        sys.exit(f'bundle order: imported after their importers: {late[:5]}')
    return order

main()
