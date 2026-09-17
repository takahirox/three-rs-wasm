import {test,expect} from '@playwright/test';
test('RobotExpressive parses, animates and switches clips entirely in Rust/Wasm',async({page})=>{
 test.setTimeout(120000);await page.setViewportSize({width:512,height:512});const errors=[],scripts=[];
 await page.addInitScript(()=>{
  window.gpuAllocations={};
  for(const name of ['createBuffer','createBindGroup','createTexture','createRenderPipeline']) {
   const original=GPUDevice.prototype[name];
   GPUDevice.prototype[name]=function(...args){window.gpuAllocations[name]=(window.gpuAllocations[name]||0)+1;return original.apply(this,args);};
  }
 });
 page.on('pageerror',e=>errors.push(String(e)));page.on('request',r=>{if(r.resourceType()==='script')scripts.push(r.url());});
 await page.goto('/web/gallery/example.html?id=webgl_animation_skinning_morph');
 await expect(page.locator('body')).toHaveAttribute('data-status','running',{timeout:90000});
 await expect(page.locator('#animation option')).toHaveCount(14);
 await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
 await expect.poll(async()=>page.evaluate(()=>JSON.parse(window.app.transfer_counts())[0])).toBeGreaterThan(0);
 const uploads=await page.evaluate(()=>JSON.parse(window.app.transfer_counts()));
 const allocations=await page.evaluate(()=>({...window.gpuAllocations}));
 const first=await page.locator('canvas').screenshot();await page.waitForTimeout(300);expect((await page.locator('canvas').screenshot()).equals(first)).toBe(false);
 const names=await page.evaluate(()=>JSON.parse(window.app.animation_names()));
 await page.selectOption('#animation',String(names.indexOf('Dance')));await page.waitForTimeout(300);
 const dancing=await page.locator('canvas').screenshot();expect(dancing.equals(first)).toBe(false);
 expect(scripts.filter(url=>url.includes('/three-r186/')||url.includes('three.module')||url.includes('GLTFLoader'))).toEqual([]);
 const after=await page.evaluate(()=>JSON.parse(window.app.transfer_counts()));expect(after.slice(0,3)).toEqual(uploads.slice(0,3));expect(after[3]).toBeGreaterThan(uploads[3]);
 expect(await page.evaluate(()=>window.gpuAllocations)).toEqual(allocations);
 // Resizing invalidates attachment bindings, then reaches a new steady state.
 await page.setViewportSize({width:640,height:480});await page.waitForTimeout(300);
 const resized=await page.evaluate(()=>({...window.gpuAllocations}));
 await page.waitForTimeout(300);
 expect(await page.evaluate(()=>window.gpuAllocations)).toEqual(resized);
 expect(errors).toEqual([]);
});
