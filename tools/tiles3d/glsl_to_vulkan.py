#!/usr/bin/env python3
"""Rewrites a WebGL2 program three.js compiles ( a vertex and a fragment
shader as the reference page passes them to shaderSource ) into a pair of
Vulkan-style GLSL 450 shaders that naga's GLSL frontend reads, without
changing any expression:

- the sources are preprocessed as WebGL does ( defines, #if, macros ), and
  precision qualifiers, #version and #extension go;
- the non-opaque uniforms of both stages move into one std140 block
  ( binding 0 ) that both stages declare identically, after every struct
  definition ( hoisted to the top in order );
- a bool uniform becomes an int read as `( name != 0 )`;
- every sampler uniform and parameter splits into a texture and a sampler
  ( `samplerType( name_t, name_s )` in texture calls ), each texture with a
  binding of its own after the block, declared in both stages;
- attributes take the locations in ATTRIBUTES, varyings the locations of the
  vertex stage's declaration order, matched by name;
- with --flip the vertex stage negates gl_Position.y, so that render targets
  hold rows as WebGL stores them ( row 0 is gl_FragCoord.y 0.5 ), and every
  vertex stage maps WebGL's clip z to WebGPU's ( the same window depth );
- a local named like a WGSL builtin function is renamed, isnan / isinf test
  the bits, and pow is WebGL's exp2( y * log2( x ) ).

Usage: glsl_to_vulkan.py [--flip] vertex.glsl fragment.glsl out.vert out.frag
"""
import re
import subprocess
import sys

OPAQUE = {
    'sampler2D': 'texture2D',
    'usampler2D': 'utexture2D',
    'isampler2D': 'itexture2D',
    'sampler3D': 'texture3D',
    'sampler2DArray': 'texture2DArray',
    'samplerCube': 'textureCube',
}
TEXTURE_FUNCTIONS = {
    'texture', 'texelFetch', 'textureLod', 'textureSize', 'textureGrad',
    'textureOffset', 'texelFetchOffset', 'textureGather', 'textureLodOffset',
}
PADDED = {
    'float': ('vec4', '.x'), 'vec2': ('vec4', '.xy'), 'vec3': ('vec4', '.xyz'),
    'int': ('ivec4', '.x'), 'ivec2': ('ivec4', '.xy'), 'ivec3': ('ivec4', '.xyz'),
}
ATTRIBUTES = {'position': 0, 'normal': 1, 'uv': 2}
BUILTINS = {
    'sign', 'step', 'mix', 'min', 'max', 'clamp', 'length', 'distance', 'normalize',
    'reflect', 'refract', 'dot', 'cross', 'floor', 'ceil', 'fract', 'exp', 'exp2', 'log',
    'log2', 'sqrt', 'abs', 'select', 'round', 'trunc', 'degrees', 'radians', 'smoothstep',
    'saturate', 'transpose', 'determinant', 'modf', 'frexp', 'ldexp', 'fma', 'sin', 'cos',
    'tan', 'asin', 'acos', 'atan', 'atan2', 'all', 'any',
}


def preprocess(source, vertex_stage):
    es3 = re.search(r'^\s*#version\s+300\s+es', source, re.M) is not None
    if not es3:
        # GLSL ES 1.00 ( the output pass ): three's WebGL2 prefix aliases.
        aliases = ['#define attribute in', '#define varying out'] if vertex_stage else [
            '#define varying in', 'layout(location = 0) out vec4 pc_fragColor;',
            '#define gl_FragColor pc_fragColor']
        source = '\n'.join(aliases + ['#define texture2D texture']) + '\n' + source
    lines = [line for line in source.split('\n')
             if not re.match(r'\s*#\s*(version|extension)\b', line)
             and not re.match(r'\s*precision\s+\w+\s+\w+\s*;\s*$', line)]
    out = subprocess.run(['clang', '-E', '-P', '-x', 'c', '-w', '-U__VERSION__', f'-D__VERSION__={300 if es3 else 100}', '-'], input='\n'.join(lines),
                         capture_output=True, text=True, check=True).stdout
    return re.sub(r'\b(highp|mediump|lowp)\b', '', out)


def top_level(text):
    """( start, end ) of each top-level statement or definition."""
    out, depth, start = [], 0, 0
    for m in re.finditer(r'[{};]', text):
        c = m.group(0)
        if c == '{':
            depth += 1
        elif c == '}':
            depth -= 1
            if depth == 0:
                # A struct definition ends with `};`, a function with `}`.
                end = m.end()
                rest = re.match(r'\s*;', text[end:])
                if rest:
                    continue
                out.append((start, end))
                start = end
        elif c == ';' and depth == 0:
            out.append((start, m.end()))
            start = m.end()
    return out


