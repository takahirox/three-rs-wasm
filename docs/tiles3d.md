# webgl_loader_3dtiles

The page streams Google Photorealistic 3D Tiles over Tokyo through Cesium ion
with 3d-tiles-renderer 0.4.27. It lights them with @takram's atmosphere and
volumetric clouds, under GlobeControls. `gallery/src/browser/tiles3d.rs` and
`gallery/src/browser/tiles3d/` port it.

## The reference page

On r186 the original page was broken: its renderer.setEffects path never
handed the effects the scene depth, so CloudsMaterial failed to compile
( `readDepthValue` ) and the canvas stayed black. r187 fixes this:

- WebGLOutput allocates a scene depth texture for effects that set
  `needsDepthTexture`;
- the page's EffectPassAdapter initializes each pass, reports whether it
  needs depth and passes the depth texture to it.

The comparison page ( `reference/three-js/texture-volumes.html` ) runs r187's
page, and the port reproduces it.

## Cesium ion key

Like the original, the port embeds an ion key in its public source
( `gallery/src/browser/tiles3d/ion_key.rs` ): a personal key whose allowed origins are
the local test server and the published gallery. The tests read it from that
file and give the comparison page the same key.

## Tiles ( tiles.rs, fade.rs, content.rs, volume.rs, ellipsoid.rs )

These files port the renderer in three's f64 order of operations:

- the optimized traversal ( markUsedTiles, markUsedSetLeaves,
  markVisibleTiles, toggleTiles );
- the error and LRU priority callbacks;
- the LRU cache ( 6000 / 8000 tiles, 0.3 / 0.4 GB );
- the download, parse and processNode queues;
- the plugins the page registers:
  - CesiumIonAuthPlugin's hand-off to Google's root ( key and session );
  - UpdateOnChangePlugin;
  - TilesFadePlugin ( 250 ms, with its setTileVisible rules );
  - TileCreasedNormalsPlugin ( 30°, computed on the CPU as its worker does ).

Google's file names differ per session, so the tests compare tiles by their
place in the tree. With the page's camera both pages use, show and keep
active the same tiles ( 281 cached, 166 visible ).

## Rendering ( render.rs, program.rs, atmosphere.rs, clouds.rs, sun.rs )

The frame runs the programs three compiles for the page. They are captured
from its shaderSource calls into `gallery/src/browser/tiles3d/glsl/` and converted to
WGSL ( `tools/tiles3d/convert.sh` ) by these steps:

- preprocessing;
- one std140 block shared by both stages;
- struct uniforms flattened into leaves;
- sub-vec4 arrays padded;
- textures split from their samplers;
- WebGL's clip z mapped to WebGPU's;
- clip y flipped into render targets, so they hold rows as WebGL does.

Uniform values are set by their WebGL names through the block layout read
from the WGSL. Each pass's values match the page's at the same frame, except
the frame counter and what depends on it.

- **Precompute.** PrecomputedTexturesGenerator: Bruneton's transmittance,
  irradiance and four scattering orders in float32. WebGPU cannot blend
  float32 targets, so each additive pass renders into a scratch target and a
  sum pass adds it to a copy of the target.
- **Scene.**
  - The tiles render into the HalfFloat output buffer with a depth texture:
    MeshBasicMaterial, with the fade plugin's Bayer dither while fading.
  - Frustum culling and the opaque sort follow three.
  - Each tile's geometry is uploaded once, when the tile is first shown, and
    kept until the tile is disposed.
  - NormalPass draws the MeshNormalMaterial override.
- **CloudsEffect.** CloudLayers.DEFAULT, then:
  - the cascaded shadow map ( two practical cascades, λ 0.71 ) and its
    temporal resolve;
  - the quarter-resolution clouds with Bayer jitter and reprojection;
  - the clouds' temporal upscale.
- **AerialPerspectiveEffect.** Merged with the clouds in one EffectPass: the
  sky, sun and sky light over the normal buffer, with the clouds' overlay,
  shadow and shadow length.
- **The rest.** LensFlareEffect ( threshold, 8-level mipmap blur, Kawase,
  features ), SMAA ( postprocessing's lookup images ), dithering, and
  WebGLOutput's AgX at exposure 10.
- **Sun.** getSunDirectionECEF ports astronomy-engine's VSOP87 Earth,
  precession, nutation and sidereal time, at Date.UTC( 2024, 2, 1 ) plus the
  time-of-day slider's hours.

The cloud textures load from the same takram URLs as the page. The transmittance
and irradiance read-backs only feed SkyLightProbe and SunDirectionalLight,
which the page does not create, so the port skips them.

## Controls ( controls.rs )

The port has GlobeControls over EnvironmentControls, in three's math:

- dragging rotates the globe;
- right or shift dragging rotates around the hit point, with the pivot ring;
- the wheel zooms toward the pointer;
- damping;
- the near and far planes fitted to the globe;
- the page's adjustHeight workaround ( off until the first interaction ).

Raycasts follow raycastTraverseFirstHit, through the tiles' bounding volumes
and their triangles, then fall back to the ellipsoid.

## Comparison

`tests/browser/tiles3d.spec.js` stops the clouds' drift on both pages, so the
temporal passes converge on the same clouds. It then checks:

- the tiles, by tree path and flags, at the page camera and at 17:30;
- the frames, with a mean error under 3.5 of 255 per channel;
- that the drag and the wheel move the camera to the page's position, with
  both pages on the same clock ( the damping integrates performance.now()
  deltas ): within 5 m after the drag, and within 25 m after the wheel, whose
  zoom point hits tiles that are still reloading;
- that the steady state creates no GPU resources.

The remaining frame differences are the clouds' STBN and Bayer jitter phase
and the tiles' edges.

## Limitations

- The gallery hands the controls one mouse pointer, so the two-finger touch
  gestures are not reachable.
- While the cloud textures load, the port draws nothing. The page draws the
  first frames with empty textures.
- Without float32-filterable, the port reports an error. The page would then
  precompute in half float with logarithmic transmittance.
- No timing-parity claim is made.
