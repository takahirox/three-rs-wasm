// webgl_renderer_pathtracer against the original page ( texture-volumes.html ):
// the data textures' bytes, the frames of the raster fallback, the fade and the
// path traced image on the shared example clock, the GUI's parameters, orbiting,
// the download and the steady state's GPU residency.
import {expect, test} from '@playwright/test';
import {PNG} from 'pngjs';

const id = 'webgl_renderer_pathtracer';
const size = {width: 512, height: 512};
// ANGLE ( WebGL ) and Dawn ( WebGPU ) round the path tracer's float math
// differently: individual paths diverge into scattered noise. The raster
// fallback's transmissive parts use the engine's transmission pass.
const limits = {mean: 1.0, over32: 0.01};

const diff = (a, b) => {
  const A = PNG.sync.read(a), B = PNG.sync.read(b);
  let sum = 0, over = 0;
  for (let i = 0; i < A.data.length; i += 4) {
    let m = 0;
    for (let k = 0; k < 3; k++) {
      const d = Math.abs(A.data[i + k] - B.data[i + k]);
      sum += d;
      m = Math.max(m, d);
    }
    if (m > 32) over++;
  }
  const n = A.width * A.height;
  return {mean: sum / n / 3, over32: over / n};
};

const openRust = async page => {
  await page.setViewportSize(size);
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);
  await page.waitForFunction(() => window.app);
  // The original renders without antialiasing here ( samples=1 ).
  await page.evaluate(() => app.set_samples(1));
  await page.waitForFunction(() => { app.gallery_time(0); return document.body.dataset.ptReady; }, null, {timeout: 300000, polling: 250});
  await page.addStyleTag({content: '#notice,#settings,#info{display:none!important}html,body{background:#000!important;background-image:none!important}'});
};
const openReference = async page => {
  await page.setViewportSize(size);
  await page.goto(`/reference/three-js/texture-volumes.html?id=${id}&samples=1`);
  await page.waitForFunction(() => window.fixturePathTracer?._generator.geometry.index?.count > 3, null, {timeout: 300000});
};
const frame = (page, runtime, t) => page.evaluate(async ({runtime, t}) => {
  if (runtime === 'reference') return renderFixture(t);
  const c = document.querySelector('canvas'), p = c.dataset.frames;
  app.gallery_time(t);
  while (c.dataset.frames === p) await new Promise(r => requestAnimationFrame(r));
}, {runtime, t});
const names = ['enable', 'pause', 'toneMapping', 'transparentBackground', 'resolutionScale', 'tiles', 'roughness', 'metalness'];
const parameter = (page, runtime, i, v) => page.evaluate(({runtime, i, v, names}) => {
  if (runtime === 'rust') return app.tsl_parameter(i, v);
  const c = fixtureControls.find(c => c.property === names[i]);
  c.setValue(typeof c.object[c.property] === 'boolean' ? v !== 0 : v);
}, {runtime, i, v, names});
const samples = (page, runtime) => page.evaluate(runtime => runtime === 'rust' ? Number(document.body.dataset.ptSamples) : Math.floor(fixturePathTracer.samples), runtime);

// Runs both pages through `frames` frames at k / 60, applying `changes` at frame `at`.
const run = async (page, {frames, changes = [], at = -1, drag}) => {
  const shots = {}, counts = {};
  for (const runtime of ['reference', 'rust']) {
    await (runtime === 'rust' ? openRust : openReference)(page);
    for (let k = 0; k < frames; k++) {
      if (k === at) for (const [i, v] of changes) await parameter(page, runtime, i, v);
      if (k === at && drag) {
        await page.mouse.move(...drag[0]);
        await page.mouse.down();
        await page.mouse.move(...drag[1], {steps: 3});
        await page.mouse.up();
      }
      await frame(page, runtime, k / 60);
    }
    counts[runtime] = await samples(page, runtime);
    shots[runtime] = await page.locator('canvas').screenshot();
  }
  return {shots, counts};
};

