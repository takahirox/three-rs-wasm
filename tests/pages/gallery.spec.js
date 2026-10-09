import {test, expect} from '@playwright/test';
import {readFileSync} from 'node:fs';

const catalog = JSON.parse(readFileSync('web/gallery/catalog.json'));
test('site entry opens the gallery under the project prefix', async ({page}) => {
  await page.goto('/three-rs-wasm/');
  await expect(page).toHaveURL(/\/three-rs-wasm\/web\/gallery\/$/);
  await expect(page.locator('.card')).toHaveCount(catalog.examples.filter(e => e.port).length);
});

for (const entry of catalog.examples.filter(e => e.port)) {
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
    await page.goto(`/three-rs-wasm/web/gallery/#${entry.id}`);
    const viewer = page.frameLocator('#viewer');
    if(entry.id==='webgpu_compute_audio')await viewer.getByRole('button',{name:'Play',exact:true}).click();
    if(entry.id==='webgl_postprocessing_glitch')await viewer.locator('#startButton').click();
    const canvas = viewer.locator('canvas').first();
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
