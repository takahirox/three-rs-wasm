import { test, expect } from '@playwright/test';
import { PNG } from 'pngjs';

// Freeze tolerances before evaluating the Rust output. Small rasterization edge
// differences are allowed; material, transform or missing-object errors are not.
const CHANNEL_TOLERANCE=6;
const MAX_DIFFERENT_PIXEL_FRACTION=0.005;
for(const [id,name] of [[0,'basic cube'],[1,'hierarchy, textures, lights, lines and points']]) {
  test(`Three.js migration: ${name}`,async({page},testInfo)=>{
    await page.goto(`/reference/three-js/?example=${id}`);
    let canvas=page.locator('canvas');
    await expect(canvas).toHaveAttribute('data-ready','true');
    expect(await canvas.getAttribute('data-error')).toBeNull();
    const reference=await canvas.screenshot();
    await page.goto(`/web/?example=${id}&static=1`);
    canvas=page.locator('canvas');
    await expect.poll(async()=>({error:await canvas.getAttribute('data-error'),ready:Number(await canvas.getAttribute('data-frames'))>=3})).toEqual({error:null,ready:true});
    const actual=await canvas.screenshot();
    const a=PNG.sync.read(actual),b=PNG.sync.read(reference);
    expect([a.width,a.height]).toEqual([b.width,b.height]);
    let different=0;
    for(let offset=0;offset<a.data.length;offset+=4) {
      if([0,1,2].some(c=>Math.abs(a.data[offset+c]-b.data[offset+c])>CHANNEL_TOLERANCE))different++;
    }
    await testInfo.attach('reference',{body:reference,contentType:'image/png'});
    await testInfo.attach('rust',{body:actual,contentType:'image/png'});
    expect(different/(a.width*a.height),`${different} pixels differ`).toBeLessThanOrEqual(MAX_DIFFERENT_PIXEL_FRACTION);
  });
}
