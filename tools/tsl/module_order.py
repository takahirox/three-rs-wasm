"""The ES module evaluation order of three r186's Three.WebGPU.js: a depth-first
post-order over each module's relative imports and re-exports in source order.
Writes .cache/tslsrc/module_order.txt ( paths relative to src/ )."""
import os, re, sys

SRC = os.path.join(os.path.dirname(__file__), '../../.cache/three-r186/src')
IMPORT = re.compile(r'''(?:^|\n)\s*(?:import|export)\s+(?:[^'";]*?\s+from\s+)?['"]([^'"]+)['"]''')

def deps(path):
    text = open(os.path.join(SRC, path)).read()
    return [os.path.normpath(os.path.join(os.path.dirname(path), m.group(1)))
            for m in IMPORT.finditer(text) if m.group(1).startswith('.')]

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
    out = os.path.join(os.path.dirname(__file__), '../../.cache/tslsrc/module_order.txt')
    open(out, 'w').write('\n'.join(order))
    print(len(order), 'modules')

main()
