import {test, expect} from '@playwright/test';
import {existsSync, readFileSync} from 'node:fs';

const catalog = JSON.parse(readFileSync('web/examples/catalog.json'));

// CI's shards ( GALLERY_SHARD=i/n, .github/workflows/pages.yml ) share the
// examples by their durations on its software GPU, recorded from a run's reports
// ( tests/pages/durations.json, tools/ci/gallery_durations.py ): each example,
// longest first, joins the shard with the least time so far. Playwright's own
// --shard splits the list into runs of consecutive examples, whose costs differ
// by half. The site's own tests run in the first shard.
const SITE_TESTS = ['site entry', 'official-style example URLs'];
const [shardOf, longestFirst] = (() => {
  const [index, count] = (process.env.GALLERY_SHARD ?? '1/1').split('/').map(Number);
  const recorded = existsSync('tests/pages/durations.json') ? JSON.parse(readFileSync('tests/pages/durations.json')) : {};
  const known = Object.values(recorded).sort((a, b) => a - b);
  const typical = known.length ? known[known.length >> 1] : 6;
  // Without a record: the city and Sponza take minutes ( see their timeouts ).
  const duration = id => recorded[id] ?? (['webgpu_generator_city', 'webgpu_lightprobes_sponza', 'webgpu_vxgi_sponza'].includes(id) ? 300 : typical);
  const load = Array(count).fill(0);
  const shard = new Map(SITE_TESTS.map(name => [name, 1]));
  for (const name of SITE_TESTS) load[0] += duration(name);
  const ids = catalog.examples.filter(e => e.port).map(e => e.id).sort((a, b) => duration(b) - duration(a) || a.localeCompare(b));
  for (const id of ids) {
    const least = load.indexOf(Math.min(...load));
    load[least] += duration(id);
    shard.set(id, least + 1);
  }
  return [name => shard.get(name) === index, ids];
})();

if (shardOf('site entry')) test('site entry opens the gallery under the project prefix', async ({page}) => {
  await page.goto('/three-rs-wasm/');
  await expect(page).toHaveURL(/\/three-rs-wasm\/examples\/$/);
  await expect(page.locator('.card')).toHaveCount(catalog.examples.filter(e => e.port).length);
});

// threejs.org's URLs: examples/#<id> opens the example in the viewer, which loads
// examples/<id>.html; that page also runs on its own.
if (shardOf('official-style example URLs')) test('official-style example URLs', async ({page}) => {
  // Three page loads, each allowed 90 seconds on a software GPU.
  test.setTimeout(360000);
  await page.goto('/three-rs-wasm/examples/#webgpu_materials');
  const viewer = page.frameLocator('#viewer');
  await expect(page.locator('#viewer')).toHaveAttribute('src', 'webgpu_materials.html');
  await expect(viewer.locator('body')).toHaveAttribute('data-backend', 'rust-wasm-webgpu', {timeout: 90000});
  await expect.poll(async () => Number(await viewer.locator('canvas').first().getAttribute('data-frames')), {timeout: 90000}).toBeGreaterThan(2);
  await page.locator('.card a[href$="webgpu_xr_cubes.html"]').click();
  await expect(page).toHaveURL(/\/examples\/#webgpu_xr_cubes$/);
  await page.goto('/three-rs-wasm/examples/webgpu_materials.html');
  await expect(page.locator('body')).toHaveAttribute('data-backend', 'rust-wasm-webgpu', {timeout: 90000});
  await page.goto('/three-rs-wasm/examples/webgl_loader_gltf.html');
  await expect(page.locator('#diagnostic a[href="webgpu_loader_gltf.html"]')).toBeVisible();
});

// Longest first, so the shard's workers finish together.
const ports = new Map(catalog.examples.filter(e => e.port).map(e => [e.id, e]));
for (const entry of longestFirst.filter(shardOf).map(id => ports.get(id))) {
  test(`published gallery: ${entry.id}`, async ({page}) => {
    // The city bakes 60 probe-cube faces per frame beside its 4096² shadow and 4×
    // MSAA scene: on the software GPU its first frames take minutes. Sponza
    // ( 481, 482 ) download about 50 MB from glTF-Sample-Assets at run time.
    const heavy = [449, 481, 482].includes(entry.port.example);
    test.setTimeout(heavy ? 600000 : 120000);
    const errors = [];
    const badResponses = [];
    const escapedAssets = [];
    page.on('pageerror', error => errors.push(String(error)));
    page.on('response', response => {
      if (response.status() >= 400) badResponses.push(response.url());
    });
    page.on('request', request => {
      if (new URL(request.url()).pathname.startsWith('/web/')) escapedAssets.push(request.url());
    });
    await page.goto(`/three-rs-wasm/examples/#${entry.id}`);
    const viewer = page.frameLocator('#viewer');
    if(entry.id==='webgpu_compute_audio')await viewer.getByRole('button',{name:'Play',exact:true}).click();
    if(entry.id==='webgl_postprocessing_glitch')await viewer.locator('#startButton').click();
    const canvas = viewer.locator('canvas').first();
    // The transpiler page draws nothing: it transpiles its editor's GLSL in Rust.
    if (entry.port.example === 483) {
      await expect.poll(async () => Number(await viewer.locator('body').getAttribute('data-builds')), {timeout: 90000}).toBeGreaterThan(0);
      await expect(viewer.locator('#result')).toContainText('Three.js Transpiler r186');
      expect(errors).toEqual([]);
      expect(badResponses).toEqual([]);
      expect(escapedAssets).toEqual([]);
      return;
    }
    // The TSL editor runs on the three-tsl crate: its builds fill the result editor and
    // its preview canvas renders through the port's WebGPURenderer.
    if (entry.port.example === 497) {
      await expect.poll(async () => Number(await viewer.locator('body').getAttribute('data-builds')), {timeout: 110000}).toBeGreaterThan(0);
      await expect(viewer.locator('#result')).toContainText('Three.js r186 - Node System');
      await expect.poll(async () => Number(await viewer.locator('#renderer canvas').getAttribute('data-frames')), {timeout: 60000}).toBeGreaterThan(2);
      expect(errors).toEqual([]);
      expect(badResponses).toEqual([]);
      expect(escapedAssets).toEqual([]);
      return;
    }
    // These scenes render on demand; the first canvas owns runtime diagnostics
    // even when the example presents through several additional canvases.
    const onDemand = [16, 28, 38, 153, 156, 157, 193, 195, 206, 213, 220, 224, 226, 235, 236, 237, 240, 242, 255, 277, 283, 296, 308, 313, 314, 315, 318, 319].includes(entry.port.example);
    await expect.poll(async () => Number(await canvas.getAttribute('data-frames')), {timeout: heavy ? 570000 : 90000}).toBeGreaterThan(onDemand ? 0 : 2);
    await expect(canvas).not.toHaveAttribute('data-error', /.+/);
    if (entry.port.example === 4) {
      await viewer.locator('#model').selectOption('5');
      await expect(viewer.locator('#asset-status')).toHaveText('', {timeout: 90000});
    }
    expect(errors).toEqual([]);
    expect(badResponses).toEqual([]);
    expect(escapedAssets).toEqual([]);
  });
}
