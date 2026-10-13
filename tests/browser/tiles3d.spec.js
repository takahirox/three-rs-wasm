// webgl_loader_3dtiles against the original page ( texture-volumes.html with upstream's
// WebGLOutput depth texture and EffectPassAdapter, which r186 lacks ): the same tiles are
// used, visible and active by their place in the tree; the frames match at the page's
// camera and at an evening hour ( with the cloud drift stopped on both sides, so the
// temporal passes converge on the same clouds ); GlobeControls' drag and wheel move the
// camera as the page's do; and the steady state creates no GPU resources.
import {expect, test} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync} from 'node:fs';

const id = 'webgl_loader_3dtiles';
const size = {width: 800, height: 600};
// The port's personal Cesium ion key ( allowed for 127.0.0.1 ) for both pages.
const ionKey = readFileSync('src/browser/tiles3d/ion_key.rs', 'utf8').match(/"([^"]+)"/)[1];
// WebGL2 against WebGPU: the clouds' STBN / Bayer jitter phase and the tiles' edges.
const limits = {mean: 3.5, over32: 0.03};

const diff = (a, b) => {
  const A = PNG.sync.read(a), B = PNG.sync.read(b);
  const width = Math.min(A.width, B.width), height = Math.min(A.height, B.height);
  let sum = 0, over = 0;
  for (let y = 0; y < height; y++) for (let x = 0; x < width; x++) {
    const i = (y * A.width + x) * 4, j = (y * B.width + x) * 4;
    let m = 0;
    for (let k = 0; k < 3; k++) {
      const d = Math.abs(A.data[i + k] - B.data[j + k]);
      sum += d;
      m = Math.max(m, d);
    }
    if (m > 32) over++;
  }
  const n = width * height;
  return {mean: sum / n / 3, over32: over / n};
};

// Requests to Cesium ion and the tiles: a frame range ends only once they settle.
const track = page => {
  const inflight = new Set();
  page.on('request', r => { if (/googleapis|cesium|githubusercontent/.test(r.url())) inflight.add(r); });
  for (const e of ['requestfinished', 'requestfailed']) page.on(e, r => inflight.delete(r));
  return inflight;
};
// A request whose page navigated away never reports its end: each wait is bounded.
const settle = async (page, inflight) => {
  for (let quiet = 0, waited = 0; quiet < 8 && waited < 120; waited++) {
    await page.waitForTimeout(250);
    quiet = inflight.size ? 0 : quiet + 1;
  }
};
// GlobeControls integrates performance.now() deltas ( clamped to 64 ms ): the gesture test
// gives both pages the same clock, one sixtieth of a second per frame.
const fixedClock = page => page.addInitScript(() => {
  let now = 0;
  performance.now = () => now;
  window.advanceClock = ms => { now += ms; };
});
const open = async (page, runtime) => {
  await page.setViewportSize(size);
  if (runtime === 'rust') {
    await page.addInitScript(() => { window.tilesState = true; window.tilesStaticWeather = true; });
    await page.goto(`/web/gallery/example.html?id=${id}&still=1`);
    await page.waitForFunction(() => window.app, null, {timeout: 120000});
  } else {
    await page.addInitScript(key => { window.fixtureIonKey = key; window.fixtureStaticWeather = true; }, ionKey);
    await page.route('**/.cache/three-r186/src/renderers/webgl/WebGLOutput.js', route => route.fulfill({contentType: 'text/javascript', body: readFileSync('reference/three-js/patches/WebGLOutput.js', 'utf8')}));
    await page.goto(`/reference/three-js/texture-volumes.html?id=${id}`);
    await page.waitForFunction(() => window.fixtureCallback, null, {timeout: 300000});
  }
};
const frame = (page, runtime, t) => page.evaluate(async ({runtime, t}) => {
  window.advanceClock?.(1000 / 60);
  if (runtime === 'reference') return renderFixture(t);
  const c = document.querySelector('canvas'), p = c.dataset.frames;
  app.gallery_time(t);
  while (c.dataset.frames === p) await new Promise(r => requestAnimationFrame(r));
}, {runtime, t});
// The used tiles by tree path ( child indices through the external tilesets: Google's file
// names differ per session ), with their frustum, loading, visible and active flags.
const tiles = (page, runtime) => page.evaluate(runtime => {
  if (runtime === 'rust') {
    return document.body.dataset.tiles.split('\n').slice(2).map(row => row.replace(/ e=\S+/, '').replace(/ ge=\S+/, ''));
  }
  const t = fixtureTiles, rows = [];
  const path = x => { const p = []; while (x.parent) { p.push(x.parent.children.indexOf(x)); x = x.parent; } return p.reverse().join('/'); };
  t.traverse(x => {
    if (x.traversal && (x.traversal.used || t.visibleTiles.has(x))) rows.push(`${path(x)} f=${+x.traversal.inFrustum} s=${x.internal.loadingState} v=${+t.visibleTiles.has(x)} a=${+x.traversal.active}`);
    return false;
  }, null, false);
  return rows.sort();
}, runtime);
const camera = (page, runtime) => page.evaluate(runtime => runtime === 'reference'
  ? Array.from(fixtureCamera.matrixWorld.elements).slice(12, 15)
  : document.body.dataset.tiles.split('\n')[1].split(',').slice(12, 15).map(Number), runtime);
