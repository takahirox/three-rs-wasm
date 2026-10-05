# WebGL pages without a WebGPU counterpart

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.

The r186 WebGPU examples with these names show different scenes, so these
WebGL pages are ported as WebGL-only examples. Each is compared against
WebGLRenderer (`tests/browser/texture-volumes.spec.js`).

| Official example | Runtime ID | Source | Retained workload and behavior |
| --- | ---: | --- | --- |
| `webgl_postprocessing_fxaa` | 452 | `fxaa_webgl.rs` | 100 instanced flat tetrahedrons, two EffectComposers per frame ( without and with FXAAPass ) in scissored halves, auto-rotating OrbitControls |
| `webgl_multisampled_renderbuffers` | 453 | `msaa_renderbuffers.rs` | 50 Lambert spheres with wireframe copies in fog, two EffectComposers per frame ( a plain and a 4× MSAA half-float target ) in scissored halves, the animate GUI |
| `webgl_postprocessing_afterimage` | 454 | `afterimage_webgl.rs` | The turning box through AfterimagePass's two half-float history targets and OutputPass, the damp and enable GUI |
| `webgl_mirror` | 455 | `mirror_webgl.rs` | The room, the two Reflectors with their 4× MSAA targets rendered in onBeforeRender order, the resolution GUI and OrbitControls |

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

## Comparison tolerances

- **4× MSAA lines and edges**: WebGL's and WebGPU's 4× MSAA resolve
  one-pixel lines and edges differently.
  - The multisampled renderbuffers page's right half is dense wireframe
    lines: 9 % of the image differ ( mean 2.5 ). The non-MSAA left half
    matches to one pixel.
  - The mirror reflections are 4× MSAA targets on both sides: 0.6 %
    ( 0.9 % with MSAA on the canvas ).
- FXAA and the afterimage match within the default thresholds; the
  afterimage matches exactly.

## Workload

- Steady frames create no GPU resources. The afterimage binds both
  ping-pong parities once.
- The mirror's resolution GUI resizes its reflection targets, as the page's
  setSize does; the residency cycle leaves that parameter out.
- No timing parity is claimed.
