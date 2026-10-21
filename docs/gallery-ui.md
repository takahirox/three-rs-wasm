# The examples' UI in the gallery

A ported example's page shows its original's UI: stats.js, its lil-gui
panels, the WebGPU Inspector and its page chrome. Each is built with the
original's own library and from what the pinned r186 page itself builds.

## What is recorded

**`tools/gallery/ui/extract.mjs`** runs each ported example that uses
lil-gui, the Inspector or stats.js, as published
(`.cache/three-r186/examples/<id>.html`, three's build served from `src/`),
at 1280×720. The libraries are wrapped to record what the page creates. When
the UI stops changing, it writes `web/gallery/ui/specs.json`, which holds:

- each lil-gui panel's tree: folders and controllers, with type, name,
  value, range, step, options, hidden and disabled state, listening;
- each lil-gui panel's root DOM, the classes and styles the page gave its
  elements, and the elements the page added to folders;
- the Inspector's options and parameter groups, built through the same
  calls;
- the Inspector's profiler calls and its parameter panel DOM;
- each stats.js panel's shown panel, its position and style, and whether the
  page calls `begin()`/`end()` or `update()`.

It covers 366 examples. specs.json is recorded, not derived on CI: rerun
the tool when the pinned examples change.

**`tools/gallery/ui/page.py`** reads each page's HTML into
`web/gallery/ui/pages.json`:

- its stylesheet (`main.css` or `example.css`, copied to
  `web/gallery/ui/css/`);
- its head `<style>` blocks;
- its `#info`.

`scripts/check-gallery` checks pages.json against the pinned sources.

An example's page carries its own part of these records, and nothing of the
other examples': `tools/gallery/pages.mjs` writes its spec, its page chrome,
the gallery's names for its controllers whose original names differ
(`web/gallery/ui/aliases.json`) and its readout module (`extras/`) into the
page as JSON (`#gallery-ui`).

## The gallery side (`web/gallery/ui/`)

Each library is the r186 package's, loaded from jsDelivr. They are identical
to the pinned copies.

**stats.js**
- It is created as the original created it, with the original's style
  changes.
- `begin()`/`end()` or `update()` run around the port's frame. Rust calls
  `galleryFrameBegin` and `galleryFrameEnd` around each frame it renders.

**lil-gui**
- The recorded tree is built with lil-gui's own API.
- Each controller writes to a proxy object. A change goes to the port's
  parameter of the same name in the example's control list
  (`app.tsl_parameter`).
- The gallery's own panel keeps only the controls no original controller
  stands for.
- `extras/` keeps the readouts some originals add to folders up to date,
  with the original's formulas, a module per readout loaded by the pages
  that show it:
  - draw calls and memory (the instancing pages);
  - path-tracer samples;
  - the active base action.

**Inspector**
- The original `Inspector` class runs unchanged. Its parameter groups are
  rebuilt with the recorded calls. The page's import map points `three`,
  `three/webgpu` and `three/tsl` at `ui/three/shim.js`, so no three.js
  renderer loads. The shim provides r186's `constants.js` and
  `EventDispatcher.js`, unchanged, and InspectorBase; the Inspector's
  rendering features get inert stand-ins.
- The Inspector reads a renderer facade:
  - **Frames:** each port frame is one render ("Scene"), timed on the CPU
    around the frame.
  - **GPU time:** with `timestamp-query`, `gpu-timing.js` adds
    `timestampWrites` to every pass of the frame. As three's
    WebGPUTimestampQueryPool does, one readback through a resident buffer
    is in flight at a time, and the frames rendered meanwhile accumulate in
    the query set. Only the last 16 frames' times are listed: the Inspector
    looks up each listed frame on every frame, and a longer list slowed the
    page to half its frame rate within seconds.
  - **Memory:** `gpu-memory.js` counts what the page allocates through
    WebGPU, in renderer.info.memory's categories:
    - buffers by usage;
    - textures by estimated size, and render targets;
    - shader modules as programs.

**Page chrome**
- The gallery's stylesheet is replaced by the original's. The original's
  head styles and `#info` are added; elements the port already made, with
  the same id, are left out.

Automated rendering comparisons capture the canvas without this UI. Pages
show it unless `navigator.webdriver` is set, and `?ui=1` shows it there too.
Without the UI, the page neither counts its allocations nor requests
`timestamp-query`.

## Verification

`tests/browser/gallery-ui.spec.js` opens every recorded example with
`?ui=1` at 1280×720 and compares:

- each lil-gui panel's DOM, with lil-gui's generated ids and the values of
  listening controllers normalized;
- the Inspector's parameter panel DOM (its random list ids normalized);
- the stats.js panels' style and the shown panel;
- the `#info` text.

All 364 compared examples match.

## Limits

- The Performance tab lists the port's frame as one "Scene" render. It does
  not list the original's individual `render()` and `compute()` calls.
- The Timeline tab's recording, the Viewer, overdraw, Settings' "Force
  WebGL" and the extensions (TSL graph, color grading) need three's
  renderer and are inert.
- Memory counts come from WebGPU allocations, not from three's objects.
  Geometries are the vertex buffers.
- Controllers that listen to a property show its recorded value; the port's
  changing state is not read back.
- Controllers the port has no parameter for are shown disabled. They are
  listed per example in the test's `unbound.json` and in each example's
  limitations.
