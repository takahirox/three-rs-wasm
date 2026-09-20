import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.TSL_EXTENDED_DPR||1)});

test.beforeEach(async({page})=>{
 await page.addInitScript(()=>{window.gpuValidationErrors=[];const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...args){const device=await request.apply(this,args);device.addEventListener('uncapturederror',e=>{gpuValidationErrors.push(e.error.message);window.fixtureError=e.error.message;});return device;};});
});
test.afterEach(async({page})=>{if(!page.isClosed())expect(await page.evaluate(()=>window.gpuValidationErrors??[])).toEqual([]);});

const controls={materials_cubemap_mipmaps:[],"rendertarget_2d-array_3d":[],instance_points:[[0,0],[0,1],[1,10],[2,25],[3,12]],struct_drawindirect:[],multiple_canvas:[],multiple_elements:[],postprocessing_dof:[[0,1000],[1,400],[2,5]],instance_uniform:[],occlusion:[],tsl_earth:[[0,0xff8080],[1,0x8040ff],[2,.1],[3,.8]],postprocessing_anamorphic:[[0,2],[1,.6],[2,32],[3,0xff8080],[4,.5],[5,.2]],texturegather:[],centroid_sampling:[[0,1],[0,2],[0,3],[0,4],[0,0]],"textures_2d-array_compressed":[],compute_texture_3d:[[0,.15],[1,.2],[2,.15],[3,150]],volume_perlin:[[0,.5],[1,100],[2,0],[2,1]],volume_cloud:[[0,.4],[1,.5],[2,.2],[3,150]],"textures_2d-array":[],multisampled_renderbuffers:[[0,0],[1,0],[0,1],[1,1]],layers:[[0,0],[1,0],[2,0],[0,1],[1,1],[2,1]]};
for(const kind of Object.keys(controls)){
 const id=`webgpu_${kind}`;
 test(`TSL extended official rendering and controls: ${kind}`,async({page},info)=>{
  test.setTimeout(180000);await page.addInitScript(()=>{window.fixtureModules=[];window.fixturePipelines=[];const shader=GPUDevice.prototype.createShaderModule;GPUDevice.prototype.createShaderModule=function(d){fixtureModules.push(d.code);return shader.call(this,d);};const pipeline=GPUDevice.prototype.createRenderPipeline;GPUDevice.prototype.createRenderPipeline=function(d){fixturePipelines.push(JSON.parse(JSON.stringify({label:d.label,multisample:d.multisample,targets:d.fragment?.targets,depth:d.depthStencil})));return pipeline.call(this,d);};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(-10,-10);
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-extended.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>{if(window.fixtureError)throw Error(window.fixtureError);if(document.querySelector('#notice')?.textContent.includes('GPU error:'))throw Error(document.querySelector('#notice').textContent);const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:60000});
   await page.addStyleTag({content:(['multiple_elements','multiple_canvas'].includes(kind)?'':'html,body{background:#000}')+'*{-webkit-font-smoothing:antialiased!important}#notice,#settings,.viewport-label{display:none!important}'});
   if(['multiple_elements','multiple_canvas'].includes(kind))writeFileSync(info.outputPath(`${runtime}-styles.json`),JSON.stringify(await page.evaluate(()=>{const e=document.querySelector('.list-item>div:nth-child(2)'),s=getComputedStyle(e);return {font:s.font,weight:s.fontWeight,rect:e.getBoundingClientRect().toJSON(),body:getComputedStyle(document.body).font}})));
   const frames=[];const counts=[];const capture=async(t,parameter=null,save=true)=>{
    const previous=await page.locator('canvas').first().getAttribute('data-frames');
    await page.evaluate(async({runtime,t,parameter})=>{
     if(parameter){const [index,value]=parameter;if(runtime==='reference')fixtureParameter(index,value);else{const input=document.querySelector(`#particle-${index}`);if(input.type==='checkbox')input.checked=value;else input.value=input.type==='color'?'#'+value.toString(16).padStart(6,'0'):value;input.dispatchEvent(new Event('input',{bubbles:true}));}}
     if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);
    },{runtime,t,parameter});
    if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);
    if(save){frames.push(PNG.sync.read(await (['multiple_elements','multiple_canvas'].includes(kind)?page.screenshot(): (kind==='centroid_sampling'&&runtime==='reference'?page.locator('body'):page.locator('canvas')).screenshot())));counts.push(await page.evaluate(()=>window.fixtureFrames??Number(document.querySelector('canvas').dataset.frames)));}
   };
   if(kind==='occlusion')for(let i=0;i<8;i++)await capture(0,null,false);
   for(const t of [0,.4,1,2])await capture(t);
   if(['multiple_elements','multiple_canvas'].includes(kind)){
    await page.mouse.move(120,150);await page.mouse.down();await page.mouse.move(140,160);await page.mouse.up();await capture(2);
    await page.evaluate(()=>scrollTo(0,160));await page.waitForTimeout(50);await capture(2);
   }
   for(const parameter of controls[kind])await capture(2.25,parameter);
   if(['materials_cubemap_mipmaps','rendertarget_2d-array_3d','instance_points','struct_drawindirect','postprocessing_dof','instance_uniform','occlusion','tsl_earth','postprocessing_anamorphic','compute_texture_3d','volume_perlin','volume_cloud'].includes(kind)){
    await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();
    if(kind==='occlusion')for(let i=0;i<8;i++)await capture(.1,null,false);
    if(['rendertarget_2d-array_3d','instance_points','tsl_earth','postprocessing_dof'].includes(kind))for(let i=0;i<240;i++)await capture(.1,null,false);
    await capture(.1);
    await page.mouse.move(256,256);await page.mouse.down({button:'right'});await page.mouse.move(276,266);await page.mouse.up({button:'right'});
    if(['rendertarget_2d-array_3d','instance_points','tsl_earth','postprocessing_dof'].includes(kind))for(let i=0;i<240;i++)await capture(.1,null,false);
    if(kind==='occlusion')for(let i=0;i<8;i++)await capture(.1,null,false);
    await capture(.1);
   }
   await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);
   // Paused renders caused by input/resize do not advance GPU simulation.
   if(kind==='postprocessing_dof')await capture(.2,null,false);
   await capture(.2);if(['instance_points','rendertarget_2d-array_3d'].includes(kind))writeFileSync(info.outputPath(`${runtime}-shaders.json`),JSON.stringify(await page.evaluate(()=>({modules:fixtureModules,pipelines:fixturePipelines}))));images[runtime]=frames;writeFileSync(info.outputPath(`${runtime}-frames.json`),JSON.stringify(counts));
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
 test(`TSL extended GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const reports=[];let layerTick=0;
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{for(const t of [0,1,2,4,0]){let prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),kind==='rendertarget_2d-array_3d'?++layerTick*.05:t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}};
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas').first()).not.toHaveAttribute('data-error',/.+/);
 });
}



test('TSL extended GPU workload and resident attributes',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,largeUniformBytes:0};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(...args){work.dispatches.push([args[0],args[1]??1,args[2]??1]);return dispatch.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const original=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if(args[0]>6||(args[1]??1)>1)work.draws.push({vertices:args[0],instances:args[1]??1});return original.apply(this,args);};}
  for(const key of ['drawIndexedIndirect','drawIndirect']){const indirect=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){work.indirect++;return indirect.apply(this,args);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,offset,data,dataOffset,size){const bytes=size===undefined?data.byteLength-(dataOffset??0)*(data.BYTES_PER_ELEMENT??1):size*(data.BYTES_PER_ELEMENT??1);if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX|GPUBufferUsage.INDIRECT)){work.attributeBytes+=bytes;work.attributeWrites.push({label:buffer.label,size:buffer.size,bytes});}if(buffer.usage&GPUBufferUsage.UNIFORM){work.uniformBytes+=bytes;if(buffer.size>=16000)work.largeUniformBytes+=bytes;}return write.apply(this,arguments);};
 });
 const reports=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-extended.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   let tick=0;const step=async(reset=false)=>{const time=(tick++)+0.5;const prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(async({runtime,reset,time})=>{if(reset)work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,largeUniformBytes:0};if(runtime==='reference')await renderFixture(time);else app.gallery_time(time);},{runtime,reset,time});if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   // Visit the measured visibility state first, then rewind and animate into it.
   for(let i=0;i<4;i++)await step();if(kind!=='rendertarget_2d-array_3d')tick=2;await step();await step(true);pair[runtime]=await page.evaluate(()=>work);
  }
  reports.push(pair);
  const sort=rows=>rows.sort((a,b)=>a.vertices-b.vertices||a.instances-b.instances);
  // Three.js draws its background on a 5,952-index sphere. Our background
  // uses a fullscreen triangle/quad and is compared in the image tests.
  const background=['layers','postprocessing_anamorphic','instance_points','rendertarget_2d-array_3d','multiple_canvas','multiple_elements'].includes(kind);
  const geometry=draws=>draws.filter(d=>!(background&&d.vertices===5952));
  expect.soft(sort(geometry(pair.rust.draws)),kind).toEqual(sort(geometry(pair.reference.draws)));
  expect.soft(pair.rust.indirect,kind).toBe(pair.reference.indirect);
  expect.soft(pair.rust.dispatches,kind).toEqual(pair.reference.dispatches);
  // Record scene/postprocess/presentation pass counts separately from timing.
  expect.soft(pair.rust.attributeBytes,kind).toBe(0);
  if(kind==='rendertarget_2d-array_3d'){expect(pair.rust.passes).toBeGreaterThan(40);expect(pair.reference.passes).toBeGreaterThan(40);}
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});