class Stage:
    def __init__(self, source, vertex_stage):
        self.text = preprocess(source, vertex_stage)
        self.structs = []  # ( name, text )
        self.uniforms = []  # ( type, name, array )
        self.opaque = {}  # name -> sampler type
        self.inputs = []  # ( qualifiers, type, name )
        self.outputs = []
        body = []
        defined = set()
        for start, end in top_level(self.text):
            item = self.text[start:end]
            s = item.strip()
            # A prototype after its definition ( naga reads it as a redefinition ).
            m = re.fullmatch(r'\w+\s+(\w+)\s*\([^;{}]*\)\s*;', s, re.S)
            if m and m.group(1) in defined:
                continue
            m = re.match(r'\w+\s+(\w+)\s*\(', s)
            if m and s.endswith('}'):
                defined.add(m.group(1))
            m = re.match(r'struct\s+(\w+)\s*\{', s)
            if m and s.endswith(';'):
                self.structs.append((m.group(1), s))
                continue
            m = re.fullmatch(r'uniform\s+(\w+)\s+(\w+)\s*(\[[^\]]*\])?\s*;', s)
            if m:
                kind, name, array = m.groups()
                if kind in OPAQUE:
                    self.opaque[name] = kind
                elif all(u[1] != name for u in self.uniforms):
                    self.uniforms.append((kind, name, array or ''))
                continue
            m = re.fullmatch(r'((?:flat\s+|smooth\s+|centroid\s+)*)(in|out)\s+(\w+)\s+(\w+)\s*(\[[^\]]*\])?\s*;', s)
            if m:
                (self.inputs if m.group(2) == 'in' else self.outputs).append(
                    (m.group(1), m.group(3), m.group(4), m.group(5) or ''))
                continue
            body.append(item)
        self.body = ''.join(body)


