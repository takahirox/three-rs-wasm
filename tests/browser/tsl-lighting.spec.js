import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.TSL_LIGHTING_DPR||1)});
const controls={pmrem_cubemap:[],pmrem_scene:[[0,0],[0,1],[0,.5]],materials_lightmap:[[0,0],[0,1]],postprocessing_dof_basic:[[0,0],[1,5],[2,1],[3,7]],postprocessing_lensflare:[[0,0],[0,2],[1,0],[2,0],[3,10],[4,.1],[5,2]]};
for(const [kind,parameters] of Object.entries(controls))test(`TSL lighting official rendering: ${kind}`,async({page},info)=>{
 test.setTimeout(240000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;window.modules=[];const fn=GPUDevice.prototype.createShaderModule;GPUDevice.prototype.createShaderModule=function(d){modules.push(d.code);return fn.call(this,d);};const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.goto(runtime==='reference'?`/reference/three-js/tsl-lighting.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const frames=[];
  const capture=async(t,parameter=null,save=true)=>{
   const prev=await page.locator('canvas').getAttribute('data-frames');
   await page.evaluate(async({runtime,t,parameter})=>{if(parameter){const [i,v]=parameter;if(runtime==='reference')fixtureParameter(i,v);else app.tsl_parameter(i,Number(v));}if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t,parameter});
   if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   if(save)frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  };
  for(const t of [0,.4,1,2])await capture(t);
  for(const parameter of parameters)await capture(2.25,parameter);
  {
   await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();
   for(let i=0;i<180;i++)await capture(2.25,null,false);await capture(2.25);
  }
  {
   await page.mouse.move(256,256);await page.mouse.down({button:'right'});await page.mouse.move(276,266);await page.mouse.up({button:'right'});
   for(let i=0;i<180;i++)await capture(2.25,null,false);await capture(2.25);
   await page.mouse.wheel(0,100);await page.waitForTimeout(100);
   for(let i=0;i<180;i++)await capture(2.25,null,false);await capture(2.25);
  }
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await capture(2.5);images[runtime]=frames;
  writeFileSync(info.outputPath(`${runtime}-shaders.json`),JSON.stringify(await page.evaluate(()=>modules)));
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
});

for(const kind of Object.keys(controls)){const id=`webgpu_${kind}`;
 test(`TSL lighting GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{for(const t of [0,1,2,4,0]){let prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}};
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas').first()).not.toHaveAttribute('data-error',/.+/);
 });
}

test('TSL lighting GPU workload and resident attributes',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,uniformWrites:[],largeUniformBytes:0};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(...args){work.dispatches.push([args[0],args[1]??1,args[2]??1]);return dispatch.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const original=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if(args[0]>6||(args[1]??1)>1)work.draws.push({vertices:args[0],instances:args[1]??1});return original.apply(this,args);};}
  for(const key of ['drawIndexedIndirect','drawIndirect']){const indirect=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){work.indirect++;return indirect.apply(this,args);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,offset,data,dataOffset,size){const bytes=size===undefined?data.byteLength-(dataOffset??0)*(data.BYTES_PER_ELEMENT??1):size*(data.BYTES_PER_ELEMENT??1);if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX|GPUBufferUsage.INDIRECT)){work.attributeBytes+=bytes;work.attributeWrites.push({label:buffer.label,size:buffer.size,bytes});}if(buffer.usage&GPUBufferUsage.UNIFORM){work.uniformBytes+=bytes;work.uniformWrites.push({label:buffer.label,size:buffer.size,bytes});if(buffer.size>=16000)work.largeUniformBytes+=bytes;}return write.apply(this,arguments);};
 });
 const reports=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-lighting.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   let tick=0;const step=async(reset=false)=>{const time=(tick++)+0.5;const prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(async({runtime,reset,time})=>{if(reset)work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,uniformWrites:[],largeUniformBytes:0};if(runtime==='reference')await renderFixture(time);else app.gallery_time(time);},{runtime,reset,time});if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   // Visit the measured visibility state first, then rewind and animate into it.
   for(let i=0;i<4;i++)await step();tick=2;await step();await step(true);pair[runtime]=await page.evaluate(()=>work);
  }
  reports.push(pair);
  const sort=rows=>rows.sort((a,b)=>a.vertices-b.vertices||a.instances-b.instances);
  // The port renders these sky backgrounds with a fullscreen triangle;
  // Three.js uses a 5,952-index sphere. All scene mesh draws must still match.
  const referenceDraws=['pmrem_cubemap','postprocessing_lensflare'].includes(kind)
   ? pair.reference.draws.filter(d=>d.vertices!==5952) : pair.reference.draws;
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(referenceDraws));
  expect.soft(pair.rust.indirect,kind).toBe(pair.reference.indirect);
  expect.soft(pair.rust.dispatches,kind).toEqual(pair.reference.dispatches);
  // Record scene/postprocess/presentation pass counts separately from timing.
  if(kind==='postprocessing_dof_basic'){
   // Five skinned primitives, 19 joints each. Only bone poses may be uploaded;
   // vertices, joint indices/weights and morph inputs remain on the GPU.
   expect.soft(pair.rust.attributeBytes,kind).toBe(5*19*64);
   for(const write of pair.rust.attributeWrites)expect.soft(write).toEqual({label:'skin/morph input',size:19*64,bytes:19*64});
  }else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});
