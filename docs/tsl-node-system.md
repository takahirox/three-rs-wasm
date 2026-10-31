# TSL node system port

`webgpu_tsl_editor` runs any TSL the user writes. It shows the WGSL or GLSL
three.js r187 generates for it and renders a preview. Reproducing it means
porting three's node system together with what that system depends on. The
port is the `three-tsl` crate (`tsl/`). It covers:

- `src/nodes`, the WebGPU and WebGL fallback node builders;
- `materials/nodes` and `renderers/common/nodes`;
- the renderer and the core classes;
- every module of `Three.WebGPU.js`: 592 in total.

It is accepted only when every corpus program matches its golden character
for character. A subset was rejected.

## Design

The JavaScript is dynamic, and the output depends on that:

- a node's children are its own properties in creation order;
- `generate.length` selects how a node is built;
- uniforms sort by node id, so module evaluation order matters;
- cache keys include `String( fn )` of function-valued material properties;
- TSL's `Fn` writes through `Reflect.set` with a Proxy receiver;
- user TSL receives nodes as chainable Proxies;
- V8's TypeError texts reach the editor and the console.

`tsl/src/rt` is therefore a JavaScript object model in Rust:

- values, and objects with prototype chains;
- data and accessor properties, in JavaScript's key order with symbol keys
  apart;
- functions with arity and source text, classes as constructor plus
  prototype, and constructible plain functions;
- Proxy, RegExp, Promise, byte-backed typed arrays, Map, Set and WeakMap;
- JSON and the globals the sources use;
- V8's error messages.

`tools/tsl/transpile.mjs` reads each ES module with acorn and emits Rust over
that model, into `tsl/src/three/<snake path>.rs`. The generated code handles:

- module bindings, and closures over shared cells, including per-iteration
  `for ( let … )` bindings;
- parameter scopes, and classes with getters, statics and `super`;
- try/catch through completion values, labeled loops and switch
  fallthrough;
- destructuring, spread and generators;
- V8's call printer for "… is not a function".

`tools/tsl/gen_mods.py` runs the modules' `init()` in the ES evaluation
order of `Three.WebGPU.js` (`tools/tsl/module_order.py`). Node ids and the
other counters therefore advance as in the original. Regenerate with:

```
python3 tools/tsl/gen_constants.py
node tools/tsl/transpile.mjs $( cat .cache/tslsrc/module_order.txt )
python3 tools/tsl/gen_mods.py
```

`tools/tsl/overrides/` replaces two modules. The originals stay transpiled
beside them as `*.original.js`:

- **`renderers/webgpu/WebGPUBackend.js`** runs r187's backend unchanged on
  `HeadlessGPU.js`, a device and canvas context that record nothing. They
  report the reference adapter's features, limits, WGSL language features
  and flag namespaces (`tests/tsl/device.json`, Chrome on macOS). Their
  methods throw Chrome's WebIDL messages where the browser throws
  synchronously (`createBuffer`, `dispatchWorkgroups`), so a frame aborts
  where the original's does. When the page provides a real `GPUDevice`, the
  backend renders through it instead (the editor's preview).
- **`renderers/webgl-fallback/WebGLBackend.js`** keeps r187's extension and
  parameter queries on a headless context with the reference context's
  answers, or on the browser's WebGL 2 context in the editor. It issues no
  GL work: the editor only reads the GLSL text from that renderer.

User code stays JavaScript, run as the editor runs it: an `AsyncFunction`
with `THREE`. `rt::interop` hands Rust objects to JavaScript as Proxies over
handles, and JavaScript values to Rust as `Val::Js`. Both sides keep their
identities.

## The gallery page

`web/examples/tsl-editor.js` builds the original page around the crate's own
Wasm (`web/build/tsl`, built with `--profile web`). It uses:

- Monaco and lil-gui from the CDNs the original uses;
- the gallery's import map, which points `three/tsl` at
  `web/examples/tsl-module.js` (the crate's `Three.TSL.js` namespace);
- the original script, with `THREE` from the crate.

The runtime is synchronous. So the page requests the device as
WebGPUBackend.init does, and makes its `createRenderPipelineAsync` and
`createComputePipelineAsync` return their result at once (a settled
thenable). The preview animates on the browser's frames.

## Verification

- **`tests/tsl/corpus.json`:** 4,217 programs, each chunk one editor session.
  - The editor's default program.
  - Every `three/tsl` export, called as a value or with nine argument
    patterns.
  - 29 hand-written programs: Fn layouts, control flow, structs, arrays,
    uniforms, textures, noise, color, tone mapping, transforms, packing,
    maps and errors.
- **`tests/tsl/golden.json.gz`:** r187's output for each program. It is
  recorded by `node tools/tsl/golden.mjs` from
  `reference/three-js/tsl-corpus.html` and holds:
  - the WGSL with sRGB output, the GLSL ES 3.0 and the WGSL with linear
    output;
  - the thrown message;
  - the console of the build and of its frame.
- **`scripts/check-tsl`** runs on CI (`.github/workflows/tsl.yml`). It runs
  the crate's tests, then replays the corpus in Node.js
  (`tools/tsl/check.mjs`). All 4,217 programs match.
- **`tests/browser/tsl-editor.spec.js`** opens the gallery page and the
  original page on the pinned sources (`reference/three-js/tsl-editor.html`),
  both with a frozen clock. They are driven by the same edits and option
  changes.
  - All twelve result texts match.
  - The previews agree.
  - Steady frames create no GPU objects.

## Differences from the original

- Console messages are compared without the locations V8 appends from
  three's JavaScript stacks (`"…" at "NodeBuilder.js:2495"`). Those files
  and lines do not exist in the port.
- The GPU's own validation messages are left out too. They are asynchronous
  shader and pipeline errors from the device, and the headless device does
  not compile shaders.
- In the editor, pipelines are created synchronously. WebGPU reports the
  same pipelines; only the timing of their creation differs.
- Typed arrays the user's JavaScript creates stay JavaScript objects. They
  are read through Reflect, and `instanceof` against the port's typed array
  constructors does not match them.