def convert(vertex, fragment, flip):
    vs, fs = Stage(vertex, True), Stage(fragment, False)
    structs = []
    for name, text in vs.structs + fs.structs:
        if all(n != name for n, _ in structs):
            structs.append((name, text))
    uniforms = []
    for u in vs.uniforms + fs.uniforms:
        if all(o[1] != u[1] for o in uniforms):
            uniforms.append(u)
    bools = [u[1] for u in uniforms if u[0] == 'bool']
    opaque = dict(vs.opaque)
    for name, kind in fs.opaque.items():
        opaque.setdefault(name, kind)
    # Struct types the block needs, with what they need.
    # std140 arrays of scalars and short vectors have a 16-byte stride: the
    # block holds vec4s ( the same layout ) and the stages private copies.
    padded = {n: (t, a) for t, n, a in uniforms if a and t in PADDED}
    # A struct uniform's leaves are block members ( WebGL's uniform names,
    # `.` and `[k]` as `_` ), copied into a private struct at entry.
    struct_map = dict(structs)
    flattened = {n: t for t, n, a in uniforms if t in struct_map and not a}
    lines = []
    struct_copies = []
    for t, n, a in uniforms:
        if n in padded:
            lines.append(f'\t{PADDED[t][0]} pt_u_{n}{a};')
        elif n in flattened:
            for leaf_type, path in leaves(t, n, struct_map):
                member = 'pt_s_' + re.sub(r'\W+', '_', path).strip('_')
                lines.append(f'\t{leaf_type} {member};')
                struct_copies.append(f'{path} = {member}; ')
        else:
            lines.append(f'\t{"int" if t == "bool" else t} {n}{a};')
    block = '\n'.join(lines)
    bindings = []
    binding = 1
    for name, kind in opaque.items():
        bindings.append(f'layout(set = 0, binding = {binding}) uniform {OPAQUE[kind]} {name}_t;')
        bindings.append(f'layout(set = 0, binding = {binding + 1}) uniform sampler {name}_s;')
        binding += 2
    varyings = {}
    location = 0
    for _, t, name, a in vs.outputs:
        varyings[name] = location
        location += locations(t, a, dict(structs))
    helpers = [
        *[f'{t} pt_pow( {t} x, {t} y ) {{ return exp2( y * log2( x ) ); }}' for t in ('float', 'vec2', 'vec3', 'vec4')],
        *[f'{t} pt_modf( {t} x, out {t} i ) {{ i = trunc( x ); return x - i; }}' for t in ('float', 'vec2', 'vec3', 'vec4')],
        'bool pt_isnan( float v ) { uint b = floatBitsToUint( v ) & 0x7fffffffu; return b > 0x7f800000u; }',
        'bool pt_isinf( float v ) { uint b = floatBitsToUint( v ) & 0x7fffffffu; return b == 0x7f800000u; }',
        *[f'bvec{n} pt_isnan( vec{n} v ) {{ uvec{n} b = floatBitsToUint( v ) & 0x7fffffffu; return greaterThan( b, uvec{n}( 0x7f800000u ) ); }}' for n in (2, 3, 4)],
        *[f'bvec{n} pt_isinf( vec{n} v ) {{ uvec{n} b = floatBitsToUint( v ) & 0x7fffffffu; return equal( b, uvec{n}( 0x7f800000u ) ); }}' for n in (2, 3, 4)],
    ]

    def emit(stage, vertex_stage):
        decls = []
        for q, t, n, a in stage.inputs:
            if vertex_stage:
                if n not in ATTRIBUTES:
                    raise SystemExit(f'unknown attribute {n}')
                decls.append(f'layout(location = {ATTRIBUTES[n]}) in {t} {n}{a};')
            else:
                if n not in varyings:
                    raise SystemExit(f'varying {n} missing from the vertex stage')
                decls.append(f'layout(location = {varyings[n]}) {q}in {t} {n}{a};')
        for q, t, n, a in stage.outputs:
            if vertex_stage:
                decls.append(f'layout(location = {varyings[n]}) {q}out {t} {n}{a};')
            else:
                raise SystemExit(f'fragment output {n} without a location')
        # WebGPU needs an output per target component: a vec2 / vec3 output
        # becomes a private variable written out as a vec4 with alpha 1.
        widened = []
        if not vertex_stage:
            def widen(m):
                count = int(m.group(4)) if m.group(4) else 0
                widened.append((m.group(3), m.group(2), count))
                array = f'[{count}]' if count else ''
                return f'layout(location = {m.group(1)}) out vec4 pt_out_{m.group(3)}{array};\n{m.group(2)} {m.group(3)}{array};'
            body = re.sub(r'layout\s*\(\s*location\s*=\s*(\d+)\s*\)\s*out\s+(vec2|vec3)\s+(\w+)\s*(?:\[\s*(\d+)\s*\])?\s*;', widen, stage.body)
        else:
            body = stage.body
        prologue = ''.join(
            f'{n}[{k}] = pt_u_{n}[{k}]{PADDED[t][1]}; ' for n, (t, a) in padded.items() for k in range(int(a.strip('[]'))))
        prologue += ''.join(struct_copies)
        globals_ = [f'{t} {n}{a};' for n, (t, a) in padded.items()] + [f'{t} {n};' for n, t in flattened.items()]
        epilogue = 'gl_Position.y = - gl_Position.y; ' if vertex_stage and flip else ''
        if vertex_stage:
            # WebGL's clip z spans [ -w, w ], WebGPU's [ 0, w ]: the same window depth.
            epilogue += 'gl_Position.z = ( gl_Position.z + gl_Position.w ) * 0.5; '
        for n, t, count in widened:
            fill = '0.0, 1.0' if t == 'vec2' else '1.0'
            for index in ([f'[{k}]' for k in range(count)] if count else ['']):
                epilogue += f'pt_out_{n}{index} = vec4( {n}{index}, {fill} ); '
        if prologue or epilogue:
            body = re.sub(r'\bvoid\s+main\s*\(\s*(void)?\s*\)', 'void pt_main()', body)
            body += f'\nvoid main() {{ {prologue}pt_main(); {epilogue} }}\n'
        body = rewrite(body, opaque, bools)
        if vertex_stage:
            # A vertex stage samples level 0 ( no derivatives ).
            body = explicit_lod(body)
        header = [
            '#version 450',
            *[text for _, text in structs],
            'layout(std140, set = 0, binding = 0) uniform Uniforms {',
            block if block else '\tfloat pt_unused;',
            '};',
            *bindings,
            *globals_,
            *helpers,
            *decls,
        ]
        return '\n'.join(header) + '\n' + body

    return emit(vs, True), emit(fs, False)


def rewrite(body, opaque, bools):
    body = re.sub(r'\bconst\s+in\s+', 'in ', body)
    for name in bools:
        body = re.sub(rf'\b{name}\b', f'( {name} != 0 )', body)
    body = re.sub(r'\bisnan\s*\(', 'pt_isnan(', body)
    body = re.sub(r'\bisinf\s*\(', 'pt_isinf(', body)
    body = re.sub(r'\bpow\s*\(', 'pt_pow(', body)
    body = re.sub(r'\bmodf\s*\(', 'pt_modf(', body)
    # naga cannot store a uniform block's struct array into a local array.
    body = re.sub(r'\b(\w+)\s+(\w+)\[(\d+)\]\s*=\s*([\w.]+)\s*;',
                  lambda m: f'{m.group(1)} {m.group(2)}[{m.group(3)}]; ' + ' '.join(
                      f'{m.group(2)}[{k}] = {m.group(4)}[{k}];' for k in range(int(m.group(3)))), body)
    body = re.sub(r'(?<![.\w])(' + '|'.join(sorted(BUILTINS)) + r')\b(?!\s*\()', r'pt_\1', body)
    return rewrite_opaque(body, opaque)


