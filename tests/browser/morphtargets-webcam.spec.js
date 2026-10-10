// webgl_morphtargets_webcam against the original page ( texture-volumes.html ). Both
// pages read a fixed canvas stream as the webcam and a scripted FaceLandmarker: the
// MediaPipe module is replaced by one whose detectForVideo returns a pose matrix and
// 52 blendshape scores from the frame's time. The pose, the morph influences, the
// eye rotations, the mirrored video plane and the letterboxed canvas must match, as
// must the GUI's influence sliders.
import {expect, test} from '@playwright/test';
import {PNG} from 'pngjs';

const id = 'webgl_morphtargets_webcam';
const MEDIAPIPE = 'https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision@0.10.35';
const categories = ['_neutral', 'browDownLeft', 'browDownRight', 'browInnerUp', 'browOuterUpLeft', 'browOuterUpRight', 'cheekPuff', 'cheekSquintLeft', 'cheekSquintRight', 'eyeBlinkLeft', 'eyeBlinkRight', 'eyeLookDownLeft', 'eyeLookDownRight', 'eyeLookInLeft', 'eyeLookInRight', 'eyeLookOutLeft', 'eyeLookOutRight', 'eyeLookUpLeft', 'eyeLookUpRight', 'eyeSquintLeft', 'eyeSquintRight', 'eyeWideLeft', 'eyeWideRight', 'jawForward', 'jawLeft', 'jawOpen', 'jawRight', 'mouthClose', 'mouthDimpleLeft', 'mouthDimpleRight', 'mouthFrownLeft', 'mouthFrownRight', 'mouthFunnel', 'mouthLeft', 'mouthLowerDownLeft', 'mouthLowerDownRight', 'mouthPressLeft', 'mouthPressRight', 'mouthPucker', 'mouthRight', 'mouthRollLower', 'mouthRollUpper', 'mouthShrugLower', 'mouthShrugUpper', 'mouthSmileLeft', 'mouthSmileRight', 'mouthStretchLeft', 'mouthStretchRight', 'mouthUpperUpLeft', 'mouthUpperUpRight', 'noseSneerLeft', 'noseSneerRight'];
// The scripted landmarker: a head turning and nodding 45 cm away, and scores
// that move through their range.
const mock = `export const FilesetResolver={forVisionTasks:async()=>({})};
export const FaceLandmarker={createFromOptions:async()=>({detectForVideo(){const t=window.fixtureFaceTime??0;
const a=0.4*Math.sin(t*1.3),b=0.25*Math.sin(t*0.7),ca=Math.cos(a),sa=Math.sin(a),cb=Math.cos(b),sb=Math.sin(b);
const data=[ca,sa*sb,-sa*cb,0, 0,cb,sb,0, sa,-ca*sb,ca*cb,0, 4*Math.sin(t),2*Math.cos(t*0.9),-45+3*Math.sin(t*0.5),1];
const names=${JSON.stringify(categories)};
return {facialTransformationMatrixes:[{rows:4,columns:4,data}],faceBlendshapes:[{categories:names.map((categoryName,i)=>({index:i,score:0.5+0.5*Math.sin(t*(0.5+i*0.07)+i),categoryName,displayName:''}))}]};}})};`;
// A deterministic webcam: getUserMedia returns a 1280 × 720 canvas stream of a fixed image.
const fakeCamera = () => {
  navigator.mediaDevices.getUserMedia = async () => {
    const canvas = document.createElement('canvas');
    canvas.width = 1280; canvas.height = 720;
    const g = canvas.getContext('2d');
    const draw = () => {
      const gradient = g.createLinearGradient(0, 0, 1280, 720);
      gradient.addColorStop(0, '#203080'); gradient.addColorStop(0.5, '#c04060'); gradient.addColorStop(1, '#f0d040');
      g.fillStyle = gradient; g.fillRect(0, 0, 1280, 720);
      g.fillStyle = '#ffffff'; g.fillRect(160, 120, 320, 480);
      g.fillStyle = '#10a050'; g.beginPath(); g.arc(900, 360, 220, 0, Math.PI * 2); g.fill();
      requestAnimationFrame(draw);
    };
    draw();
    return canvas.captureStream(30);
  };
};
const diff = (a, b) => {
  const A = PNG.sync.read(a), B = PNG.sync.read(b);
  let sum = 0, over = 0;
  for (let i = 0; i < A.data.length; i += 4) {
    let m = 0;
    for (let k = 0; k < 3; k++) { const d = Math.abs(A.data[i + k] - B.data[i + k]); sum += d; m = Math.max(m, d); }
    if (m > 32) over++;
  }
  return {mean: sum / (A.width * A.height) / 3, over32: over / (A.width * A.height), size: [A.width, A.height, B.width, B.height]};
};

