import {test,expect} from '@playwright/test';
for(const id of ['webgl_buffergeometry_lines','webgpu_lights_pointlights'])test(`GPU deformation retains geometry: ${id}`,async({page})=>{
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto(`/web/gallery/example.html?id=${id}`);
 await expect(page.locator('body')).toHaveAttribute('data-status','running',{timeout:90000});
 await page.waitForTimeout(200);
 const before=await page.evaluate(()=>JSON.parse(window.app.transfer_counts()));
 const image=await page.locator('canvas').screenshot();
 await page.waitForTimeout(300);
 const after=await page.evaluate(()=>JSON.parse(window.app.transfer_counts()));
 expect(after.slice(0,3)).toEqual(before.slice(0,3));
 expect((await page.locator('canvas').screenshot()).equals(image)).toBe(false);
 expect(errors).toEqual([]);
});
for(const id of ['webgl_loader_gltf','webgl_pmrem_equirectangular','webgpu_equirectangular'])test(`HDR background reuses resources: ${id}`,async({page})=>{
 await page.addInitScript(()=>{
  window.allocations={};
  for(const name of ['createBuffer','createBindGroup','createTexture','createRenderPipeline']){
   const original=GPUDevice.prototype[name];
   GPUDevice.prototype[name]=function(...args){window.allocations[name]=(window.allocations[name]||0)+1;return original.apply(this,args);};
  }
 });
 await page.goto(`/web/gallery/example.html?id=${id}`);
 await expect(page.locator('body')).toHaveAttribute('data-status','running',{timeout:90000});
 await expect.poll(()=>page.locator('canvas').getAttribute('data-frames').then(Number)).toBeGreaterThan(5);
 const before=await page.evaluate(()=>({...window.allocations}));
 await page.evaluate(()=>window.app.orbit(20,10,0));
 await page.waitForTimeout(250);
 expect(await page.evaluate(()=>window.allocations)).toEqual(before);
 await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
});
