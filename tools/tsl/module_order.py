"""The ES module evaluation order of three r187's Three.WebGPU.js: a depth-first
post-order over each module's relative imports and re-exports in source order.
Writes .cache/tslsrc/module_order.txt ( paths relative to src/ ), and the
ported addons' order after it, as the pages import them, to
.cache/tslsrc/addon_order.txt ( ../examples/jsm/… )."""
import os, re, sys

SRC = os.path.join(os.path.dirname(__file__), '../../.cache/three-r187/src')
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
    print(core, 'modules,', len(order) - core, 'addons')

main()