const open = async (page, runtime) => {
  await page.route(MEDIAPIPE, route => route.fulfill({contentType: 'text/javascript', body: mock}));
  await page.addInitScript(fakeCamera);
  await page.setViewportSize({width: 800, height: 600});
  if (runtime === 'rust') {
    await page.goto(`/web/gallery/example.html?id=${id}&still=1`);
    await page.waitForFunction(() => window.app && document.querySelector('#video')?.readyState >= 1 && window.galleryVideoAspect, null, {timeout: 120000});
    await page.evaluate(() => app.set_samples(1));
  } else {
    await page.goto(`/reference/three-js/texture-volumes.html?id=${id}&samples=1`);
    // The page's video element is not in the document: wait for the stream to start.
    await page.waitForFunction(() => window.fixtureCallback && window.reference !== undefined || window.fixtureCallback, null, {timeout: 120000});
    await page.waitForTimeout(1500);
  }
  // The stream's first frame reaches the video texture.
  await page.waitForTimeout(500);
};
const frame = (page, runtime, t) => page.evaluate(async ({runtime, t}) => {
  window.fixtureFaceTime = t;
  if (runtime === 'reference') return renderFixture(t);
  const c = document.querySelector('canvas'), p = c.dataset.frames;
  app.gallery_time(t);
  while (c.dataset.frames === p) await new Promise(r => requestAnimationFrame(r));
}, {runtime, t});
const shot = page => page.locator('canvas').screenshot();

test.describe.configure({mode: 'serial'});

for (const times of [[0], [0, 0.5, 1.7], [0, 1, 2, 3.4]]) {
  test(`webcam face tracking matches the original at ${times.at(-1)} s`, async ({page}) => {
    test.setTimeout(300000);
    const shots = {};
    for (const runtime of ['reference', 'rust']) {
      await open(page, runtime);
      for (const t of times) await frame(page, runtime, t);
      // The video frame can arrive after a frame: render the last time twice.
      await frame(page, runtime, times.at(-1));
      shots[runtime] = await shot(page);
    }
    const d = diff(shots.reference, shots.rust);
    expect(d.size[0]).toBe(d.size[2]);
    expect(d.mean, JSON.stringify(d)).toBeLessThan(1.0);
    expect(d.over32, JSON.stringify(d)).toBeLessThan(0.01);
  });
}

// gui.add( influences, index ): tongueOut has no MediaPipe category, so its slider
// value stays; the tracked jawOpen slider follows the landmarker ( listen ).
test('webcam GUI influence sliders match the original', async ({page}) => {
  test.setTimeout(300000);
  const shots = {};
  for (const runtime of ['reference', 'rust']) {
    await open(page, runtime);
    await frame(page, runtime, 0.8);
    await page.evaluate(runtime => {
      if (runtime === 'rust') app.tsl_parameter(51, 1);
      else fixtureControls.find(c => c.property === 51).setValue(1);
    }, runtime);
    await frame(page, runtime, 0.8);
    await frame(page, runtime, 0.8);
    if (runtime === 'rust') {
      await page.waitForFunction(() => document.querySelector('#particle-24'));
      const [jaw, tongue] = await page.evaluate(() => [document.querySelector('#particle-24').value, document.querySelector('#particle-51').value]);
      const morph = await page.evaluate(() => document.body.dataset.morph.split(',').map(Number));
      expect(Number(jaw)).toBeCloseTo(morph[24], 2);
      expect(Number(tongue)).toBe(1);
    }
    shots[runtime] = await shot(page);
  }
  const d = diff(shots.reference, shots.rust);
  expect(d.mean, JSON.stringify(d)).toBeLessThan(1.0);
  expect(d.over32, JSON.stringify(d)).toBeLessThan(0.01);
});

// The real FaceLandmarker loads and runs on the fake stream ( it finds no face, so
// the head keeps its default pose ).
test('webcam page runs the real MediaPipe FaceLandmarker', async ({page}) => {
  test.setTimeout(300000);
  const errors = [];
  page.on('pageerror', e => errors.push(String(e)));
  await page.addInitScript(fakeCamera);
  await page.setViewportSize({width: 800, height: 600});
  await page.goto(`/web/gallery/example.html?id=${id}`);
  await page.waitForFunction(() => window.galleryVideoAspect && window.galleryFaceDetect?.toString().includes('detectForVideo'), null, {timeout: 240000});
  const frames = Number(await page.locator('canvas').getAttribute('data-frames'));
  await page.waitForFunction(f => Number(document.querySelector('canvas').dataset.frames) > f + 30, frames, {timeout: 120000});
  expect(await page.locator('canvas').getAttribute('data-error')).toBeNull();
  expect(errors).toEqual([]);
});

// Tracking only rewrites the pose, influences and eye rotations; each video frame is
// copied into the one video texture.
test('webcam GPU residency', async ({page}) => {
  test.setTimeout(300000);
  await page.addInitScript(() => {
    window.creates = 0;
    for (const key of ['createBuffer', 'createTexture', 'createBindGroup', 'createShaderModule', 'createRenderPipeline', 'createComputePipeline']) {
      const original = GPUDevice.prototype[key];
      GPUDevice.prototype[key] = function (...args) { creates++; return original.apply(this, args); };
    }
  });
  await open(page, 'rust');
  for (let k = 0; k < 30; k++) await frame(page, 'rust', k / 60);
  const read = () => page.evaluate(() => ({creates, resources: Array.from(app.resource_counts())}));
  const before = await read();
  for (let k = 30; k < 90; k++) await frame(page, 'rust', k / 60);
  expect(await read()).toEqual(before);
});
