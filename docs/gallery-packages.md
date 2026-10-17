# Gallery packages

The repository's root crate, `three-rs-wasm`, is the library. The examples
gallery's ports are a separate workspace member, `gallery/`
( `three-rs-gallery` ), which depends on the library and re-exports it, so a
port reads like code inside it. The library exposes what the ports use: the
render targets' views and options, `texture_gpu`, the shader program
constructors and its embedded shader sources ( `three_rs_wasm::sources` ).

## Packages

Each example module of `gallery/src/browser/` is a Cargo feature of the
gallery, enabling the modules it uses. `example_<module>` also compiles the
module's examples into the dispatch in `expanded.rs` and its browser oracle
exports ( `fbx_canonical`, `BounceWorld`, … ). A package enables its
examples' `example_` features only: a module whose helpers another one
borrows is compiled without its own examples, so the linker drops their
code and shaders. The gallery's shell ( `browser.rs`, `gallery_scenes.rs`,
the dispatch and the shared scenes: `gallery`, `gltf_viewer`, `point_lights`,
`robot`, `expanded_*` ) is always built.

The library's KTX2 / Basis Universal transcoder is its `basisu` feature
( on by default ). The gallery depends on the library without it and the
gallery's `basisu` feature turns it on, listed by the modules whose examples
load KTX2 textures or glTF with `KHR_texture_basisu`. Without it a Basis
texture fails with an error that names the feature.

A page loads one package from `web/pkg/<package>/`:

- `core`, the shell alone, for the examples it dispatches itself;
- one of forty packages, each a group of example modules.

`tools/gallery/packages.py` writes `web/gallery/packages.json`:

- `examples` maps each ported example to its package. An example's module is
  the feature on its `create` arm in `expanded.rs`; the transpiler and the
  WebXR examples, which the shell starts, use the `tsl_transpiler` and
  `xr_app` features.
- `packages` lists each package's modules ( `packages.py --features` prints
  their `example_` features ).

The page ( `web/gallery/example.js`, and the offscreen example's worker )
reads the mapping. `scripts/build-gallery` builds every package, in
`GALLERY_JOBS` parallel lanes with their own target directories, or the
packages it is given.

## Why groups

One package per module would be about 220 packages of about 4 MB each: the
library and the shell repeat in each, about 930 MB together, past the
published site's 1 GB with its other 500 MB of assets. The modules are packed
into forty groups instead:

- largest first;
- each into the group whose modules it adds least to ( modules that share
  helper modules land together );
- under the smallest capacity that makes at most forty groups.

A module's source size measures it: across the 218 single-module packages,
their size above the core correlates with their modules' source size at 0.97.

## Sizes ( 2026-10-17 )

| Build | Wasm | gzip |
| --- | ---: | ---: |
| Before: every port in one package | 26.0 MB | 7.4 MB |
| `core` | 2.7 MB | 1.05 MB |
| Packages, median | 3.2 MB | 1.18 MB |
| Packages, largest ( `tsl_extended` ) | 6.9 MB | 2.78 MB |
| All 40 packages | 144 MB | |

Before the `example_` features and the optional transcoder ( 2026-10-16 ):
`core` 4.6 MB ( 1.87 MB compressed ), the packages 246 MB together; a
package with one example carried every example of the modules it borrowed
from ( `loader_materialx`: 24 modules, 9.5 MB, now 2.8 MB ).

A page downloads its package only. The library alone with one cube is
1.9 MB ( 0.86 MB compressed ); the shell adds the glTF importer, the image
decoders and the page bindings.

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
