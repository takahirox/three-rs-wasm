# Mapped spotlight, multiple clones, skinning blends and the walking soldier

Pinned Three.js r186: `148ef33ecb6d2502ff796d4554abd1549c95d519`.
Implemented in `src/browser/spot_skinning.rs`, with Three.js AnimationMixer
semantics in `src/browser/three_mixer.rs`.

| Official example | Runtime ID | Retained workload and behavior |
| --- | ---: | --- |
| `webgpu_lights_spotlight` | 303 | The mapped, shadowed spotlight, the Lucy PLY and floor, SpotLightHelper and the shadow CameraHelper, all ten controls, the orbit |
| `webgl_animation_multiple` | 304 | Three SkeletonUtils clones playing idle, run and walk, the shared-skeleton mode, SunLight shadows |
| `webgl_animation_skinning_blending` | 305 | Weighted idle/walk/run actions, synchronized warped cross-fades, pausing and single steps, deactivation, SkeletonHelper, all 17 controls |
| `webgl_animation_skinning_additive_blending` | 306 | Xbot base-action cross-fades, the four additive actions, SkeletonHelper, the orbit without pan or zoom, all nine controls |
| `webgl_animation_walk` | 307 | Keyboard walking and running with faded transitions, HDR environment, SunLight and bulb shadows, anisotropic floor, SkeletonHelper, damped orbit, both controls |

`webgl_lights_spotlight` is excluded as an equivalent of the WebGPU port. The
four skinning examples have no WebGPU equivalent and are compared against the
WebGL renderer.

## Port notes

### Spotlight map

- **Map projection.** `SpotLightNode` multiplies the spotlight color by
  `light.map` at `lightProjectionUV`, inside the map. The receivers' Lambert
  materials take this through a light-color hook.
  - **Matrix.** The shadow matrix is computed per frame on the CPU: the
    spotlight shadow camera's fov is 2 × angle × focus, over near 2 and
    far 10.
  - **Sampling.** Samples use explicit level 0; the maps have no mipmaps.
- **Outside the map.** With the shadow on, the r186 WebGPU reference leaves no
  spotlight outside the projected map. The measured result is dark there, and
  lit with no map or with the shadow off. Weighted by the shadow intensity,
  the port reproduces this.
- **Shadows.** Two new fields in `crate::shadow::Shadow`:
  - `focus` narrows the spot shadow camera, as `SpotLightShadow.focus` does;
  - `intensity` mixes received shadows toward lit, as `LightShadow.intensity`
    does.
- **Helpers.**
  - SpotLightHelper's cone follows the light, looking at the target with the
    up-vector roll.
  - The CameraHelper keeps the lines computed before the shadow camera's first
    update (fov 50, near 2, far 10, measured) and follows only the camera's
    world matrix.

### Skinning

- **Mixer.** `ThreeMixer` ports AnimationAction and PropertyMixer:
  - effective weights and time scales with their control interpolants;
  - `crossFadeTo` with warping, `fadeIn` and `fadeOut`;
  - loop events, delivered to the examples' synchronizing listeners during the
    update;
  - pause, stop (restoring the original state) and the lend and take-back
    activation order;
  - normal accumulation by `slerpFlat`, and additive accumulation, which
    multiplies quaternions and adds vectors.

  Keyframes are sampled as three's interpolants sample them.
- **Additive clips.** `makeClipAdditive` and `subclip` are ported. The Xbot
  actions are created in `gltf.animations` order, which fixes the
  non-commutative order of additive accumulation.
- **Multiple clones.** Each `SkeletonUtils.clone` is a separate instance with
  its own mixer.
- **Shared skeleton.** Each `vanguard_Mesh` is bound in DetachedBindMode at
  ( x, 0, 0 ), scale 0.01, rotation.x −π/2. It is drawn from a copy of the
  skeleton under that transform, driven by the same clip and time. The
  original skins all three meshes from one skeleton, so the port evaluates
  that skeleton three times on the CPU. Switching modes restarts the clips, as
  the new mixers do.
- **SkeletonHelper.** It rewrites its bone segments each rendered frame after
  the mixer, as the helper's geometry does.
- **Walk.** Each step ports the walk update:
  - the azimuth-relative movement and `rotateTowards`;
  - the floor shift;
  - `fixe_transition`'s `_scheduleFading` fades;
  - the body's `metalnessMap = map`, taken as the map's blue channel.
- **Shadow culling.** Skinned shadow casters are now culled against each
  shadow camera, as three culls a SkinnedMesh by its cached bounding sphere.
  The sphere is computed once, by `Sphere.expandByPoint`, from the pose at
  first use.

Inspector, Stats and lil-gui appearance are not reproduced. OrbitControls touch
input is not separately verified.

## Comparison

`reference/three-js/spot-skinning.html` executes the pinned sources on the
example clock.

- **Coverage.** Every control, key walking and running, orbit input and
  resize, at DPR 1 and 2.
- **GUI stub.** The fixture's GUI stub resolves option-object names and
  accepts `disable()` and `enable()`.

Examples with antialiasing are compared both with and without MSAA.

Ordinary limits: at most 0.5% of pixels with an RGB channel difference above
6/255, and mean RGB error at most 0.6/255. Measured maxima:

| Case | DPR 1 | DPR 2 |
| --- | --- | --- |
| Spotlight, MSAA off / on | 0.043% / 0.10, 1.810% / 0.68 | 0.034% / 0.09, 0.925% / 0.37 |
| Multiple clones, MSAA off / on | 0.105% / 0.14, 1.007% / 0.34 | 0.087% / 0.13, 0.532% / 0.23 |
| Skinning blending, MSAA off / on | 0.101% / 0.17, 0.946% / 0.35 | 0.069% / 0.16, 0.490% / 0.25 |
| Additive blending, MSAA off / on | 0.022% / 0.13, 0.420% / 0.22 | 0.025% / 0.14, 0.217% / 0.18 |
| Walk, MSAA off / on | 0.351% / 0.21, 0.844% / 0.30 | 0.211% / 0.20, 0.456% / 0.24 |

With MSAA off, all five match within the ordinary threshold. With MSAA on,
only edge pixels (helper lines and skinned silhouettes) differ. These cases
are bounded at:

- spotlight: 2% / 0.7;
- multiple clones: 1.5% / 0.35;
- blending: 1% / 0.4;
- additive blending: 1% / 0.3;
- walk: 1.5% / 0.35.

## Performance evidence

- **Draw workload.** The measured draws equal the original's, including every
  shadow pass (the bulb's cube faces cull the soldier as three does).
- **Uploads.**
  - Skinned meshes write only their resident pose data each frame.
  - Helper segments are written only while shown.
  - No scene uploads geometry or texture data by writes after a warm pass.
- **Warmed cycles.** Warmed cycles of time, controls, input and resize create no
  GPU resources.

No GPU timing parity is claimed. Full measurements are in
[spot-skinning-comparison.json](spot-skinning-comparison.json).

```sh
npx playwright test -c playwright.gallery.config.js spot-skinning.spec.js
SKINNING_DPR=2 npx playwright test -c playwright.gallery.config.js spot-skinning.spec.js
```
