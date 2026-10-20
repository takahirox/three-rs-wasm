"""Assign the gallery's functions and data to the modules that alone reach them.

The input is the whole gallery linked with `--emit-relocs`: its relocations give
every reference exactly ( calls, function pointers, addresses of data in code and
in data ). The roots of a module are its own functions ( their paths start with
`three_rs_gallery::browser::<module>` ); the shell's roots are the exports. The
shell reaches what it reaches without entering a module's roots; a module owns
what only its roots reach outside the shell. Everything else is shared and stays
in the primary module."""
import bisect
import collections
import re
import subprocess
import wasm

MEMORY = {3, 4, 5, 11, 14, 15, 16, 17, 23}
FUNCTION = {0, 1, 2, 12, 18, 19, 24, 26}
PREFIX = '16three_rs_gallery7browser'
# Exports a page calls only for its module's examples ( browser.rs gates them on
# the module's example_ feature ), by export name or prefix.
EXPORTS = {
    'fbx_canonical': 'fbx_loader',
    'vrml_canonical': 'vrml',
    'usd_canonical': 'usdz',
    'rhino3dm_canonical': 'loader_3dm',
    'tsl_transpile': 'tsl_transpiler',
    'bounceworld_': 'ballpool',
    '__wbg_bounceworld_free': 'ballpool',
    'xrapp_': 'xr_app',
    '__wbg_xrapp_free': 'xr_app',
    'browserapp_audio_play': 'tsl_procedural',
    'browserapp_audio_result': 'tsl_procedural',
    'browserapp_audio_spectrum': 'tsl_procedural',
    'offscreen_': 'offscreen',
}


def export_module(name):
    for prefix, module in EXPORTS.items():
        if name.startswith(prefix):
            return module
    return None


def module_of(symbol, modules):
    """The gallery module a v0-mangled function lives in, or None."""
    if not symbol.startswith('_R'):
        return None
    i = 2
    if symbol[i:i + 1] == 'I':
        i += 1
    while i < len(symbol):
        c = symbol[i]
        if c == 'N':
            i += 2
        elif c in 'MX':
            i += 1
            if symbol[i:i + 1] == 's':
                i = symbol.index('_', i) + 1
        elif c == 'C':
            break
        else:
            return None
    m = re.match(r'C(?:s[0-9A-Za-z]*_)?' + PREFIX + r'(?:s[0-9A-Za-z]*_)?(\d+)', symbol[i:])
    if not m:
        return None
    n = int(m[1])
    start = i + m.end()
    name = symbol[start:start + n]
    return name if name in modules else None


def demangle(symbols):
    """Rust v0 symbols demangled ( c++filt: LLVM's on macOS, binutils' on Linux )."""
    out = subprocess.run(['c++filt'], input='\n'.join(symbols), capture_output=True, text=True,
                         check=True).stdout.split('\n')
    plain = dict(zip(symbols, out))
    rust = [s for s in symbols if s.startswith('_R')]
    if rust and sum(plain[s] != s for s in rust) < len(rust) * 0.9:
        raise SystemExit('split: c++filt does not demangle Rust v0 symbols')
    return plain


MENTION = re.compile(r'three_rs_gallery(?:\[[0-9a-f]+\])?::browser::(\w+)')


