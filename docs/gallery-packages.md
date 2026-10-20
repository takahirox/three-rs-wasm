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
module's examples into the dispatch in `expanded.rs` and its browser oracle
exports. The gallery's shell ( `browser.rs`, `gallery_scenes.rs`, the dispatch
and the shared scenes: `gallery`, `gltf_viewer`, `point_lights`, `robot`,
`expanded_*` ) is always built.

`scripts/build-gallery` links the whole gallery once, every feature on, and
splits the result ( `tools/gallery/split/` ) into `web/pkg/gallery/`:

- `three_rs_wasm_bg.wasm`, the primary module, with wasm-bindgen's glue:
  the library, the shell and the code no single module owns;
- `parts/<module>.wasm`, one part per module: the functions and the read-only
  data only that module reaches;
- `parts/chunk<n>.wasm`: code and data that several modules share and the
  shell does not, one chunk per set of modules ( sets under 5 KB stay in the
  primary module );
- `parts.json`, the parts each module loads.

A page loads the primary module, then its module's parts
( `web/gallery/runtime.js` ); `web/gallery/modules.json` maps each example to
its module ( `tools/gallery/modules.py`, from the `create` arms of
`expanded.rs` ). The parts import the primary module's memory, stack pointer,
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

## Sizes ( 2026-10-20 )

| Build | Wasm | gzip |
| --- | ---: | ---: |
| Primary module | 3.28 MB | 1.22 MB |
| 292 parts ( 219 modules, 73 chunks ) | 23.7 MB | 7.3 MB |
| A page: primary and parts, median over the examples | 3.43 MB | 1.28 MB |
| A page, largest | 5.48 MB | 1.84 MB |

A page loads up to 23 parts. Before the split, a page's package was 2.19 MB
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