test.describe.configure({mode: 'serial'});

test('path tracer data textures match the original uploads', async ({page}) => {
  test.setTimeout(600000);
  await page.addInitScript(() => {
    window.uploads = [];
    const hash = a => {
      const u = new Uint8Array(a.buffer, a.byteOffset, a.byteLength);
      let h = 2166136261;
      for (let i = 0; i < u.length; i++) { h ^= u[i]; h = Math.imul(h, 16777619); }
      return (h >>> 0).toString(16) + ':' + u.length;
    };
    for (const fn of ['texSubImage2D', 'texSubImage3D']) {
      const o = WebGL2RenderingContext.prototype[fn];
      WebGL2RenderingContext.prototype[fn] = function (...a) {
        const data = a.find(x => ArrayBuffer.isView(x));
        if (data) uploads.push([fn, ...a.filter(x => typeof x === 'number').slice(0, 9), hash(data)]);
        return o.apply(this, a);
      };
    }
  });
  await openReference(page);
  for (let k = 0; k < 10; k++) await frame(page, 'reference', k / 60);
  const up = await page.evaluate(() => uploads);
  const find = (fn, pred) => up.filter(u => u[0] === fn && pred(u)).map(u => u[u.length - 1]);
  const reference = {
    index: find('texSubImage2D', u => u[7] === 36249)[0],
    position: find('texSubImage2D', u => u[5] === 899 && u[7] === 6408)[0],
    bvhBounds: find('texSubImage2D', u => u[7] === 6408 && u[8] === 5126 && u[5] === 760)[0],
    bvhContents: find('texSubImage2D', u => u[7] === 33320)[0],
    attributes: find('texSubImage3D', u => u[6] === 899)[0],
    materialIndex: find('texSubImage2D', u => u[7] === 36244)[0],
    materials: find('texSubImage2D', u => u[7] === 6408 && u[8] === 5126 && u[5] < 64 && u[5] !== 25 && u[5] !== 1)[0],
    blueNoise: find('texSubImage2D', u => u[5] === 64 && u[7] === 6403)[0],
    stratified: find('texSubImage2D', u => u[5] === 25)[0],
    gradient: find('texSubImage2D', u => u[5] === 512 && u[7] === 6408 && u[8] === 5126)[0],
    floorMap: find('texSubImage2D', u => u[5] === 1024 && u[8] === 5121)[0],
    environment: find('texSubImage2D', u => u[5] === 2048 && u[7] === 6408 && u[8] === 5131)[0],
  };
  // The blurred environment the GPU filtered and the original read back.
  const blurred = await page.evaluate(() => Array.from(fixturePathTracer._pathTracer.material.envMapInfo.map.image.data));
  await page.addInitScript(() => { window.ptExposeEnvironment = true; });
  await openRust(page);
  await page.waitForFunction(() => window.ptEnvironment);
  const rust = JSON.parse(await page.evaluate(() => document.body.dataset.ptHashes));
  for (const key of Object.keys(reference)) expect(rust[key], key).toBe(reference[key]);
  // ANGLE and Dawn filter the cube-UV atlas with different rounding: most
  // halfs are identical and the rest differ in their last places.
  const halfs = await page.evaluate(() => Array.from(window.ptEnvironment));
  expect(halfs.length).toBe(blurred.length);
  const value = h => {
    const s = h & 0x8000 ? -1 : 1, e = (h >> 10) & 31, m = h & 1023;
    return e === 0 ? s * m * 2 ** -24 : s * (1 + m / 1024) * 2 ** (e - 15);
  };
  let same = 0, worst = 0;
  for (let i = 0; i < halfs.length; i++) {
    if (halfs[i] === blurred[i]) { same++; continue; }
    const a = value(blurred[i]), b = value(halfs[i]);
    worst = Math.max(worst, Math.abs(a - b) / Math.max(Math.abs(a), 1e-6));
  }
  expect(same / halfs.length).toBeGreaterThan(0.7);
  expect(worst).toBeLessThan(0.01);
});

