import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
for(const [kind,example] of [['anisotropy',30],['sheen',31],['transmission',32],['dispersion',33],['compressed',34]]){
 const id=`webgpu_loader_gltf_${kind}`;
 test(`physical glTF views and controls: ${kind}`,async({page},info)=>{
  test.setTimeout(180000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));
  const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));expect(catalog.examples.find(e=>e.id===id)?.port?.example).toBe(example);
  const images={},compressedResources={};
  if(kind==='compressed')await page.addInitScript(()=>{
   window.compressedTextures=[];
   const create=GPUDevice.prototype.createTexture,write=GPUQueue.prototype.writeTexture,records=new WeakMap();
   GPUDevice.prototype.createTexture=function(d){const texture=create.call(this,d);if(/^(bc|etc2|astc)/.test(d.format)){const r={format:d.format,width:texture.width,height:texture.height,levels:texture.mipLevelCount,bytes:0,writes:0};compressedTextures.push(r);records.set(texture,r);}return texture;};
   GPUQueue.prototype.writeTexture=function(destination,data,...rest){const r=records.get(destination.texture);if(r){r.bytes+=data.byteLength;r.writes++;}return write.call(this,destination,data,...rest);};
  });
  for(const runtime of ['reference','rust']){
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/gltf-examples.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true'||Number(document.querySelector('canvas')?.dataset.frames)>2,null,{timeout:90000});
   if(runtime==='rust')await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
   const states=[];
   const actions=[async()=>{},async()=>page.evaluate(runtime=>runtime==='reference'?renderFixture(1):app.gallery_time(1),runtime),async()=>page.evaluate(runtime=>runtime==='reference'?renderFixture(3):app.gallery_time(3),runtime),
    async()=>{await page.mouse.move(250,250);await page.mouse.down();await page.mouse.move(285,262,{steps:5});await page.mouse.up();},
    async()=>{await page.mouse.move(250,250);await page.mouse.down({button:'right'});await page.mouse.move(240,255,{steps:3});await page.mouse.up({button:'right'});},
    async()=>page.mouse.wheel(0,100),async()=>page.setViewportSize({width:640,height:400})];
   if(kind==='sheen')for(const value of [0,.4,1])actions.push(async()=>page.evaluate(({runtime,value})=>{if(runtime==='rust')app.gallery_sheen(value);else{reference.scene.getObjectByName('SheenChair_fabric').material.sheen=value;reference.renderer.render(reference.scene,reference.camera);}}, {runtime,value}));
   for(const action of actions){await action();await page.waitForTimeout(['anisotropy','dispersion','compressed'].includes(kind)?120:2800);states.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
   images[runtime]=states;
   if(kind==='compressed')compressedResources[runtime]=await page.evaluate(()=>compressedTextures);
  }
  const results=[];
  for(let state=0;state<images.rust.length;state++){
   const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let count=0;
   for(let i=0;i<a.data.length;i+=4)if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))count++;
   const fraction=count/(a.width*a.height);results.push({state,fraction});
   writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));
  }
  if(kind==='compressed'){
   writeFileSync(info.outputPath('compressed-resources.json'),JSON.stringify(compressedResources,null,2));
   // Rust shares the same MR/AO image; upstream may upload that image twice.
   const layout=rows=>[...new Set(rows.map(r=>JSON.stringify(r)))].sort();
   expect(compressedResources.rust.length).toBeGreaterThan(0);
   expect(layout(compressedResources.rust)).toEqual(layout(compressedResources.reference));
   expect(compressedResources.rust.reduce((n,r)=>n+r.bytes,0)).toBeLessThanOrEqual(compressedResources.reference.reduce((n,r)=>n+r.bytes,0));
  }
  writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
  for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
 });
 test(`physical glTF GPU residency and resize: ${kind}`,async({page})=>{
  const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));expect(catalog.examples.find(e=>e.id===id)?.port?.example).toBe(example);
  const errors=[];page.on('pageerror',e=>errors.push(String(e)));
  await page.addInitScript(()=>{window.creates=0;for(const n of ['createBuffer','createTexture','createBindGroup','createRenderPipeline']){const f=GPUDevice.prototype[n];GPUDevice.prototype[n]=function(...a){creates++;return f.apply(this,a);};}});
  await page.goto(`/web/gallery/example.html?id=${id}`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>10,null,{timeout:90000});
  for(const resized of [false,true]){
   if(resized){await page.setViewportSize({width:640,height:400});await page.waitForTimeout(250);}
   const before=await page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));await page.waitForTimeout(350);
   expect(await page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}))).toEqual(before);
  }
  expect(errors).toEqual([]);await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
 });
}
