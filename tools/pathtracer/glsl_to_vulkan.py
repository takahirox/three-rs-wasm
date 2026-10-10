#!/usr/bin/env python3
"""Rewrites the WebGL2 GLSL three.js compiles for three-gpu-pathtracer's
PhysicalPathTracingMaterial into Vulkan-style GLSL 450 that naga's GLSL
frontend reads, without changing any expression:

- the version, precision and output header becomes GLSL 450's;
- every non-opaque uniform moves into one std140 block ( binding 0 ), and the
  uniform structs ( BVH, LightsInfo, EquirectHdrInfo, PhysicalCamera ) are
  flattened into `struct_member` names;
- every sampler uniform and parameter splits into a texture and a sampler:
  a texture call's first argument becomes `samplerType( name_t, name_s )`, and
  an argument to a user function becomes `name_t, name_s`;
- global textures that three samples with the same sampler state share one
  sampler ( SHARED ), since WebGPU allows 16 samplers per stage;
- a local variable named like a WGSL builtin function is renamed;
- pow is WebGL's exp2( y * log2( x ) ) ( NaN for a negative base, as the
  reference's GPU computes it ) rather than the backend's exact pow.

Usage: glsl_to_vulkan.py captured.glsl > vulkan.glsl
"""
import re
import sys

OPAQUE = {
    'sampler2D': 'texture2D',
    'usampler2D': 'utexture2D',
    'isampler2D': 'itexture2D',
    'sampler2DArray': 'texture2DArray',
}
TEXTURE_FUNCTIONS = {
    'texture', 'texelFetch', 'textureLod', 'textureSize', 'textureGrad',
    'textureOffset', 'texelFetchOffset', 'textureGather', 'textureLodOffset',
}
# Uniform structs: ( type, name ) -> members ( type, name ).
STRUCTS = {
    'BVH': [('usampler2D', 'index'), ('sampler2D', 'position'),
            ('sampler2D', 'bvhBounds'), ('usampler2D', 'bvhContents')],
    'LightsInfo': [('sampler2D', 'tex'), ('uint', 'count')],
    'EquirectHdrInfo': [('sampler2D', 'marginalWeights'), ('sampler2D', 'conditionalWeights'),
                        ('sampler2D', 'map'), ('float', 'totalSum')],
    'PhysicalCamera': [('float', 'focusDistance'), ('float', 'anamorphicRatio'),
                       ('float', 'bokehSize'), ('int', 'apertureBlades'),
                       ('float', 'apertureRotation')],
}
# Global textures and the shared sampler of their state ( PhysicalPathTracingMaterial ).
SHARED = {
    **{name: 'pt_nearest' for name in (
        'stratifiedTexture', 'stratifiedOffsetTexture', 'sobolTexture', 'lights_tex',
        'attributesArray', 'materialIndexAttribute', 'materials', 'bvh_index',
        'bvh_position', 'bvh_bvhBounds', 'bvh_bvhContents')},
    **{name: 'pt_linear' for name in ('iesProfiles', 'envMapInfo_marginalWeights', 'envMapInfo_conditionalWeights')},
    **{name: 'pt_linear_repeat_u' for name in ('backgroundMap', 'envMapInfo_map')},
    'textures': 'pt_linear_repeat',
}


