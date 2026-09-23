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
    test.setTimeout(120000);
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
    const canvas = viewer.locator('canvas').first();
    // These scenes render on demand; the first canvas owns runtime diagnostics
    // even when the example presents through several additional canvases.
    const onDemand = [16, 28, 38, 153, 156, 157, 193, 195].includes(entry.port.example);
    await expect.poll(async () => Number(await canvas.getAttribute('data-frames')), {timeout: 90000}).toBeGreaterThan(onDemand ? 0 : 2);
    await expect(canvas).not.toHaveAttribute('data-error', /.+/);
    // data-frames counts submissions. A screenshot waits for an actually presented
    // frame, so pipeline compilation finishes inside this test on software GPUs.
    await page.screenshot({timeout: 90000});
    if (entry.port.example === 4) {
      await viewer.locator('#model').selectOption('5');
      await expect(viewer.locator('#asset-status')).toHaveText('', {timeout: 90000});
    }
    expect(errors).toEqual([]);
    expect(badResponses).toEqual([]);
    expect(escapedAssets).toEqual([]);
  });
}
