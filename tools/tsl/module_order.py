"""The ES module evaluation order of three r186's Three.WebGPU.js: a depth-first
post-order over each module's relative imports and re-exports in source order.
Writes .cache/tslsrc/module_order.txt ( paths relative to src/ )."""
import os, re, sys

SRC = os.path.join(os.path.dirname(__file__), '../../.cache/three-r186/src')
IMPORT = re.compile(r'''(?:^|\n)\s*(?:import|export)\s+(?:[^'";]*?\s+from\s+)?['"]([^'"]+)['"]''')

OVERRIDES = os.path.join(os.path.dirname(__file__), 'overrides')
BARE = {'three/webgpu': 'Three.WebGPU.js', 'three': 'Three.WebGPU.js'}

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
    out = os.path.join(os.path.dirname(__file__), '../../.cache/tslsrc/module_order.txt')
    open(out, 'w').write('\n'.join(order))
    print(len(order), 'modules')

main()