const setHour = (page, runtime, hour) => page.evaluate(({runtime, hour}) => {
  if (runtime === 'rust') return app.tsl_parameter(0, hour);
  const input = document.getElementById('hour');
  input.value = hour;
  input.dispatchEvent(new Event('input'));
}, {runtime, hour});

const run = async (page, {frames, hour, gestures}) => {
  const results = {};
  const inflight = track(page);
  for (const runtime of ['reference', 'rust']) {
    await open(page, runtime);
    if (hour !== undefined) await setHour(page, runtime, hour);
    let k = 0;
    for (; k < frames; k++) {
      await frame(page, runtime, k / 60);
      if (k % 30 === 29) await settle(page, inflight);
    }
    const positions = [];
    if (gestures) {
      // A drag ( the globe rotates under the pointer ), then the wheel toward the center.
      await page.mouse.move(400, 450);
      await page.mouse.down();
      for (let i = 1; i <= 20; i++) { await page.mouse.move(400 + i * 5, 450 - i * 2); await frame(page, runtime, k++ / 60); }
      await page.mouse.up();
      for (let i = 0; i < 30; i++) await frame(page, runtime, k++ / 60);
      positions.push(await camera(page, runtime));
      await page.mouse.move(400, 300);
      for (let i = 0; i < 5; i++) { await page.mouse.wheel(0, -100); await frame(page, runtime, k++ / 60); }
      for (let i = 0; i < 30; i++) await frame(page, runtime, k++ / 60);
      positions.push(await camera(page, runtime));
    }
    if (runtime === 'rust') await page.addStyleTag({content: '#notice,#settings,#info,#credits{display:none!important}'});
    results[runtime] = {tiles: await tiles(page, runtime), positions, shot: await page.locator('canvas').screenshot()};
  }
  return results;
};

test.describe.configure({mode: 'serial'});

for (const [name, scenario] of Object.entries({
  'the page camera': {frames: 600},
  'an evening hour': {frames: 600, hour: 8.5},
})) {
  test(`3D tiles and the atmosphere match the original at ${name}`, async ({page}) => {
    test.setTimeout(1200000);
    const {reference, rust} = await run(page, scenario);
    expect(rust.tiles).toEqual(reference.tiles);
    const d = diff(reference.shot, rust.shot);
    expect(d.mean, JSON.stringify(d)).toBeLessThan(limits.mean);
    expect(d.over32, JSON.stringify(d)).toBeLessThan(limits.over32);
  });
}

test('GlobeControls drag and wheel follow the original', async ({page}) => {
  test.setTimeout(1200000);
  await fixedClock(page);
  const {reference, rust} = await run(page, {frames: 240, gestures: true});
  // The drag rotates about the first hit; the wheel's zoom point hits tiles that are still
  // reloading after the drag, as fast as the network serves them ( about 2 % of the move ).
  for (const [k, limit] of [5, 25].entries()) {
    const p = reference.positions[k], q = rust.positions[k];
    const distance = Math.hypot(p[0] - q[0], p[1] - q[1], p[2] - q[2]);
    expect(distance, JSON.stringify({reference: p, rust: q})).toBeLessThan(limit);
  }
});

test('3D tiles GPU residency', async ({page}) => {
  test.setTimeout(1200000);
  await page.addInitScript(() => {
    window.creates = 0;
    for (const key of ['createBuffer', 'createTexture', 'createBindGroup', 'createShaderModule', 'createRenderPipeline', 'createComputePipeline', 'createSampler']) {
      const original = GPUDevice.prototype[key];
      GPUDevice.prototype[key] = function (...args) { creates++; return original.apply(this, args); };
    }
  });
  const inflight = track(page);
  await open(page, 'rust');
  for (let k = 0; k < 400; k++) {
    await frame(page, 'rust', k / 60);
    if (k % 30 === 29) await settle(page, inflight);
  }
  const read = () => page.evaluate(() => ({creates, transfers: JSON.parse(app.transfer_counts()).slice(0, 3), tiles: document.body.dataset.tiles.split('\n')[0]}));
  for (let k = 400; k < 430; k++) await frame(page, 'rust', k / 60);
  const before = await read();
  for (let k = 430; k < 490; k++) await frame(page, 'rust', k / 60);
  expect(await read()).toEqual(before);
});
