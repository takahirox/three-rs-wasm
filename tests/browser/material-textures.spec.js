import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.MATERIAL_TEXTURES_DPR||1)});
const controls={materials_arrays:[],clipping:[[0,false],[0,true],[1,false],[1,true],[2,true],[3,false],[4,1.1],[5,false],[5,true],[6,1.2]],materials_texture_manualmipmap:[],textures_anisotropy:[],textures_partialupdate:[]};
for(const [kind,parameters] of Object.entries(controls))test(`Material textures official rendering: ${kind}`,async({page},info)=>{
 test.setTimeout(240000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;window.modules=[];const fn=GPUDevice.prototype.createShaderModule;GPUDevice.prototype.createShaderModule=function(d){modules.push(d.code);return fn.call(this,d);};const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.goto(runtime==='reference'?`/reference/three-js/material-textures.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info,.texture-label{display:none!important}'});
  const frames=[];const settle=180;
  const capture=async(t,parameter=null,save=true)=>{
   const prev=await page.locator('canvas').getAttribute('data-frames');
   await page.evaluate(async({runtime,t,parameter})=>{if(parameter){const [i,v]=parameter;if(runtime==='reference')fixtureParameter(i,v);else app.tsl_parameter(i,Number(v));}if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t,parameter});
   if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   if(save)frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  };
  if(kind.includes("mipmap")||kind.includes("anisotropy"))for(let i=0;i<180;i++)await capture(0,null,false);
  for(const t of [0,.4,1,2])await capture(t);
  for(const parameter of parameters)await capture(2.25,parameter);
  if(['materials_arrays','clipping'].includes(kind)){
   await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
  }
  if(['materials_arrays','clipping'].includes(kind)){
   await page.mouse.move(256,256);await page.mouse.down({button:'right'});await page.mouse.move(276,266);await page.mouse.up({button:'right'});
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
   await page.mouse.wheel(0,100);await page.waitForTimeout(100);
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
  }
  if(['materials_texture_manualmipmap','textures_anisotropy'].includes(kind)) {
   await page.mouse.move(410,180);for(let i=0;i<180;i++)await capture(2.25,null,false);await capture(2.25);
  }
  if(kind!=='materials_texture_manualmipmap')await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await capture(2.5,null,false);await capture(2.5);images[runtime]=frames;
  writeFileSync(info.outputPath(`${runtime}-shaders.json`),JSON.stringify(await page.evaluate(()=>modules)));
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
});

for(const kind of Object.keys(controls)){const id=`webgpu_${kind}`;
 test(`Material textures GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{for(const t of [0,1,2,4,0]){let prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);} for(const [i,v] of controls[kind]){const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(([i,v])=>app.tsl_parameter(i,Number(v)),[i,v]);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}};
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas').first()).not.toHaveAttribute('data-error',/.+/);
 });
}




test('Material textures GPU workload and region uploads',async({page},info)=>{
 test.setTimeout(120000);
 await page.addInitScript(()=>{
  window.resetWork=()=>window.work={passes:0,draws:[],attributeBytes:0,textureBytes:0,copies:[]};resetWork();
  const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(key==='drawIndexed'||a[0]>6)work.draws.push([a[0],a[1]??1]);return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX))work.attributeBytes+=size??data.byteLength;return write.apply(this,arguments);};
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
  const copy=GPUCommandEncoder.prototype.copyTextureToTexture;GPUCommandEncoder.prototype.copyTextureToTexture=function(a,b,size){work.copies.push({width:size.width??size[0],height:size.height??size[1]});return copy.apply(this,arguments);};
 });
 const report=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/material-textures.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   for(let t=1;t<=5;t++){const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(async({runtime,t})=>{if(t===5)resetWork();if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t});if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  report.push(pair);const sort=a=>a.sort((a,b)=>a[0]-b[0]||a[1]-b[1]);
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(pair.reference.draws));
  expect.soft(pair.rust.passes,kind).toBeLessThanOrEqual(pair.reference.passes);
  expect.soft(pair.rust.attributeBytes,kind).toBe(0);
  expect.soft(pair.rust.textureBytes,kind).toBe(kind==='textures_partialupdate'?4096:0);
  if(kind==='textures_partialupdate'){expect.soft(pair.rust.copies).toEqual([{width:32,height:32}]);expect.soft(pair.reference.textureBytes).toBe(4096);expect.soft(pair.reference.copies).toContainEqual({width:32,height:32});}
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});
