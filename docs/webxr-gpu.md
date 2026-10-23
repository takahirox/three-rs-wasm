# WebXR on WebGPU

This port covers three of r186's WebXR examples:

- `webgpu_xr_cubes` (498);
- `webgpu_xr_rollercoaster` (499);
- `webgpu_xr_shadows` (500).

They run only on the WebXR/WebGPU binding (`XRGPUBinding`, Immersive Web
editor's draft of 2026-06-15). The examples' WebGL fallback
(`WebGLXRFallback.js`, which swaps in a `forceWebGL` renderer when the binding
is missing) is not ported.

As of 2026-10, no browser ships the binding by default. Chrome offers it behind
two flags: "WebXR Projection Layers" and "WebXR/WebGPU Bindings". Without the
binding, a session cannot start; three's XRManager throws in that case too.
`webgpu_xr_native_layers`, which forces WebGL, and the `webxr_*` examples,
which use WebGLRenderer, stay deferred.

## Structure

The work is split between JavaScript and Rust as three's own code splits it.

**`web/examples/xr.js`: the browser side of XRManager.setSession and
onAnimationFrame.**

- It creates the XRButton (cubes) or VRButton with the original's labels and
  styles. XRButton prefers `immersive-ar`.
- It requests the session with the `webgpu` feature.
- It creates the `XRGPUBinding` on the app's `GPUDevice`, and a projection
  layer in the binding's preferred color format.
- Each session frame, it reads the viewer pose, each view's sub-image
  (texture, viewport, view descriptor), and the input sources' target-ray and
  grip poses and gamepads. It hands them to the app.
- It maps input sources to controllers with XRManager's algorithm, and
  forwards the session's select and squeeze events and session end.
- The window animation loop stops while presenting. Like three's
  `Animation.start`, it restarts with an immediate frame.

**`src/xr.rs`: XRManager's state.**

- The view cameras get their transforms and projections as given, under the
  user camera's parent, with three's layer masks.
- The union camera is setProjectionFromUnion, with three's GL-convention
  arithmetic.
- updateUserCamera, and the fov and zoom restore at the end of a session.
- WebXRController's target-ray and grip spaces: matrices, visibility and
  selection state.
- The render loop over views: one pass per view into its layer of the
  session's texture, multisampled and resolved there. Shadows update once per
  frame. Sorting uses the union camera, as three's renderer sorts once for an
  ArrayCamera; each view culls with its own frustum.

**`gallery/src/browser/xr_app.rs`: the three scenes, ported from the originals.**

- DirectRenderPipeline's in-shader output for cubes and rollercoaster. The
  backgrounds clear to the encoded color.
- For shadows, a frame-buffer target of the page's `outputBufferType`
  (`UnsignedByteType`) per view, then three's output pass (ACES, sRGB) into
  the layer.
- An `additive` or `alpha-blend` session clears to opaque or transparent
  black, as three's Background does.

**`gallery/src/browser/xr_controller_model.rs`: XRControllerModelFactory and
motion-controllers 1.0.**

- On `connected`, it fetches the WebXR input profiles from the CDN the
  original uses, picks the layout for the handedness, and loads its glTF under
  the grip space.
- Each frame, the gamepad's buttons and axes move the visual-response nodes,
  using three's slerp.

**`gallery/src/browser/roller_coaster.rs`: `RollerCoaster.js`.**

- The track, lifters, shadow, sky and trees geometry.

**Engine changes.**

- `RenderTarget` allows multisampled layer arrays, through one-layer MSAA
  attachments, as WebGPUBackend's external XR textures do.
- `Renderer::array_camera_view_projection` sorts with the union camera.
- `Renderer::blit_encoded` is the output pass into a non-sRGB view.
- `Renderer::viewport_transmission` follows WebGPURenderer for transmission:
  - the opaque pass is snapshotted from an 8-bit target too;
  - double-sided transmissive meshes draw their back sides, then the front
    sides over a fresh copy (`transparentDoublePass`).
- The vendored wgpu adds the binding's needs (`vendor/wgpu/PATCH.md`):
  - XR-compatible adapters;
  - the `GPUDevice` of a device;
  - a `Texture` over an external `GPUTexture`.

Random numbers come from `Math.random`, or from `galleryRandom` when a page
installs one. The comparison tests install the seeded generator that the
reference page substitutes for `Math.random` in the example and in
`RollerCoaster.js`.

## Verification

`tests/browser/webxr-gpu.spec.js` runs each example twice on
`tests/browser/fake-xr.js`: once as the pinned r186 page
(`reference/three-js/webxr-gpu.html`) and once as the port.

The fake device:

- installs `navigator.xr`, `XRGPUBinding` and `XRRigidTransform`;
- has a stereo pair with asymmetric fields of view, using WebGPU's [0, 1]
  depth range, which the binding specifies, and the render state's near and
  far planes;
- has 1024² views in a two-layer projection layer of the preferred canvas
  format;
- has one right-hand `oculus-touch-v3` tracked pointer;
- steps both the session frames and the window animation frames, so both
  pages run the same frames on the same clock.

The original renders on it through `XRGPUBinding`; its WebGL fallback is not
taken.

Each case compares the following:

- the canvas before the session and after it;
- both views after 30 session frames;
- for the cubes, both views while the trigger selects (cubes launch from the
  controller, the model's trigger moves) and after release;
- both views at the session's last frame;
- the button's label and computed style through the session;
- the adapter request (`xrCompatible: true`) and the session request;
- the XRManager warning (cubes ask for multiview);
- the cubes again on an AR-capable device: `immersive-ar` with an
  `alpha-blend` display, where the background is transparent.

Tolerance: per image, at most 0.1% of pixels may have an RGBA channel
difference above 2, and the mean difference must be at most 0.5. Measured
maxima:

| Case | Fraction above 2 | Mean |
| --- | --- | --- |
| cubes (VR and AR) | 0% | 0.005 |
| rollercoaster | 0.0005% | 0.001 |
| shadows | 0.05% | 0.22 |

The shadows differences are 8-bit quantization of the lit surfaces before the
output pass.

The port must create no GPU buffers, textures, bind groups, pipelines,
shaders, samplers or layouts during 10 steady session frames.

## Limits

- No headset has run the port. Agreement with the original holds on the fake
  device only. A real runtime's views, frame timing, foveation and input
  differ from it.
- Hand input (`hand-tracking`), XR layers other than the projection layer,
  depth sensing and multiview are not ported. three's WebGPU path does not
  use multiview either.
- The profile's layout keys are read in sorted order. This only matters for
  a profile whose handedness is `any`.
- No timing parity is claimed.
