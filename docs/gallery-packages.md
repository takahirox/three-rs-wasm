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
( `browser.rs`, `expanded.rs`, `gallery_scenes.rs` and the shared scenes:
`gallery`, `gltf_viewer`, `point_lights`, `robot`, `expanded_*` ) is always
built.

`scripts/build-gallery` links the whole gallery once, every feature on, and
splits the result ( `tools/gallery/split/` ) into `web/pkg/gallery/`:

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

A page loads the primary module, then its parts ( `web/gallery/runtime.js` );
`web/gallery/modules.json` maps each example to its module
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

## A page and a program per example

As in three.js, every example has its own page, `web/gallery/examples/<id>.html`,
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
`web/gallery/example.js` with the example's id, parts and program written in,
the code for the other examples dropped ( the generator evaluates the
conditions on `PAGE_EXAMPLE`, the catalog's number for the example ) and the
imports the rest does not use removed. Examples not ported get a page with
their diagnostic ( `web/gallery/diagnostic.js` ).
`web/gallery/example.html?id=<id>`, the gallery's earlier URL, forwards to
the example's page.

The WebXR examples' programs return the app `web/gallery/xr.js` drives with
the session's frames; the transpiler's page calls its module's
`tsl_transpile`, and the TSL editor's runs on the three-tsl crate's Wasm.

## Sizes ( 2026-10-21 )

| Build | Size | gzip |
| --- | ---: | ---: |
| Primary module | 2.82 MB | 1.06 MB |
| Its glue ( `three_rs_wasm.js` ) | 235 KB | 26 KB |
| 293 parts ( 219 modules, 74 chunks ) | 26.1 MB | 8.0 MB |
| 220 parts' glue | 172 KB | 71 KB |
| A page's Wasm: primary and parts, median over the examples | 2.99 MB | 1.12 MB |
| A page's Wasm, largest | 5.03 MB | 1.71 MB |
| A page's glue, median | 236 KB | 26 KB |

With one glue for every example ( 2026-10-20 ), the glue was 342 KB ( 37 KB
compressed ) on every page and the primary module 2.96 MB: it held the
wrappers of every example's program and the JavaScript functions the parts
call. Giving every example its own program added about 2 MB to the parts
together ( 24.0 MB before ), a few kilobytes per program; a page's Wasm went
from 3.10 MB at the median to 2.99 MB.

A page loads up to 24 parts. Before the split, a page's package was 2.19 MB
compressed at the median over the examples ( 2.78 MB at most ), and the
forty packages 144 MB together; the gallery's Wasm is now 27 MB, built in one
link instead of forty.

The Draco decoder and the Basis Universal transcoders are installed by the
pages that use them, as three.js pages hand GLTFLoader a DRACOLoader or a
KTX2Loader ( `compression::install_draco`, `basis::install`: the loaders call
them through a function pointer, so only the installing modules reach them;
without one, loading a compressed asset fails with GLTFLoader's message ).
They are 1.3 MB of the primary module otherwise. webgl_loader_gltf_avif, an
example the shell dispatches, installs Draco through
`gallery/src/browser/draco_loader.rs`, a module of its own, so its page loads
the decoder as a part.

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
  `web/gallery/upstream-assets.json`: each file under `web/gallery/assets`,
  `web/gallery/textures`, `web/models` and `web/environments` whose bytes equal
  an r186 file's ( `.cache/three-r186` ). Its base is null: locally the pages
  load the repository's files.
- `tools/build_site.py` leaves the listed files out and writes the CDN base
  into the published listing.
- `web/gallery/upstream.js`, which the example pages, the offscreen worker and
  `web/index.html` load first, redirects the listed URLs: `fetch()`, the `src`
  of images, media and sources, `new Audio()` and images in HTML markup. A
  redirected image or medium loads with CORS, so canvases and GPU uploads can
  read it.

The published-site check ( `tests/pages/` ) runs the examples against the
CDN.
