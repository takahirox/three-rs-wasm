import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const controls={mrt_mask:[],tsl_vfx_tornado:[[0,0x55aaff],[1,.5],[2,.8],[3,.5],[4,.4],[5,2],[6,.6]],compute_geometry:[[0,.2],[1,.96],[2,.4],[3,.3]],storage_buffer:[],postprocessing_bloom_selective:[[0,.5],[1,2.5],[2,.8],[3,1.5]],postprocessing_bloom:[[0,.8],[1,2],[2,.8],[3,1.2]],postprocessing_bloom_emissive:[[0,4],[1,.8],[2,1.4]],mrt:[],lights_selective:[[0,.1],[1,.9]],lights_phong:[],custom_fog_background:[],tsl_vfx_flames:[],shadertoy:[],lights_rectarealight:[],depth_texture:[],multiple_rendertargets:[],lights_custom:[],skinning:[],tsl_halftone:[[0,1],[1,5],[3,80],[7,.75],[8,.2],[9,.2],[10,.7],[11,.4],[13,120],[22,0x22aaff]],tsl_raging_sea:[[1,.8],[3,-.4],[4,.4],[5,3],[6,2],[7,.1],[8,5],[9,2],[10,5],[11,4],[12,.5],[13,.1],[14,.025]]};
for(const kind of Object.keys(controls)){
 const id=`webgpu_${kind}`;
 test(`TSL surface official rendering and controls: ${kind}`,async({page},info)=>{
  test.setTimeout(180000);await page.addInitScript(()=>{addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(-10,-10);
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-surface.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>{if(window.fixtureError)throw Error(window.fixtureError);if(document.querySelector('#notice')?.textContent.includes('GPU error:'))throw Error(document.querySelector('#notice').textContent);const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:60000});
   await page.addStyleTag({content:'html,body{background:#000}#notice,#settings{display:none!important}'});
   const frames=[];const counts=[];const capture=async(t,parameter=null,save=true)=>{
    const previous=await page.locator('canvas').getAttribute('data-frames');
    await page.evaluate(async({runtime,t,parameter})=>{
     if(parameter){const [index,value]=parameter;if(runtime==='reference')fixtureParameter(index,value);else{const input=document.querySelector(`#particle-${index}`);if(input.type==='checkbox')input.checked=value;else input.value=input.type==='color'?'#'+value.toString(16).padStart(6,'0'):value;input.dispatchEvent(new Event('input',{bubbles:true}));}}
     if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);
    },{runtime,t,parameter});
    if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);
    if(save){frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));counts.push(await page.evaluate(()=>window.fixtureFrames??Number(document.querySelector('canvas').dataset.frames)));}
   };
   for(const t of [0,.4,1,2])await capture(t);
   if(kind==='postprocessing_bloom_selective'){
    await page.mouse.click(320,64);await capture(2);
    expect(frames[4].data.equals(frames[3].data),'click changes selected bloom').toBe(false);
    await page.mouse.click(320,64);await capture(2);
   }
   if(kind==='mrt_mask'){
    await page.mouse.move(10,10);await page.mouse.down();await capture(2.5);
    await page.mouse.up();await capture(3);
   }
   for(const parameter of controls[kind])await capture(2.25,parameter);
   if(['lights_rectarealight','tsl_raging_sea','tsl_halftone','depth_texture','multiple_rendertargets','lights_custom','tsl_vfx_flames','custom_fog_background','lights_selective','lights_phong','mrt','postprocessing_bloom','postprocessing_bloom_emissive','postprocessing_bloom_selective','compute_geometry','tsl_vfx_tornado','mrt_mask'].includes(kind)){
    {await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();}
    if(kind==='compute_geometry')await page.mouse.move(10,10);
    // Match settled input, independent of each backend's event/rAF scheduling.
    for(let i=0;i<240;i++){
     const previous=await page.locator('canvas').getAttribute('data-frames');
     await page.evaluate(async runtime=>{if(runtime==='reference')await renderFixture(.1);else app.gallery_time(.1);},runtime);
     if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);
    }
    await capture(.1);
    if(kind==='compute_geometry'){
     // Set a fresh ray after the camera has rendered; avoid upstream's stale
     // matrix during an Orbit event and compare the same GPU simulation steps.
     await page.mouse.move(256,256);
     for(let i=0;i<60;i++)await capture(.1,null,false);
     await capture(.1);await page.mouse.move(10,10);
    }
   }
   await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);
   // Paused renders caused by input/resize do not advance GPU simulation.
   await capture(.2);images[runtime]=frames;writeFileSync(info.outputPath(`${runtime}-frames.json`),JSON.stringify(counts));
  }
  const results=[];
  for(let state=0;state<images.rust.length;state++){
   const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let count=0,sum=0;
   for(let p=0;p<a.data.length;p+=4){let bad=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;bad ||= d>6;}if(bad)count++;}
   results.push({state,fraction:count/(a.width*a.height),meanError:sum/(a.width*a.height*3)});
   writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));
  }
  writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
  for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
 });
 test(`TSL surface GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{for(const t of [0,1,2,4,0]){let prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}};
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
 });
}



test('TSL surface GPU workload and resident attributes',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,largeUniformBytes:0};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(...args){work.dispatches.push([args[0],args[1]??1,args[2]??1]);return dispatch.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const original=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if(args[0]>6||(args[1]??1)>1)work.draws.push({vertices:args[0],instances:args[1]??1});return original.apply(this,args);};}
  const indirect=GPURenderPassEncoder.prototype.drawIndexedIndirect;GPURenderPassEncoder.prototype.drawIndexedIndirect=function(...args){work.indirect++;return indirect.apply(this,args);};
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,offset,data,dataOffset,size){const bytes=size===undefined?data.byteLength-(dataOffset??0)*(data.BYTES_PER_ELEMENT??1):size*(data.BYTES_PER_ELEMENT??1);if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX|GPUBufferUsage.INDIRECT)){work.attributeBytes+=bytes;work.attributeWrites.push({label:buffer.label,size:buffer.size,bytes});}if(buffer.usage&GPUBufferUsage.UNIFORM){work.uniformBytes+=bytes;if(buffer.size>=16000)work.largeUniformBytes+=bytes;}return write.apply(this,arguments);};
 });
 const reports=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-surface.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   let tick=0;const step=async(reset=false)=>{const time=(tick++)+0.5;const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(async({runtime,reset,time})=>{if(reset)work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,largeUniformBytes:0};if(runtime==='reference')await renderFixture(time);else app.gallery_time(time);},{runtime,reset,time});if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   // Visit the measured visibility state first, then rewind and animate into it.
   for(let i=0;i<4;i++)await step();tick=2;await step();await step(true);pair[runtime]=await page.evaluate(()=>work);
  }
  reports.push(pair);
  const sort=rows=>rows.sort((a,b)=>a.vertices-b.vertices||a.instances-b.instances);
  // Three.js draws its background on a 5,952-index sphere. Our background
  // uses a fullscreen triangle/quad and is compared in the image tests.
  const background=['mrt_mask','skinning','compute_geometry','mrt','postprocessing_bloom_emissive','custom_fog_background'].includes(kind);
  const geometry=draws=>draws.filter(d=>!(background&&d.vertices===5952)).map(d=>kind==='lights_custom'?{vertices:d.vertices*d.instances,instances:1}:d);
  expect.soft(sort(geometry(pair.rust.draws)),kind).toEqual(sort(geometry(pair.reference.draws)));
  expect.soft(pair.rust.indirect,kind).toBe(pair.reference.indirect);
  expect.soft(pair.rust.dispatches,kind).toEqual(pair.reference.dispatches);
  // Record scene/postprocess/presentation pass counts separately from timing.
  if(['skinning','mrt_mask'].includes(kind))expect.soft(pair.rust.attributeBytes,kind).toBeLessThanOrEqual(65536);else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});