for (const frames of [20, 50, 100]) {
  // 20: the raster fallback; 50: the quad fading in; 100: the path traced image.
  test(`path tracer frame ${frames} matches the original`, async ({page}) => {
    test.setTimeout(600000);
    const {shots, counts} = await run(page, {frames});
    expect(counts.rust).toBe(counts.reference);
    const d = diff(shots.reference, shots.rust);
    expect(d.mean, JSON.stringify(d)).toBeLessThan(limits.mean);
    expect(d.over32, JSON.stringify(d)).toBeLessThan(limits.over32);
  });
}

const scenarios = {
  'tiles 1 and resolution scale 0.5': {changes: [[5, 1], [4, 0.5]], at: 40},
  'transparent background': {changes: [[3, 1]], at: 40},
  'transparent background before the first sample': {changes: [[3, 1]], at: 5},
  'floor roughness and metalness': {changes: [[6, 0.5], [7, 0.2]], at: 40},
  'tone mapping off': {changes: [[2, 0]], at: 40},
  'path tracing disabled': {changes: [[0, 0]], at: 60},
  'paused': {changes: [[1, 1]], at: 45},
  'orbit drag': {at: 40, drag: [[256, 256], [300, 280]]},
};
for (const [name, scenario] of Object.entries(scenarios)) {
  test(`path tracer GUI: ${name}`, async ({page}) => {
    test.setTimeout(600000);
    const {shots, counts} = await run(page, {frames: 100, ...scenario});
    if (process.env.PT_SHOTS) for (const [k, v] of Object.entries(shots)) (await import('node:fs')).writeFileSync(`${process.env.PT_SHOTS}/${k}.png`, v);
    expect(counts.rust).toBe(counts.reference);
    const d = diff(shots.reference, shots.rust);
    expect(d.mean, JSON.stringify(d)).toBeLessThan(limits.mean);
    expect(d.over32, JSON.stringify(d)).toBeLessThan(limits.over32);
  });
}

test('path tracer download image saves the drawn canvas', async ({page}) => {
  test.setTimeout(600000);
  await openRust(page);
  for (let k = 0; k < 60; k++) await frame(page, 'rust', k / 60);
  await parameter(page, 'rust', 8, 1);
  await frame(page, 'rust', 60 / 60);
  const file = await page.evaluate(async () => {
    for (let i = 0; i < 120; i++) {
      const f = app.gallery_take_export();
      if (f) return [f[0], Array.from(f[1])];
      await new Promise(r => requestAnimationFrame(r));
    }
    return null;
  });
  expect(file?.[0]).toBe('pathtraced-render.png');
  const saved = Buffer.from(file[1]);
  const d = diff(saved, await page.locator('canvas').screenshot());
  expect(d.mean).toBeLessThan(0.5);
});

test('path tracer GPU residency', async ({page}) => {
  test.setTimeout(600000);
  await page.addInitScript(() => {
    window.creates = 0;
    for (const key of ['createBuffer', 'createTexture', 'createBindGroup', 'createShaderModule', 'createRenderPipeline', 'createComputePipeline']) {
      const original = GPUDevice.prototype[key];
      GPUDevice.prototype[key] = function (...args) { creates++; return original.apply(this, args); };
    }
  });
  await openRust(page);
  const read = () => page.evaluate(() => ({creates, transfers: JSON.parse(app.transfer_counts()).slice(0, 3), resources: Array.from(app.resource_counts())}));
  // The raster fallback's frames, then the path traced ones.
  for (let k = 0; k < 15; k++) await frame(page, 'rust', k / 60);
  const raster = await read();
  for (let k = 15; k < 30; k++) await frame(page, 'rust', k / 60);
  expect(await read()).toEqual(raster);
  for (let k = 30; k < 70; k++) await frame(page, 'rust', k / 60);
  const traced = await read();
  for (let k = 70; k < 110; k++) await frame(page, 'rust', k / 60);
  expect(await read()).toEqual(traced);
});