def members(text):
    body = text[text.index('{') + 1:text.rindex('}')]
    return [(m.group(1), m.group(2), m.group(3) or '')
            for m in re.finditer(r'(\w+)\s+(\w+)\s*(\[[^\]]*\])?\s*;', body)]


def leaves(t, path, structs):
    """( type, GLSL path ) of every scalar, vector or matrix in a struct value."""
    if t not in structs:
        return [(t, path)]
    out = []
    for mt, mn, ma in members(structs[t]):
        if ma:
            for k in range(int(ma.strip('[]'))):
                out += leaves(mt, f'{path}.{mn}[{k}]', structs)
        else:
            out += leaves(mt, f'{path}.{mn}', structs)
    return out


def locations(t, a, structs):
    """The interface locations a varying of type t ( array a ) takes."""
    count = int(a.strip('[]')) if a else 1
    if t in structs:
        body = structs[t][structs[t].index('{') + 1:structs[t].rindex('}')]
        size = sum(locations(m.group(1), m.group(3) or '', structs)
                   for m in re.finditer(r'(\w+)\s+(\w+)\s*(\[[^\]]*\])?\s*;', body))
    else:
        size = int(t[3]) if t.startswith('mat') else 1
    return count * size


def explicit_lod(text):
    out = []
    i = 0
    for m in re.finditer(r'\btexture\s*\(', text):
        if m.start() < i:
            continue
        depth = 0
        for j in range(m.end() - 1, len(text)):
            if text[j] == '(':
                depth += 1
            elif text[j] == ')':
                depth -= 1
                if depth == 0:
                    break
        out.append(text[i:m.start()] + 'textureLod(' + explicit_lod(text[m.end():j]) + ', 0.0)')
        i = j + 1
    out.append(text[i:])
    return ''.join(out)


def tokens(text):
    return re.finditer(r'[A-Za-z_]\w*|\d[\w.]*|//[^\n]*|/\*.*?\*/|"[^"]*"|\S|\s+', text, re.S)


def rewrite_opaque(text, globals_):
    """Splits opaque parameters and uses, scope by scope."""
    out = []
    toks = [m.group(0) for m in tokens(text)]
    scope = dict(globals_)
    pending = {}
    depth = 0
    function_scope = None
    calls = []
    i = 0
    n = len(toks)

    def next_significant(k):
        while k < n and toks[k].isspace():
            k += 1
        return k

    def prev_significant(k):
        k -= 1
        while k >= 0 and toks[k].isspace():
            k -= 1
        return k

    while i < n:
        t = toks[i]
        if t in OPAQUE:
            j = next_significant(i + 1)
            name = toks[j]
            k = next_significant(j + 1)
            if toks[k] in (',', ')'):
                out.append(f'{OPAQUE[t]} {name}_t, sampler {name}_s')
                pending[name] = t
                i = j + 1
                continue
        if t == '(':
            p = prev_significant(i)
            calls.append(toks[p] if p >= 0 else '')
        elif t == ')':
            if calls:
                calls.pop()
        elif t == '{':
            depth += 1
            if depth == 1 and pending:
                function_scope = dict(pending)
                scope.update(function_scope)
            if depth == 1:
                pending.clear()
        elif t == '}':
            depth -= 1
            if depth == 0 and function_scope is not None:
                for name in function_scope:
                    if name in globals_:
                        scope[name] = globals_[name]
                    else:
                        scope.pop(name, None)
                function_scope = None
        elif t == ';' and depth == 0:
            pending.clear()
        if re.match(r'[A-Za-z_]\w*$', t) and t in scope:
            p = prev_significant(i)
            if p < 0 or toks[p] != '.':
                kind = scope[t]
                if calls and calls[-1] in TEXTURE_FUNCTIONS and toks[p] == '(':
                    out.append(f'{kind}( {t}_t, {t}_s )')
                else:
                    out.append(f'{t}_t, {t}_s')
                i += 1
                continue
        out.append(t)
        i += 1
    return ''.join(out)


def main():
    args = sys.argv[1:]
    flip = '--flip' in args
    args = [a for a in args if a != '--flip']
    vertex, fragment = convert(open(args[0]).read(), open(args[1]).read(), flip)
    open(args[2], 'w').write(vertex)
    open(args[3], 'w').write(fragment)


if __name__ == '__main__':
    main()
