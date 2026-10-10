# webgl_morphtargets_webcam

The page tracks the viewer's face with MediaPipe's FaceLandmarker ( tasks-vision
0.10.35 ) and drives the Face Cap head over the mirrored webcam image.
`src/browser/morphtargets_webcam.rs` ports the scene and animate().

## Split between JavaScript and Rust

MediaPipe is a JavaScript and Wasm library that runs its own GPU model. The
gallery page loads and runs it exactly as the original page does
( `web/gallery/example.js`, example 487 ):

- the same CDN modules, model asset, options ( GPU delegate, blendshapes,
  transformation matrices, VIDEO mode, one face ) and getUserMedia constraint;
- the page's `video.readyState >= HAVE_METADATA` check and `detectForVideo(
  video, Date.now() )`.

`window.galleryFaceDetect` exposes that call. The Rust frame calls it once per
animation frame, as animate() does, and applies the results:

- the facial transformation matrix, copied verbatim into the pose container;
- the blendshape scores, mapped through the page's blendshapesMap onto the
  head's morph target influences ( morphTargetDictionary from targetNames );
- the eye-look scores, summed per axis in category order into the eyes' x and
  z rotations, limited to 30°.

The rest of the scene is Rust:

- MediaPipe's virtual camera ( 63°, 1 to 10,000 cm, at the origin ), with the
  video's aspect from loadedmetadata;
- the scene mirrored in x;
- the registration transform and the reparented `grp_transform`;
- MeshNormalMaterial on the head and teeth;
- the ambient light;
- ACES Filmic;
- the video plane, which fills the frustum 100 cm away, draws first without
  depth, and copies each new video frame to its texture.

The canvas is the largest rectangle of the video's aspect, centred over the
grey body.

The GUI starts closed. It holds the 52 influence sliders in dictionary order,
and they follow the tracked values ( listen ).

## Engine change

three's MeshNormalMaterial shader has no tone-mapping step. The renderer
used to tone-map it under ACES. It now writes the normal colour unmapped, as
three does.

## Comparison

`tests/browser/morphtargets-webcam.spec.js` gives both pages the same
deterministic inputs:

- a fixed 1280 × 720 canvas stream as the webcam;
- a scripted MediaPipe module, served in place of the CDN one, whose
  detectForVideo returns a pose matrix and 52 scores computed from the
  frame's time.

Under these inputs the pose, the influences, the eye rotations, the mirrored
video plane and the letterbox match the original. Across the sampled times
the mean error is about 0.04 per channel ( of 255 ). The GUI case sets the
untracked tongueOut slider and checks that the tracked sliders follow the
values. A separate test runs the real FaceLandmarker on the fake stream:
there is no face in the stream, so the head keeps its default pose.

What tracking a real face produces depends on MediaPipe's model, the same
library in both pages. It is not compared. No timing-parity claim is made.
