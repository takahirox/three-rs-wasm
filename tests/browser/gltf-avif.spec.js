import {test, expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';

const id = 'webgl_loader_gltf_avif';
for(const samples of [1,4])test(`AVIF Forest House matches orbit, pan, zoom and resize (${samples} samples)`, async ({page}, info) => {
  test.setTimeout(120000);
  const errors = [];
  page.on('pageerror', error => errors.push(String(error)));
  const states = [];
  async function capture(reference) {
    await page.setViewportSize({width:512,height:512});
    await page.goto(reference ? `/reference/three-js/expanded.html?id=${id}&samples=${samples}` : `/web/gallery/example.html?id=${id}`);
    const canvas = page.locator('canvas');
    if (reference) await expect(canvas).toHaveAttribute('data-ready','true',{timeout:90000});
    else {
      await expect.poll(async()=>Number(await canvas.getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan(0);
      if(samples===1){await page.evaluate(()=>app.set_samples(1));await page.waitForTimeout(100);}
      await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
    }
    const images = [];
    for (const action of [
      async()=>{},
      async()=>{await page.mouse.move(250,250);await page.mouse.down();await page.mouse.move(295,275,{steps:5});await page.mouse.up();},
      async()=>{await page.mouse.move(250,250);await page.mouse.down({button:'right'});await page.mouse.move(230,270,{steps:4});await page.mouse.up({button:'right'});},
      async()=>{await page.mouse.wheel(0,120);},
      async()=>{await page.setViewportSize({width:640,height:400});},
    ]) {
      await action();
      await page.waitForTimeout(100);
      images.push(PNG.sync.read(await canvas.screenshot()));
    }
    if (!reference) await expect(canvas).not.toHaveAttribute('data-error',/.+/);
    return images;
  }
  const reference = await capture(true);
  const actual = await capture(false);
  for (let n=0;n<actual.length;n++) {
    const a=actual[n], b=reference[n];
    expect([a.width,a.height]).toEqual([b.width,b.height]);
    let different=0,rawDifferent=0;
    // WebGL and WebGPU use different subpixel MSAA coverage. Compare a 3x3
    // area average for 4x MSAA; the 1x run retains the strict per-pixel check.
    for(let y=0;y<a.height;y++)for(let x=0;x<a.width;x++){
      const i=(y*a.width+x)*4;
      if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))rawDifferent++;
      const radius=samples===4?1:0;
      if([0,1,2].some(c=>{
        let delta=0,count=0;
        for(let dy=-radius;dy<=radius;dy++)for(let dx=-radius;dx<=radius;dx++){
          const xx=x+dx,yy=y+dy;if(xx<0||yy<0||xx>=a.width||yy>=a.height)continue;
          const index=(yy*a.width+xx)*4+c;delta+=a.data[index]-b.data[index];count++;
        }
        return Math.abs(delta/count)>6;
      }))different++;
    }
    states.push({state:n,samples,width:a.width,height:a.height,rawFraction:rawDifferent/(a.width*a.height),fraction:different/(a.width*a.height)});
    writeFileSync(info.outputPath(`${n}-actual.png`),PNG.sync.write(a));
    writeFileSync(info.outputPath(`${n}-reference.png`),PNG.sync.write(b));
  }
  writeFileSync(info.outputPath('comparison.json'),JSON.stringify(states,null,2));
  expect(errors).toEqual([]);
  for(const state of states)expect(state.fraction,JSON.stringify(state)).toBeLessThanOrEqual(.005);
});

test('AVIF scene idles without GPU work and retains geometry through camera changes', async ({page}) => {
  await page.addInitScript(()=>{
    window.gpuActivity={submits:0,bufferWrites:0,creates:0,destroyed:0,imageUploads:0,imagesClosed:0};
    for(const [proto,name,key] of [
      [GPUQueue.prototype,'copyExternalImageToTexture','imageUploads'],[ImageBitmap.prototype,'close','imagesClosed'],
      [GPUQueue.prototype,'submit','submits'],[GPUQueue.prototype,'writeBuffer','bufferWrites'],
      [GPUDevice.prototype,'createBuffer','creates'],[GPUDevice.prototype,'createTexture','creates'],
      [GPUDevice.prototype,'createBindGroup','creates'],[GPUDevice.prototype,'destroy','destroyed'],
    ]) {
      const original=proto[name];proto[name]=function(...args){window.gpuActivity[key]++;return original.apply(this,args);};
    }
  });
  await page.goto(`/web/gallery/example.html?id=${id}`);
  const canvas=page.locator('canvas');
  await expect.poll(async()=>Number(await canvas.getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan(0);
  await page.waitForTimeout(100);
  const before=await page.evaluate(()=>({activity:{...gpuActivity},transfer:JSON.parse(app.transfer_counts()),textures:Array.from(app.resource_counts())}));
  expect(before.activity.imageUploads).toBe(12);
  await page.waitForTimeout(400);
  expect(await page.evaluate(()=>({...gpuActivity}))).toEqual(before.activity);
  await page.mouse.move(250,250);await page.mouse.down();await page.mouse.move(300,275,{steps:5});await page.mouse.up();
  await expect.poll(()=>page.evaluate(()=>gpuActivity.submits)).toBeGreaterThan(before.activity.submits);
  const after=await page.evaluate(()=>({activity:{...gpuActivity},transfer:JSON.parse(app.transfer_counts()),textures:Array.from(app.resource_counts())}));
  expect(after.transfer.slice(0,3)).toEqual(before.transfer.slice(0,3));
  expect(after.textures).toEqual(before.textures);
  expect(after.activity.creates).toBe(before.activity.creates);
  expect(after.activity.imageUploads).toBe(before.activity.imageUploads);
  await page.evaluate(()=>dispatchEvent(new Event('pagehide')));
  await expect.poll(()=>page.evaluate(()=>gpuActivity.destroyed)).toBe(1);
  const final=await page.evaluate(()=>({...gpuActivity}));
  expect(final.imagesClosed).toBe(12);
  await page.waitForTimeout(200);
  expect(await page.evaluate(()=>({...gpuActivity}))).toEqual(final);
});