def main():
    source = open(sys.argv[1]).read()
    # The header up to the material's defines.
    body = source[source.index('#define HIGH_PRECISION'):] if '#define HIGH_PRECISION' in source else source[source.index('#define SHADER_TYPE'):]
    body = re.sub(r'^\s*precision\s+\w+\s+\w+\s*;\s*$', '', body, flags=re.M)
    # Varyings take locations in their order of declaration.
    varyings = iter(range(64))
    body = re.sub(r'^(\s*)varying\s+(\w+)\s+(\w+)\s*;', lambda m: f'{m.group(1)}layout(location = {next(varyings)}) in {m.group(2)} {m.group(3)};', body, flags=re.M)
    # Uniform declarations, struct ones flattened.
    block = []
    opaque_globals = {}

    def declare(kind, name):
        if kind in OPAQUE:
            opaque_globals[name] = kind
        else:
            block.append(f'\t{kind} {name};')
        return ''

    def uniform(match):
        kind, name = match.group(1), match.group(2)
        if kind in STRUCTS:
            return '\n'.join(declare(t, f'{name}_{m}') for t, m in STRUCTS[kind])
        return declare(kind, name)

    body = re.sub(r'^\s*uniform\s+(\w+)\s+(\w+)\s*;', uniform, body, flags=re.M)
    # The flattened structs' definitions ( sampler members are not types in GLSL 450 ).
    for kind in STRUCTS:
        body = re.sub(rf'\bstruct\s+{kind}\s*\{{[^}}]*\}}\s*;', '', body)
    flattened = {}
    # Member references of the flattened struct uniforms.
    for decl in re.findall(r'uniform\s+(\w+)\s+(\w+)\s*;', source):
        kind, name = decl
        if kind in STRUCTS:
            for _, member in STRUCTS[kind]:
                flattened[f'{name}.{member}'] = f'{name}_{member}'
    for old, new in flattened.items():
        body = re.sub(rf'\b{re.escape(old)}\b', new, body)
    # The bool uniform has no std140 equivalent in WGSL: an int read as a bool.
    block = [line.replace('\tbool ', '\tint ') for line in block]
    body = re.sub(r'\bisOrthographic\b', 'bool( isOrthographic )', body)
    # naga reads `in` parameters but not `const in`; both are read-only copies.
    body = re.sub(r'\bconst\s+in\s+', 'in ', body)
    # texture2D is a texture type in GLSL 450: the WebGL alias's calls are texture's.
    body = re.sub(r'\btexture2D\s*\(', 'texture(', body)
    # WGSL's builtin functions cannot be shadowed: `float sign = ...`.
    body = re.sub(r'\bsign\b(?!\s*\()', 'pt_sign', body)
    body = rewrite_opaque(body, opaque_globals)
    # Opaque globals: a texture each and a sampler each or shared, after the block.
    bindings = []
    binding = 1
    samplers = []
    for name, kind in opaque_globals.items():
        bindings.append(f'layout(set = 0, binding = {binding}) uniform {OPAQUE[kind]} {name}_t;')
        binding += 1
        sampler = SHARED.get(name, f'{name}_s')
        if sampler not in samplers:
            samplers.append(sampler)
            bindings.append(f'layout(set = 0, binding = {binding}) uniform sampler {sampler};')
            binding += 1
    # WGSL has no isnan / isinf and may assume finite math: test the bits.
    body = re.sub(r'\bisnan\s*\(', 'pt_isnan(', body)
    body = re.sub(r'\bisinf\s*\(', 'pt_isinf(', body)
    body = re.sub(r'\bpow\s*\(', 'pt_pow(', body)
    helpers = [
        'bvec3 pt_isnan( vec3 v ) { uvec3 b = floatBitsToUint( v ) & 0x7fffffffu; return greaterThan( b, uvec3( 0x7f800000u ) ); }',
        'bvec3 pt_isinf( vec3 v ) { uvec3 b = floatBitsToUint( v ) & 0x7fffffffu; return equal( b, uvec3( 0x7f800000u ) ); }',
        *[f'{t} pt_pow( {t} x, {t} y ) {{ return exp2( y * log2( x ) ); }}' for t in ('float', 'vec2', 'vec3', 'vec4')],
    ]
    header = [
        '#version 450',
        'layout(location = 0) out vec4 pc_fragColor;',
        '#define gl_FragColor pc_fragColor',
        'layout(std140, set = 0, binding = 0) uniform Uniforms {',
        *block,
        '};',
        *bindings,
        *helpers,
    ]
    sys.stdout.write('\n'.join(header) + '\n' + body)


def tokens(text):
    return re.finditer(r'[A-Za-z_]\w*|\d[\w.]*|//[^\n]*|/\*.*?\*/|"[^"]*"|\S|\s+', text, re.S)


def rewrite_opaque(text, globals_):
    """Splits opaque declarations and uses, scope by scope."""
    out = []
    toks = [m.group(0) for m in tokens(text)]
    scope = dict(globals_)
    depth = 0
    function_scope = None
    # ( function name, argument index ) stacks for the calls being read.
    calls = []
    i = 0
    n = len(toks)

    def next_significant(k):
        while k < n and (toks[k].isspace() or toks[k].startswith('//') or toks[k].startswith('/*')):
            k += 1
        return k

    def prev_significant(k):
        k -= 1
        while k >= 0 and (toks[k].isspace() or toks[k].startswith('//') or toks[k].startswith('/*')):
            k -= 1
        return k

    while i < n:
        t = toks[i]
        # A parameter or local declaration of an opaque type.
        if t in OPAQUE:
            j = next_significant(i + 1)
            name = toks[j]
            k = next_significant(j + 1)
            if toks[k] in (',', ')'):
                # A function parameter.
                out.append(f'{OPAQUE[t]} {name}_t, sampler {name}_s')
                pending_params[name] = t
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
            if depth == 1 and pending_params:
                function_scope = dict(pending_params)
                scope.update(function_scope)
            pending_params.clear() if depth == 1 else None
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
            pending_params.clear()
        if re.match(r'[A-Za-z_]\w*$', t) and t in scope:
            p = prev_significant(i)
            if p < 0 or toks[p] != '.':
                kind = scope[t]
                local = function_scope is not None and t in function_scope
                sampler = f'{t}_s' if local else SHARED.get(t, f'{t}_s')
                if calls and calls[-1] in TEXTURE_FUNCTIONS and toks[prev_significant(i)] == '(':
                    out.append(f'{kind}( {t}_t, {sampler} )')
                else:
                    out.append(f'{t}_t, {sampler}')
                i += 1
                continue
        out.append(t)
        i += 1
    return ''.join(out)


pending_params = {}

if __name__ == '__main__':
    main()
