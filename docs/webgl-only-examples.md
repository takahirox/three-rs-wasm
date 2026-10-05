# WebGL pages without a WebGPU counterpart

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

The r186 WebGPU examples with these names show different scenes, so these
WebGL pages are ported as WebGL-only examples. Each is compared against
WebGLRenderer (`tests/browser/texture-volumes.spec.js`).

`webgl_morphtargets_webcam` ( a webcam and MediaPipe face tracking ) is not
ported.

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgl_postprocessing_fxaa` | 452 | `fxaa_webgl.rs` | 100 instanced flat tetrahedrons, two EffectComposers per frame ( without and with FXAAPass ) in scissored halves, auto-rotating OrbitControls |
| `webgl_multisampled_renderbuffers` | 453 | `msaa_renderbuffers.rs` | 50 Lambert spheres with wireframe copies in fog, two EffectComposers per frame ( a plain and a 4× MSAA half-float target ) in scissored halves, the animate GUI |
| `webgl_postprocessing_afterimage` | 454 | `afterimage_webgl.rs` | The turning box through AfterimagePass's two half-float history targets and OutputPass, the damp and enable GUI |
| `webgl_mirror` | 455 | `mirror_webgl.rs` | The room, the two Reflectors with their 4× MSAA targets rendered in onBeforeRender order, the resolution GUI and OrbitControls |
| `webgl_refraction` | 456 | `refraction_webgl.rs` | The Refractor's oblique render into its 1024² 4× MSAA target, WaterRefractionShader with the scrolling dudv map, OrbitControls |
| `webgl_portal` | 457 | `portal_webgl.rs` | Both portal views per frame ( frameCorners projections, 256² sRGB targets ), the clipped icospheres, ACES output, OrbitControls |
| `webgl_postprocessing_sobel` | 458 | `sobel_webgl.rs` | RenderPass, LuminosityShader and SobelOperatorShader straight to the canvas, the enable GUI |
| `webgl_postprocessing_dof` | 459 | `dof_webgl.rs` | 1,764 spheres with their own envMap materials recolored per frame, the pointer-eased camera, BokehPass's RGBA-packed depth render and 41 taps, the GUI |
| `webgl_shadowmap` | 460 | `shadowmap_webgl.rs` | The galloping morphs, the text and blocks, the 2048 × 1024 PCF shadow map with reversed depth, the ShadowMapViewer HUD on T, OrbitControls |
| `webgl_postprocessing_3dlut` | 461 | `lut_webgl.rs` | The DamagedHelmet under the UltraHDR environment, OutputPass and LUTPass with the nine tables, the GUI |
| `webgl_materials_cubemap_dynamic` | 463 | `cubemap_webgl.rs` | The CubeCamera's capture and its PMREM per frame for the mirror sphere's envMap, the circling box and knot, the quarry environment, ACES, auto-rotating OrbitControls, the GUI |
| `webgl_postprocessing_ssr` | 462 | `ssr_webgl.rs` | SSRPass's beauty, normal, metalness, march, blur and output passes, ReflectorForSSRPass, every GUI control |

## Engine additions

- **Rectangular shadow maps** ( `Shadow::map_height` ): a directional or spot
  light's map can be narrower than its atlas layer in y, with the PCF radius
  in its x texels on both axes, as r186's.
- **Reversed-depth shadow comparison** ( `Shadow::reversed_depth` ): receivers
  beyond the shadow camera's far plane read shadowed, as WebGLRenderer's
  `reversedDepthBuffer` compares them.
- **Materials' own environments** ( `MaterialProperties::env_map` ): a
  Standard or Physical material can sample its own PMREM instead of the
  scene's, at envMapIntensity, as Material.envMap does.
- **Resident cube captures** ( `CubeCapture` ): a CubeCamera's six faces and
  their PMREM render into the same textures every update.
- **A resident environment across passes**: rendering a scene without an
  environment ( a full-screen pass ) no longer drops the filtered
  environment of the scene rendered before it.

## Port notes

### Split screens ( 452, 453 )

WebGLRenderer applies its scissor to the canvas only. Each composer's
RenderPass renders its whole target, and only the last pass to the canvas
is scissored, as on the page. The column left of the right half is never
drawn.

Both pages put a transparent canvas over a white body, which shows through
where the canvas alpha is below one: the FXAA page's background and the gap
column. The fixture keeps the page's styles with the container moved to the
top of the viewport, so that the canvas fills it as the gallery's does. The
ports composite their premultiplied sRGB output over white.

FXAAShader reads the OutputPass result, which is already sRGB. The port
decodes the FXAA result before the output target's sRGB encoding; the round
trip can move a channel by one step.

### Frames

These pages draw only in animate(). The ports advance their animation only
on animation frames: the gallery clock, or a seek. Re-renders for pointer
input and resizes advance nothing, and the afterimage shows its last result
again. OrbitControls' wheel handler calls update(), which auto-rotates once
more on the FXAA page; the port counts it.

### Reflectors ( 455 )

A Reflector renders its reflection in onBeforeRender, just before it is
drawn, from inside whichever render draws it. In the main render the ground
mirror's reflection draws the back mirror, whose onBeforeRender renders
again from the ground's reflected camera. The back mirror's own reflection
then re-renders the ground mirror's target from its reflected camera.

The port replays that order. Every reflection render writes a new version
of its mirror's target. Every draw samples the version and texture matrix
current at that point, including stale ones when a mirror faces away. The
versions rotate through three targets per mirror, each with its material,
built once.

The oblique projections follow Reflector.js in WebGL clip space; the port
converts them to WebGPU's depth range ( z′ = ( z + w ) / 2 ), which keeps
the clip plane.

### Refractor and portals ( 456, 457 )

The Refractor renders from the camera with its near plane moved onto the
refractor, so only what lies behind it is drawn, unless it is seen from
behind; then it is drawn as a transparent object.

Each animation frame renders the left portal's view, which shows the right
portal's target from the previous frame, then the right portal's view, which
shows the left target just rendered. The portal targets store sRGB without
tone mapping; the main render applies ACES Filmic.

### Passes straight to the canvas ( 458, 461 )

The Sobel pass and LUTPass are their composers' last passes and write their
results to the canvas as they are, without an OutputPass. The ports decode
those values before the output target's sRGB encoding.

### DoF ( 459 )

Each sphere has its own MeshBasicMaterial. Its cube reflection is computed
per vertex, as envmap_vertex does for basic materials. BokehPass renders
the spheres again with MeshDepthMaterial's RGBA packing into a half-float
target cleared white. The port keeps that render as a second resident
scene of the same spheres.

### Shadow map ( 460 )

The page asks for `reversedDepthBuffer`. Its PCF map is compared
greater-equal against a depth cleared to 0, so receivers beyond the shadow
camera's far plane read shadowed, and the bias moves toward the light. r186's
PCF radius uses the map's x texel on both axes. The engine's shadow atlas
gains rectangular maps ( `Shadow::map_height` ) and that reversed-depth
comparison ( `Shadow::reversed_depth` ).

The four GLTFLoader callbacks draw from Math.random in completion order. The
fixture loads them one after another, in the page's order.

### Dynamic cube map ( 463 )

The mirror sphere's envMap is the CubeCamera's render target, which WebGL
filters into a PMREM again whenever the camera updates it, every frame. The
port captures the six faces without tone mapping, as render targets are
drawn, into one resident cube target, and filters it into the same atlas each
frame. The box and the knot use the scene's environment. The scene is turned
0.5 about y; the cube camera, outside it, stays at the world origin.

### SSR ( 462 )

SSRPass.render's passes run in order. The ground reflector renders with the
global clipping plane into its half-float target with a 16-bit depth
texture; the depth textures are read at 16-bit precision. The normal and
metalness renders use resident mirror scenes of the same meshes. A fragment
that returns without writing outputs zero, as ANGLE initializes it.

## Comparison tolerances

- **4× MSAA lines and edges**: WebGL's and WebGPU's 4× MSAA resolve
  one-pixel lines and edges differently.
  - The multisampled renderbuffers page's right half is dense wireframe
    lines: 9 % of the image differ ( mean 2.5 ). The non-MSAA left half
    matches to one pixel.
  - The mirror reflections are 4× MSAA targets on both sides: 0.6 %
    ( 0.9 % with MSAA on the canvas ).
- **Shadow map**: the morphs' thin limbs under 4× MSAA ( 1.5 % ).
- **3D LUT** ( unresolved ): the helmet's textured surface and the
  background's highlights differ in 1–2 % of the pixels on every state,
  graded or not.
- **SSR** ( unresolved ): at resolutionScale 0.5 the objects' reflections
  differ in 1.2 % of the pixels; at full resolution the states match within
  0.3 %. After a jump of the clock the original's ground reflection shows a
  stale band for one frame, so each capture renders two frames.
- **Dynamic cube map**: at roughness 0.4 the mirror reads the PMREM's
  blurred levels, where 0.5 % of the pixels differ ( 0.7 % with MSAA ).
- FXAA, the afterimage, refraction, portal, Sobel and DoF match within the
  default thresholds; the afterimage, Sobel and portal ( without MSAA )
  exactly.

## Workload

- Steady frames create no GPU resources. The afterimage binds both
  ping-pong parities once.
- The mirror's resolution GUI and SSR's resolutionScale resize their
  targets, as the pages' setSize does; the residency cycles leave those
  parameters out.
- The DoF residency cycle leaves the pointer out: the easing camera keeps
  changing which spheres the frustum culls.
- The dynamic cube map's draw comparison leaves out the reference's 36-count
  background boxes in the six cube faces and PMREMGenerator's face-set
  passes, which the port draws as fullscreen triangles.
- No timing parity is claimed.
