# Gallery packages

The repository's root crate, `three-rs-wasm`, is the library. The examples
gallery's ports are a separate workspace member, `gallery/`
( `three-rs-gallery` ), which depends on the library and re-exports it, so a
port reads like code inside it. The library exposes what the ports use: the
render targets' views and options, `texture_gpu`, the shader program
constructors and its embedded shader sources ( `three_rs_wasm::sources` ).

## One link, split

Each example module of `gallery/src/browser/` is a Cargo feature of the
gallery, enabling the modules it uses; `example_<module>` also compiles the
module's example programs and its browser oracle exports. The gallery's shell
( `browser.rs`: the page, its frame loop and its input; `expanded.rs`, which
hands them to the example; `example.rs`, the example's hooks; and helpers the
modules share, such as `gltf_viewer` ) is always built. It holds no example:
every program, the MVP page's ( `mvp`, `geometry_cube`, `point_lights`,
`loader_gltf` ) included, is a module's.

`scripts/build-gallery` links the whole gallery once, every feature on, and
splits the result ( `tools/gallery/split/` ) into `web/build/`:

- `three_rs_wasm_bg.wasm`, the primary module, with its part of
  wasm-bindgen's glue ( `three_rs_wasm.js` ): the library, the shell and the
  code no single module owns;
- `parts/<module>.wasm`, one part per module: the functions and the read-only
  data only that module reaches, its programs among them;
- `parts/chunk<n>.wasm`: code and data that several modules share and the
  shell does not, one chunk per set of modules ( sets under 5 KB stay in the
  primary module );
- `parts/<name>.js`, a part's glue: the wrappers of the exports it holds ( its
  programs, its classes' methods ) and the JavaScript functions only it calls
  ( `tools/gallery/split/glue.mjs` );
- `parts.json`, the parts each module loads and those with glue.

