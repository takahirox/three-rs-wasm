import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.TSL_PROCEDURAL_DPR||1)});
const controls={upscaling_taau:[[0,0],[0,1],[1,.25],[1,1],[2,0],[2,1],[3,1.5]],postprocessing_motion_blur:[[0,0],[1,0],[1,2],[2,.5]],postprocessing_traa:[],materials_retroreflection:[[0,0],[0,1],[1,.3],[2,15]],compute_particles_rain:[[0,15],[1,1.5],[2,50000]],compute_particles_snow:[],postprocessing_pixel:[[0,8],[1,1],[2,.7],[3,0]],reflection_blurred:[[0,.3],[1,.7],[2,.75]],reflection:[],mirror:[],shadowmap:[],multiple_rendertargets_readback:[[0,1],[0,2]],tsl_procedural_terrain:[[0,5],[1,.23],[2,7],[3,4],[4,.4],[5,0xdbbd89],[9,.2],[10,1.5]],tsl_angular_slicing:[[0,-1],[1,3],[2,0x236fa2]],tsl_wood:[[0,1.5],[1,.5],[4,3],[6,20],[7,.04],[8,.12],[11,.6],[14,1200],[15,.2],[16,0x542820],[17,0xb29170],[19,.7]],skinning_points:[],portal:[],materialx_noise:[],reflection_roughness:[]};
for(const [kind,parameters] of Object.entries(controls))test(`TSL procedural official rendering: ${kind}`,async({page},info)=>{
 test.setTimeout(240000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;window.modules=[];const fn=GPUDevice.prototype.createShaderModule;GPUDevice.prototype.createShaderModule=function(d){modules.push(d.code);return fn.call(this,d);};const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.goto(runtime==='reference'?`/reference/three-js/tsl-procedural.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const frames=[];const settle=180;
  const capture=async(t,parameter=null,save=true)=>{
   const prev=await page.locator('canvas').getAttribute('data-frames');
   const rb=Number(await page.locator("canvas").getAttribute("data-readbacks")||0);
   await page.evaluate(async({runtime,t,parameter})=>{if(parameter){const [i,v]=parameter;if(runtime==='reference')fixtureParameter(i,v);else app.tsl_parameter(i,Number(v));}if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t,parameter});
   if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);
   if(runtime==='rust'&&kind==='multiple_rendertargets_readback'&&(parameter||t>=2.25))await page.waitForFunction(n=>Number(document.querySelector('canvas').dataset.readbacks)>=n+1,rb);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   if(save)frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  };
  if(['postprocessing_traa','upscaling_taau'].includes(kind))for(let i=0;i<128;i++)await capture(0,null,false);
  for(const t of [0,.4,1,2])await capture(t);
  for(const parameter of parameters)await capture(2.25,parameter);
  // Exercise moving history, including GPU-skinned motion, at frame intervals.
  if(['postprocessing_traa','postprocessing_motion_blur','upscaling_taau'].includes(kind))
   for(let frame=1;frame<=60;frame++)await capture(2.25+frame/60,null,[1,16,60].includes(frame));
  if(!['skinning_points','postprocessing_traa'].includes(kind)){
   await page.mouse.move(256,256);if(kind==='tsl_procedural_terrain')await capture(2.25,null,false);await page.mouse.down();await page.mouse.move(296,276);if(kind==='tsl_procedural_terrain')await capture(2.25,null,false);await page.mouse.up();
   // Rust redraws once for pointer input, so advance the reference Halton phase too.
   if(kind==='upscaling_taau'&&runtime==='reference')await capture(2.25,null,false);
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
  }
  if(!['skinning_points','postprocessing_traa'].includes(kind)){
   await page.mouse.move(256,256);if(kind==='tsl_procedural_terrain')await capture(2.25,null,false);await page.mouse.down({button:'right'});await page.mouse.move(276,266);if(kind==='tsl_procedural_terrain')await capture(2.25,null,false);await page.mouse.up({button:'right'});
   // Rust redraws once for pointer input, so advance the reference Halton phase too.
   if(kind==='upscaling_taau'&&runtime==='reference')await capture(2.25,null,false);
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
   await page.mouse.wheel(0,100);await page.waitForTimeout(100);
   // Rust redraws once for pointer input, so advance the reference Halton phase too.
   if(kind==='upscaling_taau'&&runtime==='reference')await capture(2.25,null,false);
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
  }
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);if(['postprocessing_traa','upscaling_taau'].includes(kind)&&runtime==='reference')await capture(2,null,false);await capture(2.5,null,false);if(['postprocessing_traa','upscaling_taau'].includes(kind))for(let i=0;i<128;i++)await capture(2.5,null,false);await capture(2.5);images[runtime]=frames;
  writeFileSync(info.outputPath(`${runtime}-shaders.json`),JSON.stringify(await page.evaluate(()=>modules)));
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}const metric={state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};
 if(kind==='materials_retroreflection'){
  let filteredBad=0;
  for(let y=1;y<a.height-1;y++)for(let x=1;x<a.width-1;x++){
   let fail=false;for(let c=0;c<3;c++){let delta=0;for(let dy=-1;dy<=1;dy++)for(let dx=-1;dx<=1;dx++){const i=((y+dy)*a.width+x+dx)*4+c;delta+=a.data[i]-b.data[i];}fail ||=Math.abs(delta)>6*9;}if(fail)filteredBad++;
  }
  metric.filteredFraction=filteredBad/((a.width-2)*(a.height-2));
 }
 results.push(metric);writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);for(const r of results){
  // Float32 UV sensitivity of r186 hashBlur is measured independently by
  // tools/tsl/check-retro-precision.mjs; keep both raw and spatial error bounded.
  if(kind==='materials_retroreflection'&&Number(process.env.TSL_PROCEDURAL_DPR||1)>1){
   expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.01);
   expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.5);
   expect(r.filteredFraction,JSON.stringify(r)).toBeLessThanOrEqual(.001);
  }else expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
 }
});

for(const kind of Object.keys(controls)){const id=`webgpu_${kind}`;
 test(`TSL procedural GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>{const c=document.querySelector('canvas'),error=c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return Number(c?.dataset.frames)>0;});
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{for(const t of [0,1,2,4,0]){let prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);} for(const [i,v] of (kind==='upscaling_taau'?[]:controls[kind])){const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(([i,v])=>app.tsl_parameter(i,Number(v)),[i,v]);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}};
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
   if(kind==='upscaling_taau')for(const [i,v] of controls[kind]){
    const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(([i,v])=>app.tsl_parameter(i,v),[i,v]);await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);
    await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,parameter:[i,v],before,after});expect(after).toEqual(before);
   }

  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas').first()).not.toHaveAttribute('data-error',/.+/);
 });
}




