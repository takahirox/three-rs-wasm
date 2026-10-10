# TSL transpiler

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

`webgpu_tsl_transpiler` ( gallery example 483 ) converts GLSL into TSL or
WGSL as the user types. `src/transpiler/` ports the classes the page uses
from `examples/jsm/transpiler/`:

| r186 | Rust |
| --- | --- |
| `GLSLDecoder` ( preprocessor, tokenizer, parser ) | `preprocess.rs`, `decoder.rs` |
| `AST.js` | `ast.rs`: the nodes in an arena, with their parents and linker records |
| `Linker` | `linker.rs` |
| `TSLEncoder` | `tsl_encoder.rs` |
| `WGSLEncoder` | `wgsl_encoder.rs` |
| `Transpiler.parse( source )` | `transpiler::transpile( source, Encoder )` |

## The port

The port reproduces what the JS implementation's output depends on:

- **Preprocessor.** `#define`, `#undef`, `#ifdef`, `#ifndef`, `#if`, `#elif`,
  `#else` and `#endif` work as in the original, along with function-like
  macros and line continuations. The decoder evaluates `#if` conditions
  with `Function`; a small evaluator gives the same strict-mode result,
  including false for syntax errors and leading-zero literals.
- **Regular expressions and string methods.** The JS semantics of the
  decoder's regular expressions are written out: `\w`, `\s` and `\b`, `.`
  ( which does not match line terminators ), `trim()`, `slice()` and the
  `$&` patterns of `replace()`. Error positions count columns in UTF-16
  units.
- **Encoder output.**
  - The `import` list in TSL holds only names that r186's `three/tsl`
    exports. `tsl_names.txt` lists the 682 exports, taken from the pinned
    build.
  - Where the original prints `undefined` or `null` in a template, the port
    prints the same text.
- **Error messages.**
  - The decoder's and encoders' own errors match.
  - So do the V8 TypeErrors that malformed input reaches, for example
    `Cannot read properties of undefined (reading 'str')` for
    `float x = ;`.
  - A `null` the decoder leaves in a statement list is a `Null` node, so the
    linker fails on it as the original does.

The gallery page builds the page's two Monaco editors ( 0.55.1, from the
same CDN ) and its Encoder and Decoder selections. It transpiles again a
second after the last edit. Only the transpiling runs in Rust
( `tsl_transpile` in the Wasm module ); the page draws nothing, so it does
not need WebGPU.

## Comparison

`tests/browser/tsl-transpiler.spec.js` runs the r186 transpiler and the
port in one page, over both encoders.

The 287 inputs are:
- the page's GLSL;
- every string in r186's `ShaderChunk`;
- every vertex and fragment shader in `examples/jsm/shaders/`;
- the cases in `tests/browser/transpiler-cases.json`. These cover textures,
  qualifiers, overloads, switch, loops, macros and malformed statements.

The test also makes 5,500 seeded mutations of those inputs. They drop,
repeat and swap tokens, and insert characters such as preprocessor
fragments, `\r`, ` `, quotes and non-ASCII text.

All 11,574 outputs match character for character. Over 6,000 of them are
error messages. The gallery test checks the page's own flow:
- the initial TSL;
- switching to WGSL, which also changes the result editor's language;
- editing the source, which transpiles again after a second;
- an error message.

Limits:
- **Object property names.** The decoder's property bag is a plain JS
  object. A GLSL name that matches an `Object.prototype` member, such as
  `constructor`, takes another path in the original; the port does not
  reproduce it.
- **`#if` conditions.** In a condition, `/` in operand position would start
  a regular expression literal in JS; the port treats it as a syntax error.
