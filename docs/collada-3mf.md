# Collada animation, 3MF, the raycaster helper and Web Audio timing

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
The Collada scenes are in `src/browser/refraction_loaders/collada_anim.rs`,
with the loader extensions in `src/browser/refraction_loaders/formats.rs`.
The 3MF scenes are in `src/browser/threemf.rs`, the raycaster helper in
`src/browser/raycaster_helper.rs`, and the audio timing scene in
`src/browser/audio_timing.rs` with its Web Audio graph in
`web/gallery/audio.js`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgl_loader_collada_skinning` | 316 | The textured stormtrooper's Collada skin and bone hierarchy, its matrix animation on an AnimationMixer, the grid and OrbitControls |
| `webgl_loader_collada_kinematics` | 317 | The ABB robot's kinematics joints, random joint targets tweened with Quadratic.Out on the example clock's timers, and the orbiting camera |
| `webgl_loader_3mf` | 318 | All seven 3MF samples (materials, face and vertex colors, textures, beam lattices, components), centered on load, with the z-up orbit, rendering only on change |
| `webgl_loader_3mf_materials` | 319 | The truck (base materials, logo texture, nested components) with shadows from a SunLight, a hemisphere light, fog and the orbit, rendering only on change |
| `misc_raycaster_helper` | 322 | Three double-sided capsules crossing a fixed ray, and the helper's ray, near and far squares, origin color and up to 20 hit points |
| `webaudio_timing` | 323 | The Play overlay, five balls bouncing with PCF shadows, one HRTF PositionalAudio per ball played on each bounce, and the orbit |

`misc_raycaster_helper` is a WebGPU example and is compared against the
WebGPU renderer. The others have no WebGPU equivalent in the pinned
inventory and are compared against the WebGL renderer.

## Port notes

### Collada

- **Skins.** `instance_controller` is parsed into skin joints, inverse bind
  matrices and vertex weights. Weights are sorted to the four largest and
  normalized, as `normalizeSkinWeights` does. The bind shape matrix is baked
  into the positions and normals: it is a linear transform applied before
  skinning, so the result is the same.
- **Scene graph.** Nodes are built as `buildNode` builds them: a node with a
  single object collapses into it, bones take their sid, and the skeleton
  follows the joint order. The Z_UP scene turns by −π/2.
- **Animation.** Matrix channels become position, rotation and scale tracks
  on a linear AnimationMixer.
- **Kinematics.** Joints, axes, limits and `bind_joint_axis` are parsed, and
  `setJointValue` turns or slides the bound nodes. The tween, its easing and
  the `setTimeout` loop run on the example clock, with the fixture's seeded
  `randInt`.
- **Shading.** Lambert, Phong, Standard and Physical materials on geometry
  without normals now shade flat, as three switches them to flat shading.

### 3MF

- **Archive.** The ZIP reader handles ZIP64 (the truck).
- **Parsing.** Elements are matched by local name. Base materials become flat
  Phong, or Standard when a display-properties group gives metallic values.
  Texture groups keep their wrap and filter settings; images with embedded
  ICC profiles are decoded without color management, as the browser's
  `ImageBitmap` for the loader is.
- **Objects.** Color groups become vertex colors, beam lattices merge their
  cylinders and spheres, components are cloned, and build items apply their
  transforms.
- **Switching.** All seven samples are loaded at startup. Selecting one
  rebuilds the scene, centers it with a Box3 and resets the controls.

### Raycaster helper

- **Raycast.** Each frame moves the capsules and then intersects them. As in
  the original, the raycast reads the world matrices from the previous
  render. Double-sided capsules report both entry and exit hits.
- **Helper.** The hit points are one InstancedMesh of 20 spheres; missing hits
  keep the last point under a zero scale. The renderer now accepts
  zero-scale instance matrices (still rejecting mirrored ones), which draw
  only degenerate triangles, as in three.
- **Lines.** The helper rewrites its two lines with the same points every
  frame. The ray never moves, so the port keeps them resident.
- **Normals.** MeshNormalMaterial's packed normals display unchanged. With
  an encoding output they are first read as sRGB, as the node material's
  `colorSpaceToWorking` does.

### Audio timing

- **Start.** The Play overlay starts the example, as `init()` runs on its
  click. The scene appears once the sound is decoded, as the loader callback
  adds the balls.
- **Bounces.** Rust computes the heights and detects the turn from falling to
  rising. Each frame the page takes the listener and source placement and
  the pending plays from `audio_frame`.
- **Web Audio.** `web/gallery/audio.js` builds three's graph: the listener
  gain, and one HRTF panner and gain per ball. Positions and orientations
  ramp to their values over the frame, and a play is ignored while that
  ball's previous sound is still playing, as `Audio.play()` does.

Stats and the lil-gui appearance are not reproduced.

## Comparison

`reference/three-js/collada-3mf.html` executes the pinned sources on the
example clock. The raycaster helper imports the pinned npm package in place
of its CDN URL. The suite covers:

- time steps, all seven 3MF samples, drags, pans, wheels and resize, at DPR 1
  and 2;
- the audio example's sound starts: the same frame sequence must start the
  same number of sounds on both sides.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Collada skinning, MSAA off / on | 0% / 0.01, 6.408% / 2.27 | 0.001% / 0.00, 3.313% / 1.18 |
| Collada kinematics, MSAA off / on | 0% / 0.00, 3.886% / 1.32 | 0.005% / 0.00, 2.008% / 0.68 |
| 3MF samples, MSAA off / on | 0.323% / 0.10, 5.067% / 1.20 | 0.221% / 0.07, 2.578% / 0.58 |
| 3MF truck, MSAA off / on | 0.713% / 0.12, 2.598% / 0.52 | 0.725% / 0.13, 1.817% / 0.31 |
| Raycaster helper, MSAA off / on | 0% / 0.00, 0.342% / 0.15 | 0% / 0.00, 0.171% / 0.07 |
| Audio timing, MSAA off / on | 0.029% / 0.01, 0.220% / 0.06 | 0.025% / 0.01, 0.126% / 0.03 |

### Truck shadow edges

- **Cause.** The SunLight's cascaded shadow edges fall on slightly different
  texels in the WebGL and WebGPU shadow paths.
- **Effect.** Only pixels along shadow boundaries on the truck and ground
  differ.
- **Bound.** With MSAA off, the truck is bounded at 1% / 0.3.

### MSAA

With 4× MSAA, the differences lie on the one-pixel grid lines, silhouettes
and thin 3MF details. With MSAA off, all scenes except the truck match within
the ordinary threshold. The suite requires, with MSAA:

| Case | Bound |
| --- | --- |
| Collada skinning | 7% / 2.5 |
| Collada kinematics | 4.5% / 1.5 |
| 3MF samples | 6% / 1.4 |
| 3MF truck | 3% / 0.6 |

## Performance evidence

- **Draw workload.** The measured draws equal the original's.
- **Uploads.** No scene writes geometry or texture data in steady frames:
  - skinning streams only the bone palettes;
  - the raycaster helper writes its 20 instance matrices each frame, as
    `instanceMatrix.needsUpdate` does, but not the two unchanged lines the
    original rewrites (48 bytes per frame);
  - the 3MF samples rebuild only when another sample is selected.
- **Warmed cycles.** Warmed cycles of time, input and resize create no GPU
  resources. The truck's first views from new angles create the draw slots
  of objects entering the frustum, as three uploads an object on its first
  render, so its cycle turns the camera and back.

No GPU timing parity is claimed. Full measurements are in
[collada-3mf-comparison.json](collada-3mf-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js collada-3mf.spec.js
LOADERS_DPR=2 npx playwright test -c playwright.gallery.config.js collada-3mf.spec.js
```