test('TSL procedural GPU workload and resident attributes',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,uniformWrites:[],largeUniformBytes:0};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(...args){work.dispatches.push([args[0],args[1]??1,args[2]??1]);return dispatch.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const original=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if(key==='drawIndexed'||args[0]>6||(args[1]??1)>1)work.draws.push({vertices:args[0],instances:args[1]??1});return original.apply(this,args);};}
  for(const key of ['drawIndexedIndirect','drawIndirect']){const indirect=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){work.indirect++;return indirect.apply(this,args);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,offset,data,dataOffset,size){const bytes=size===undefined?data.byteLength-(dataOffset??0)*(data.BYTES_PER_ELEMENT??1):size*(data.BYTES_PER_ELEMENT??1);if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX|GPUBufferUsage.INDIRECT)){work.attributeBytes+=bytes;work.attributeWrites.push({label:buffer.label,size:buffer.size,bytes});}if(buffer.usage&GPUBufferUsage.UNIFORM){work.uniformBytes+=bytes;work.uniformWrites.push({label:buffer.label,size:buffer.size,bytes});if(buffer.size>=16000)work.largeUniformBytes+=bytes;}return write.apply(this,arguments);};
 });
 const reports=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-procedural.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   let tick=0;const step=async(reset=false)=>{const time=(tick++)+0.5;const prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(async({runtime,reset,time})=>{if(reset)work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,uniformWrites:[],largeUniformBytes:0};if(runtime==='reference')await renderFixture(time);else app.gallery_time(time);},{runtime,reset,time});if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   // Visit the measured visibility state first, then rewind and animate into it.
   for(let i=0;i<4;i++)await step();tick=2;await step();await step(true);pair[runtime]=await page.evaluate(()=>work);
  }
  reports.push(pair);
  // Geometry is resident; bone palettes are the only animated storage uploads.
  const boneBytes={portal:17152,skinning_points:4160,reflection_blurred:4160,postprocessing_motion_blur:17152,upscaling_taau:32768};
  expect.soft(pair.rust.attributeBytes,kind).toBe(boneBytes[kind]||0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});

test('MRT readback transfers only the selected resident attachment',async({page},info)=>{
 await page.addInitScript(()=>{
  window.readbackWork={copies:[],uploads:[],buffers:0};
  const create=GPUDevice.prototype.createBuffer;GPUDevice.prototype.createBuffer=function(...a){readbackWork.buffers++;return create.apply(this,a);};
  const copy=GPUCommandEncoder.prototype.copyTextureToBuffer;GPUCommandEncoder.prototype.copyTextureToBuffer=function(a,b,size){readbackWork.copies.push(Array.isArray(size)?size:[size.width,size.height,size.depthOrArrayLayers??1]);return copy.apply(this,arguments);};
  const upload=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(a,data,layout,size){readbackWork.uploads.push(data.byteLength);return upload.apply(this,arguments);};
 });
 const reports=[];
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/tsl-procedural.html?id=webgpu_multiple_rendertargets_readback':'/web/gallery/example.html?id=webgpu_multiple_rendertargets_readback&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
  for(const selection of [1,2]){
   const step=async(reset=false)=>{
    const before=Number(await page.locator('canvas').getAttribute('data-readbacks')||0);
    await page.evaluate(async({runtime,selection,reset})=>{
     if(reset)readbackWork={copies:[],uploads:[],buffers:0};
     if(runtime==='reference'){fixtureParameter(0,selection);await renderFixture(1);}else{app.tsl_parameter(0,selection);app.gallery_time(1);}
    },{runtime,selection,reset});
    if(runtime==='rust')await page.waitForFunction(n=>Number(document.querySelector('canvas').dataset.readbacks)>n,before);
   };
   await step();await step(true);const work=await page.evaluate(()=>readbackWork);reports.push({runtime,selection,...work});
   expect(work.copies).toEqual([[512,512,1]]);expect(work.uploads).toEqual([512*512*4]);
   if(runtime==='rust')expect(work.buffers).toBe(0);
  }
 }
 writeFileSync(info.outputPath('mrt-readback-work.json'),JSON.stringify(reports,null,2));
});
