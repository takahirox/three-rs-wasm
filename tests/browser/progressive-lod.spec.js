// webgl_loader_gltf_progressive_lod against the original page ( texture-volumes.html ):
// once the streamed LODs settle, every mesh shows the same geometry and texture
// levels and the frames match, at the page's camera and after an orbit; the steady
// state creates no GPU resources and uploads nothing.
import {expect, test} from '@playwright/test';
import {PNG} from 'pngjs';

const id = 'webgl_loader_gltf_progressive_lod';
const size = {width: 800, height: 600};
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

// In-flight requests to Needle Cloud: a frame range ends only once they settle.
const track = page => {
  const inflight = new Set();
  page.on('request', r => { if (r.url().includes('needle.tools')) inflight.add(r); });
  for (const e of ['requestfinished', 'requestfailed']) page.on(e, r => inflight.delete(r));
  return inflight;
};
const settle = async (page, inflight) => {
  for (let quiet = 0; quiet < 10;) {
    await page.waitForTimeout(200);
    quiet = inflight.size ? 0 : quiet + 1;
  }
};
const open = async (page, runtime) => {
  await page.setViewportSize(size);
  if (runtime === 'rust') {
    await page.goto(`/web/gallery/example.html?id=${id}&still=1`);
    await page.waitForFunction(() => window.app, null, {timeout: 120000});
  } else {
    await page.goto(`/reference/three-js/texture-volumes.html?id=${id}`);
    await page.waitForFunction(() => window.fixtureCallback && window.reference, null, {timeout: 300000});
  }
};
const frame = (page, runtime, t) => page.evaluate(async ({runtime, t}) => {
  if (runtime === 'reference') return renderFixture(t);
  const c = document.querySelector('canvas'), p = c.dataset.frames;
  app.gallery_time(t);
  while (c.dataset.frames === p) await new Promise(r => requestAnimationFrame(r));
}, {runtime, t});
// Each mesh: name ( GLTFLoader drops dots ), geometry LOD level, vertex count and
// the LOD level of each texture slot.
const state = (page, runtime) => page.evaluate(runtime => {
  if (runtime === 'rust') {
    return document.body.dataset.lodState.split('\n').map(row => {
      const [name, level, count, slots] = row.split('|');
      return [name.replaceAll('.', ''), level, count, slots.split(',').filter(Boolean).sort().join(',')].join('|');
    }).sort();
  }
  const names = {map: 'Map', emissiveMap: 'Emissive', normalMap: 'Normal', roughnessMap: 'Roughness', metalnessMap: 'Metalness', aoMap: 'Ao'};
  const rows = [];
  reference.scene.traverse(o => {
    if (!o.isMesh) return;
    const m = o.material, slots = [];
    for (const k of Object.keys(m)) if (m[k]?.isTexture && names[k]) slots.push(`${names[k]}:${m[k].userData?.LODS?.level ?? ''}`);
    rows.push([o.name, o.geometry.userData?.LODS?.level ?? '', o.geometry.attributes.position.count, slots.sort().join(',')].join('|'));
  });
  return rows.sort();
}, runtime);

const run = async (page, {frames, drag}) => {
  const results = {};
  const inflight = track(page);
  for (const runtime of ['reference', 'rust']) {
    await open(page, runtime);
    for (let k = 0; k < frames; k++) {
      if (drag && k === drag.at) {
        await page.mouse.move(...drag.from);
        await page.mouse.down();
        await page.mouse.move(...drag.to, {steps: 3});
        await page.mouse.up();
      }
      await frame(page, runtime, k / 60);
      if (k % 30 === 29) await settle(page, inflight);
    }
    if (runtime === 'rust') await page.addStyleTag({content: '#notice,#settings,#info{display:none!important}'});
    results[runtime] = {state: await state(page, runtime), shot: await page.locator('canvas').screenshot()};
  }
  return results;
};

test.describe.configure({mode: 'serial'});

for (const [name, scenario] of Object.entries({
  'the page camera': {frames: 240},
  'an orbit drag': {frames: 360, drag: {at: 150, from: [400, 300], to: [470, 330]}},
})) {
  test(`progressive LOD levels and frames match the original at ${name}`, async ({page}) => {
    test.setTimeout(900000);
    const {reference, rust} = await run(page, scenario);
    expect(rust.state).toEqual(reference.state);
    const d = diff(reference.shot, rust.shot);
    expect(d.mean, JSON.stringify(d)).toBeLessThan(limits.mean);
    expect(d.over32, JSON.stringify(d)).toBeLessThan(limits.over32);
  });
}

test('progressive LOD GPU residency', async ({page}) => {
  test.setTimeout(900000);
  await page.addInitScript(() => {
    window.creates = 0;
    for (const key of ['createBuffer', 'createTexture', 'createBindGroup', 'createShaderModule', 'createRenderPipeline', 'createComputePipeline']) {
      const original = GPUDevice.prototype[key];
      GPUDevice.prototype[key] = function (...args) { creates++; return original.apply(this, args); };
    }
  });
  const inflight = track(page);
  await open(page, 'rust');
  for (let k = 0; k < 120; k++) {
    await frame(page, 'rust', k / 60);
    if (k % 30 === 29) await settle(page, inflight);
  }
  const read = () => page.evaluate(() => ({creates, transfers: JSON.parse(app.transfer_counts()).slice(0, 3), resources: Array.from(app.resource_counts())}));
  for (let k = 120; k < 150; k++) await frame(page, 'rust', k / 60);
  const before = await read();
  for (let k = 150; k < 210; k++) await frame(page, 'rust', k / 60);
  expect(await read()).toEqual(before);
});