def assign(path, modules, min_chunk=5000):
    b = open(path, 'rb').read()
    secs = wasm.sections(b)
    nimp = wasm.imported_functions(b, secs)
    bodies = wasm.bodies(b, secs)
    starts = [s for s, _ in bodies]
    segments = wasm.data_segments(b, secs)
    data_base = wasm.section(b, secs, 11)[2]
    names = wasm.function_names(b, secs)
    syms = wasm.symbols(b, secs)
    # Data nodes: defined data symbols, by memory address.
    data = {}
    for k, s in enumerate(syms):
        if s[0] == 1 and s[4]:
            seg, off, size = s[4]
            data[k] = (segments[seg][1] + off, size)
    intervals = sorted((a, a + max(n, 1), k) for k, (a, n) in data.items())
    iv_starts = [x[0] for x in intervals]

    def data_at(addr):
        i = bisect.bisect_right(iv_starts, addr) - 1
        while i >= 0 and intervals[i][0] <= addr:
            if addr < intervals[i][1]:
                return intervals[i][2]
            i -= 1
        return None

    def target(t, idx):
        s = syms[idx]
        if t in FUNCTION and s[0] == 0:
            return ('f', s[2])
        if t in MEMORY and idx in data:
            return ('d', idx)
        return None

    edges = collections.defaultdict(set)
    for t, off, idx, _ in wasm.relocations(b, secs, 'reloc.CODE'):
        node = target(t, idx)
        if node is None:
            continue
        k = bisect.bisect_right(starts, off) - 1
        edges[('f', nimp + k)].add(node)
    # reloc.DATA offsets are relative to the data section's payload.
    seg_offsets = [s[0] for s in segments]
    for t, off, idx, _ in wasm.relocations(b, secs, 'reloc.DATA'):
        node = target(t, idx)
        if node is None:
            continue
        si = bisect.bisect_right(seg_offsets, off) - 1
        addr = segments[si][1] + off - segments[si][0]
        src = data_at(addr)
        if src is not None:
            edges[('d', src)].add(node)
    # Roots.
    unit_roots = collections.defaultdict(set)
    owner = {}
    # Identical functions merge into one with several symbols ( LLVM's
    # MergeFunctions ): a function belongs to a module only when every symbol
    # naming it does.
    aliases = collections.defaultdict(set)
    for idx, nm in names.items():
        aliases[idx].add(nm)
    for s in syms:
        if s[0] == 0 and s[2] is not None and s[3] and not s[1] & 0x10:
            aliases[s[2]].add(s[3])
    # Generic code instantiated for one module's types ( a boxed future of its
    # async block, the drop of its demo ) is that module's too.
    plain = demangle(sorted({nm for nms in aliases.values() for nm in nms}))

    def module_of_alias(nm):
        m = module_of(nm, modules)
        if m:
            return m
        mentioned = {x for x in MENTION.findall(plain.get(nm, '')) if x in modules}
        return mentioned.pop() if len(mentioned) == 1 else None

    for idx, nms in aliases.items():
        ms = {module_of_alias(nm) for nm in nms}
        if len(ms) == 1 and None not in ms:
            (m,) = ms
            unit_roots[m].add(('f', idx))
            owner[('f', idx)] = m
    exports = []
    s = wasm.section(b, secs, 7)
    i = s[2]
    n, i = wasm.leb(b, i)
    for _ in range(n):
        ename, i = wasm.name(b, i)
        kind = b[i]
        idx, i = wasm.leb(b, i + 1)
        if kind != 0 or ename.startswith('__wbindgen_describe'):
            continue
        m = export_module(ename)
        if m in modules:
            unit_roots[m].add(('f', idx))
            owner[('f', idx)] = m
        else:
            exports.append(('f', idx))
    start = wasm.section(b, secs, 8)
    if start:
        exports.append(('f', wasm.leb(b, start[2])[0]))
    # The shell.
    shell = set()
    stack = [x for x in exports if x not in owner]
    while stack:
        x = stack.pop()
        if x in shell:
            continue
        shell.add(x)
        for y in edges.get(x, ()):
            if y not in shell and y not in owner:
                stack.append(y)
    # The modules.
    reached = collections.defaultdict(set)
    for m, roots in unit_roots.items():
        seen = set()
        stack = list(roots)
        while stack:
            x = stack.pop()
            if x in seen or x in shell:
                continue
            seen.add(x)
            stack.extend(edges.get(x, ()))
        for x in seen:
            reached[x].add(m)
    # Parts: each module's own nodes, and the nodes several modules share by the
    # set that reaches them ( a chunk, when it is large enough ); smaller shared
    # sets stay in the primary module.
    def size(x):
        if x[0] == 'f':
            return bodies[x[1] - nimp][1] - bodies[x[1] - nimp][0] if x[1] >= nimp else 0
        a, n = data[x[1]]
        return n if syms[x[1]][4][0] == 0 else 0

    shared = collections.Counter()
    for x, ms in reached.items():
        if len(ms) > 1:
            shared[frozenset(ms)] += size(x)
    chunks = {k: f'chunk{i}' for i, k in enumerate(sorted(
        (k for k, v in shared.items() if v >= min_chunk), key=lambda k: (-shared[k], sorted(k))))}
    parts = collections.defaultdict(lambda: ([], []))
    data_part = {}
    for x, ms in reached.items():
        part = next(iter(ms)) if len(ms) == 1 else chunks.get(frozenset(ms))
        if part is None:
            continue
        if x[0] == 'f':
            if x[1] >= nimp:
                parts[part][0].append(x[1])
        elif syms[x[1]][4][0] == 0 and data[x[1]][1]:  # read-only data only
            data_part[x[1]] = part
    # Merged constants let symbols share bytes: a symbol whose bytes overlap
    # another symbol's with a different owner ( the primary module included )
    # stays in the primary module.
    spans = sorted((a, a + n, k) for k, (a, n) in data.items() if syms[k][4][0] == 0 and n)
    clash = set()
    active = []
    for a, e, k in spans:
        active = [x for x in active if x[1] > a]
        for x in active:
            if data_part.get(x[2]) != data_part.get(k):
                clash.add(x[2])
                clash.add(k)
        active.append((a, e, k))
    for k, part in data_part.items():
        if k not in clash:
            parts[part][1].append(data[k])
    loads = {m: sorted([m] + [c for k, c in chunks.items() if m in k]) for m in modules}
    check(edges, exports, unit_roots, owner, parts, loads, data, names)
    return dict(parts), loads


def check(edges, exports, unit_roots, owner, parts, loads, data, names):
    """From the shell's roots and a module's roots, without entering another
    module's roots, everything reached is in the primary module or in a part the
    module loads."""
    function_part = {('f', f): p for p, (fs, _) in parts.items() for f in fs}
    data_parts = collections.defaultdict(set)
    for p, (_, ivs) in parts.items():
        for iv in ivs:
            data_parts[iv].add(p)
    for m, allowed in loads.items():
        allowed = set(allowed)
        seen = set()
        stack = list(exports) + list(unit_roots[m])
        while stack:
            x = stack.pop()
            if x in seen:
                continue
            seen.add(x)
            if x[0] == 'f':
                p = function_part.get(x)
                missing = p is not None and p not in allowed
            else:
                ps = data_parts.get(data[x[1]], set())
                missing = bool(ps) and not ps & allowed
            if missing:
                raise SystemExit(f'split: {m} reaches {names.get(x[1], x)} outside its parts')
            for y in edges.get(x, ()):
                if y not in owner or owner[y] == m:
                    stack.append(y)