A page loads the primary module, then its parts ( `web/examples/runtime.js` );
`web/examples/modules.json` maps each example to its module
( `tools/gallery/modules.py`: the module whose file defines the example's
program ), and the build writes the module's parts into the example's page.
wasm-bindgen's adapters go with what they adapt: an export's ( a program's
`externref shim` ) into the part that holds the program, which exports it
itself, and a JavaScript function's into the part that alone calls it, which
imports that function from its glue rather than through the primary module.
The primary module's glue keeps what the library and the shell call, and
none of the snippets ( the Rust modules' `inline_js`: Ammo, Jolt, IFC,
MaterialX, Rhino, meshoptimizer, HTML textures ): a part's glue imports the
snippets its functions come from. A snippet function several modules call
without a chunk of their own ( Ammo's loader and soft-body world ) keeps its
adapter in the primary module, whose glue forwards it to what those modules'
glue hands over as their parts load. The parts import the primary module's memory, stack pointer,
function tables and functions, and a part's instantiation writes its
functions into the table the primary module calls them through and its data
into the shared memory, at the addresses the linker gave it. The page then
runs in one Wasm instance's memory: a call into a part is a call through a
table, as a call through a function pointer is; nothing crosses into
JavaScript.

### How the split is chosen

The gallery links with `--emit-relocs`, so the linker's relocations give
every reference: calls, function pointers ( in code and in vtables ) and
addresses of data in code and in data. `assign.py` builds the graph:

- a module's roots are the functions its symbols place in it
  ( `three_rs_gallery::browser::<module>::…` ), generic code instantiated for
  its types alone ( demangled with `c++filt` ), and the exports its pages call
  ( `fbx_canonical`, `BounceWorld`, `XrApp`, the offscreen worker's );
- the shell is everything the other exports reach without entering a module's
  roots ( the dispatch calls into a module only for that module's examples );
- a module owns what only its roots reach outside the shell; what several
  reach is shared.

Data whose bytes overlap a symbol with another owner ( merged constants )
stays in the primary module. The build then checks the result: from the
exports and a module's roots, without entering another module's roots,
everything reached must be in the primary module or in a part that module
loads.

A function's owner comes from its symbol, so the build turns off LLVM's
merging of identical functions ( `-Zmerge-functions=disabled`, through
`RUSTC_BOOTSTRAP` on the stable compiler ): a merged function would carry one
module's name and serve another's calls.

The functions move with Binaryen's `wasm-split --multi-split` ( version 127,
without placeholders: the parts load before the example starts ), the data
with `tools/gallery/split/wasm.py`, and `wasm-opt -O` optimizes each module.

## The site's layout

`web/` is the site, laid out as the three.js repository is, and the same
locally ( `/web/examples/<id>.html` ) and published
( `/three-rs-wasm/examples/<id>.html` ):

- `build/`, the build's output: the gallery's Wasm and glue
  ( `scripts/build-gallery` ) and the TSL editor's ( `build/tsl/` );
- `examples/`: the index, every example's page beside it, the pages' scripts,
  and the examples' assets ( `models/`, `textures/`, `environments/`,
  `assets/` ), which the ports load by paths relative to their pages, as
  three.js examples do;
- `files/`: the pages' stylesheet, fonts and icons, as three.js's `files/`;
- `index.html`, which opens `examples/`.

`tools/build_site.py` copies `web/` with the build's output, leaving out the
r186 assets the CDN serves ( below ).

## A page and a program per example

As in three.js, every example has its own page, `web/examples/<id>.html`,
and its own program, a Rust function exported under the example's id
( `webgl_loader_texture_ktx`, `-` read as `_` ) in the module that ports it.
The program creates the example's renderer, its scene and its frame loop
( `browser::program` with the example's `PageOptions`: canvas alpha, samples,
output encoding and target format, on-demand rendering ); its scene object
implements `browser::example::Example`, whose hooks the frame loop and the
page's input call. Nothing dispatches on an example number.

A page loads the primary module and its glue, then its parts and their glue,
and calls its program. `scripts/build-gallery` writes the pages with
`tools/gallery/pages.mjs`, as the parts are its output: the markup is
`tools/gallery/page.html`, and a ported example's script is
`web/examples/example.js` with the example's id, parts and program written in,
the code for the other examples dropped ( the generator evaluates the
conditions on `PAGE_EXAMPLE`, the catalog's number for the example ) and the
imports the rest does not use removed. Examples not ported get a page with
their diagnostic ( `web/examples/diagnostic.js` ).
`web/examples/example.html?id=<id>`, the gallery's earlier URL, forwards to
the example's page.

The WebXR examples' programs return the app `web/examples/xr.js` drives with
the session's frames; the transpiler's page calls its module's
`tsl_transpile`, and the TSL editor's runs on the three-tsl crate's Wasm.

## Sizes ( 2026-10-23 )

| Build | Size | gzip |
| --- | ---: | ---: |
| Primary module | 1.10 MB | 0.36 MB |
| Its glue ( `three_rs_wasm.js` ) | 201 KB | 21 KB |
| 334 parts ( 239 modules, 95 chunks ) | 28.0 MB | 8.9 MB |
| 244 parts' glue | 215 KB | 84 KB |
| A page's Wasm: primary and parts, median over the examples | 2.31 MB | 0.92 MB |
| A page's Wasm, largest | 4.72 MB | 1.64 MB |
| A page's glue, median | 240 KB | 29 KB |

A page loads 15 parts at the median, 44 at most.

When the shell still held 27 examples' scenes ( the MVP page's, the glTF
viewer's and the shell's own, until 2026-10-23 ), the primary module was
2.82 MB: their code and the library code only they reached loaded on every
page, and a page's Wasm was 2.99 MB at the median. With one glue for every
example ( 2026-10-20 ), the glue was 342 KB ( 37 KB compressed ) on every
page: it held the wrappers of every example's program and the JavaScript
functions the parts call. Giving every example its own program added about
2 MB to the parts together ( 24.0 MB before ), a few kilobytes per program.

Before the split, a page's package was 2.19 MB
compressed at the median over the examples ( 2.78 MB at most ), and the
forty packages 144 MB together; the gallery's Wasm is now 29 MB, built in one
link instead of forty.

The Draco decoder and the Basis Universal transcoders are installed by the
pages that use them, as three.js pages hand GLTFLoader a DRACOLoader or a
KTX2Loader ( `compression::install_draco`, `basis::install`: the loaders call
them through a function pointer, so only the installing modules reach them;
without one, loading a compressed asset fails with GLTFLoader's message ).
They are 1.3 MB of the primary module otherwise.

Images follow the same rule. In the browser the shell decodes JPEG, WebP and
AVIF with the browser's codecs, as TextureLoader and GLTFLoader do, and PNG
and GIF in Rust with their decoders alone ( `Texture::from_image`, which
keeps RGB under transparent pixels ); Radiance HDR uses the HDR decoder
alone. The `image` crate's other codecs are reached only by the modules that
call it themselves or install them ( `material::install_image_codecs` ), so
they left the primary module too ( 0.3 MB ).

## Before: forty packages

Until 2026-10-20 the gallery was forty packages, each the library, the shell
and a group of modules linked on its own ( `tools/gallery/packages.py` ):
246 MB together, then 144 MB once a module whose helpers another borrowed no
longer brought its examples along. Every package repeated the library and the
shell; the split keeps one copy.

## Assets from the three.js repository

623 of the examples' asset files ( 300 MB ) are the pinned r186's own files
under `examples/`. They stay in the repository, for the tests and tools that
read them, but the published site leaves them out and loads them from the
three.js repository at the pinned commit on jsDelivr
( `https://cdn.jsdelivr.net/gh/mrdoob/three.js@<commit>/` ), which sends CORS
and cross-origin resource policy headers. The site is about 460 MB instead
of 750 MB.

- `tools/gallery/upstream_assets.py` lists them in
  `web/examples/upstream-assets.json`: each file under `web/examples/assets`,
  `web/examples/textures`, `web/models` and `web/environments` whose bytes equal
  an r186 file's ( `.cache/three-r187` ). Its base is null: locally the pages
  load the repository's files.
- `tools/build_site.py` leaves the listed files out and writes the CDN base
  into the published listing.
- `web/examples/upstream.js`, which the example pages, the offscreen worker and
  `web/examples/mvp.html` load first, redirects the listed URLs: `fetch()`, the `src`
  of images, media and sources, `new Audio()` and images in HTML markup. A
  redirected image or medium loads with CORS, so canvases and GPU uploads can
  read it.
- An example's page carries its part of the listing ( `#upstream-assets` ):
  the files its example loads, as `tools/gallery/upstream_pages.mjs` records
  them by opening every page ( `tools/gallery/upstream-pages.json`; rerun it
  when an example's assets change ). A request for a listed file its part
  lacks ( after input the recording did not give ) fails on the published
  site and is retried with the whole listing; the offscreen worker and
  `web/examples/mvp.html` read the whole listing.

The published-site check ( `tests/pages/` ) runs the examples against the
CDN.
