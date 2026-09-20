import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const controls={particles:[[0,.1],[0,.8]],instance_mesh:[[0,500],[0,1000]],compute_points:[[0,.5],[1,.7]],compute_particles:[[0,-.001],[1,.6],[2,.98],[3,.3]],compute_texture_pingpong:[]};
for(const kind of Object.keys(controls)){
 const id=`webgpu_${kind}`;
 test(`TSL compute official rendering and controls: ${kind}`,async({page},info)=>{
  test.setTimeout(180000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(-10,-10);
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-compute.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:60000});
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
   for(const parameter of controls[kind])await capture(2.25,parameter);
   if(kind==='compute_texture_pingpong'){for(let i=0;i<60;i++)await capture(2.1,null,false);await capture(2.1);}
   if(['particles','compute_particles','compute_points'].includes(kind)){
    if(kind==='compute_points')await page.mouse.move(360,200);
    else {await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();}
    // Match settled input, independent of each backend's event/rAF scheduling.
    for(let i=0;i<240;i++){
     const previous=await page.locator('canvas').getAttribute('data-frames');
     await page.evaluate(async runtime=>{if(runtime==='reference')await renderFixture(.1);else app.gallery_time(.1);},runtime);
     if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);
    }
    await capture(.1);
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
 test(`TSL compute GPU residency: ${kind}`,async({page},info)=>{
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



test('GPU compute/instancing workload matches upstream and keeps storage resident',async({page},info)=>{
 test.setTimeout(90000);
 await page.addInitScript(()=>{
  window.work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,uniformBytes:0,largeUniformBytes:0};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(...args){work.dispatches.push([args[0],args[1]??1,args[2]??1]);return dispatch.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const original=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if((args[1]??1)>1)work.draws.push({vertices:args[0],instances:args[1]});return original.apply(this,args);};}
  const indirect=GPURenderPassEncoder.prototype.drawIndexedIndirect;GPURenderPassEncoder.prototype.drawIndexedIndirect=function(...args){work.indirect++;return indirect.apply(this,args);};
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,offset,data,dataOffset,size){const bytes=size===undefined?data.byteLength-(dataOffset??0)*(data.BYTES_PER_ELEMENT??1):size*(data.BYTES_PER_ELEMENT??1);if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX|GPUBufferUsage.INDIRECT))work.attributeBytes+=bytes;if(buffer.usage&GPUBufferUsage.UNIFORM){work.uniformBytes+=bytes;if(buffer.size>=16000)work.largeUniformBytes+=bytes;}return write.apply(this,arguments);};
 });
 const reports=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-compute.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   let tick=0;const step=async(reset=false)=>{const time=.5+(tick++)*.01;const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(async({runtime,reset,time})=>{if(reset)work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,uniformBytes:0,largeUniformBytes:0};if(runtime==='reference')await renderFixture(time);else app.gallery_time(time);},{runtime,reset,time});if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   for(let i=0;i<3;i++)await step();await step(true);pair[runtime]=await page.evaluate(()=>work);
  }
  reports.push(pair);
  expect(pair.rust.draws,kind).toEqual(pair.reference.draws);
  expect(pair.rust.indirect,kind).toBe(pair.reference.indirect);
  expect(pair.rust.dispatches,kind).toEqual(pair.reference.dispatches);
  expect(pair.rust.passes,kind).toBe(pair.reference.passes);
  if(kind==='instance_mesh'){expect(pair.rust.attributeBytes).toBeGreaterThan(0);expect(pair.rust.attributeBytes).toBeLessThanOrEqual(80000);}else expect(pair.rust.attributeBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});
