import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.TSL_VIEWPORT_DPR||1)});
const controls={backdrop:[],backdrop_area:[[0,1.5],[1,.7],[2,'blurred'],[2,'depth'],[2,'pixel'],[2,'checker']],refraction:[],particles_soft:[[0,0],[0,1],[1,.2],[2,6]],upscaling_fsr1:[[0,'Bilinear'],[1,1],[0,'FSR1'],[1,.25]]};
for(const [kind,parameters] of Object.entries(controls))test(`TSL viewport official rendering: ${kind}`,async({page},info)=>{
 test.setTimeout(240000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;window.modules=[];const fn=GPUDevice.prototype.createShaderModule;GPUDevice.prototype.createShaderModule=function(d){modules.push(d.code);return fn.call(this,d);};const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.goto(runtime==='reference'?`/reference/three-js/tsl-viewport.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const frames=[];const settle=['particles_soft','upscaling_fsr1'].includes(kind)?360:180;
  const capture=async(t,parameter=null,save=true)=>{
   const prev=await page.locator('canvas').getAttribute('data-frames');
   await page.evaluate(async({runtime,t,parameter})=>{if(parameter){const [i,v]=parameter;if(runtime==='reference')fixtureParameter(i,v);else app.tsl_parameter(i,Number(typeof v==='string'?({blurred:0,checker:1,depth:2,pixel:3,Bilinear:0,FSR1:1}[v]):v));}if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t,parameter});
   if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   if(save)frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  };
  if(kind==='upscaling_fsr1'){await capture(.001,null,false);await capture(0,null,false);}
  for(const t of [0,.4,1,2])await capture(t);
  for(const parameter of parameters)await capture(2.25,parameter);
  {
   await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
  }
  {
   await page.mouse.move(256,256);await page.mouse.down({button:'right'});await page.mouse.move(276,266);await page.mouse.up({button:'right'});
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
   await page.mouse.wheel(0,100);await page.waitForTimeout(100);
   for(let i=0;i<settle;i++)await capture(2.25,null,false);await capture(2.25);
  }
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await capture(2.5);images[runtime]=frames;
  writeFileSync(info.outputPath(`${runtime}-shaders.json`),JSON.stringify(await page.evaluate(()=>modules)));
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
});

for(const kind of Object.keys(controls)){const id=`webgpu_${kind}`;
 test(`TSL viewport GPU residency: ${kind}`,async({page},info)=>{
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


test('TSL viewport GPU workload and resident attributes',async({page},info)=>{
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
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-viewport.html?id=webgpu_${kind}`:`/web/gallery/example.html?id=webgpu_${kind}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   let tick=0;const step=async(reset=false)=>{const time=(tick++)+0.5;const prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(async({runtime,reset,time})=>{if(reset)work={passes:0,dispatches:[],draws:[],indirect:0,attributeBytes:0,attributeWrites:[],uniformBytes:0,uniformWrites:[],largeUniformBytes:0};if(runtime==='reference')await renderFixture(time);else app.gallery_time(time);},{runtime,reset,time});if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   // Visit the measured visibility state first, then rewind and animate into it.
   for(let i=0;i<4;i++)await step();tick=2;await step();await step(true);pair[runtime]=await page.evaluate(()=>work);
  }
  reports.push(pair);
  const sort=rows=>rows.sort((a,b)=>a.vertices-b.vertices||a.instances-b.instances);
  // The port renders these sky backgrounds with a fullscreen triangle;
  // Three.js uses a 5,952-index sphere. All scene mesh draws must still match.
  const referenceDraws=['backdrop','backdrop_area'].includes(kind)
   ? pair.reference.draws.filter(d=>d.vertices!==5952) : pair.reference.draws;
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(referenceDraws));
  expect.soft(pair.rust.indirect,kind).toBe(pair.reference.indirect);
  expect.soft(pair.rust.dispatches,kind).toEqual(pair.reference.dispatches);
  // Record scene/postprocess/presentation pass counts separately from timing.
  if(['backdrop','backdrop_area','upscaling_fsr1'].includes(kind)){
   // Only the Michelle skeleton palette may change; no geometry/skin inputs stream.
   const bytes=kind==='upscaling_fsr1'?2048:4160;
   const palettes=pair.reference.uniformWrites.filter(w=>w.size===bytes&&w.label.includes('UniformBuffer'));
   expect.soft(palettes.length,kind).toBe(kind==='upscaling_fsr1'?8:1);
   expect.soft(pair.rust.attributeBytes,kind).toBe(palettes.reduce((sum,w)=>sum+w.bytes,0));
   for(const write of pair.rust.attributeWrites)expect.soft(write.label).toBe('skin/morph input');
  }else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});

test('TSL viewport FSR1 kernels share official HDR input',async({page},info)=>{
await page.setViewportSize({width:512,height:512});
await page.addInitScript(()=>{window.targets=[];const view=GPUTexture.prototype.createView;const views=new WeakMap();GPUTexture.prototype.createView=function(...a){const v=view.apply(this,a);views.set(v,this);return v;};const begin=GPUCommandEncoder.prototype.beginRenderPass;GPUCommandEncoder.prototype.beginRenderPass=function(d){const v=d.colorAttachments[0];if(v)targets.push({label:d.label,texture:views.get(v.resolveTarget||v.view)});return begin.call(this,d);};});await page.goto('/reference/three-js/tsl-viewport.html?id=webgpu_upscaling_fsr1');await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true');
const report=await page.evaluate(async wgsl=>{targets=[];await renderFixture(0);const list=targets.filter(t=>t.texture?.format==='rgba16float');const d=reference.renderer.backend.device;
const read=async texture=>{const row=Math.ceil(texture.width*8/256)*256;const buffer=d.createBuffer({size:row*texture.height,usage:GPUBufferUsage.COPY_DST|GPUBufferUsage.MAP_READ});const e=d.createCommandEncoder();e.copyTextureToBuffer({texture},{buffer,bytesPerRow:row},{width:texture.width,height:texture.height});d.queue.submit([e.finish()]);await buffer.mapAsync(GPUMapMode.READ);const a=Array.from(new Uint16Array(buffer.getMappedRange()));buffer.unmap();buffer.destroy();return a;};
const result={passes:list.map(t=>({label:t.label,size:[t.texture.width,t.texture.height]})),comparisons:[]};
const source=list.find(t=>t.texture.width===256)?.texture;const full=list.filter(t=>t.texture.width===512);for(const [index,fn,input] of [[0,'fsr_easu',source],[1,'fsr_rcas',full[0]?.texture]]){if(!input||!full[index])continue;const texture=d.createTexture({size:[512,512],format:'rgba16float',usage:GPUTextureUsage.RENDER_ATTACHMENT|GPUTextureUsage.COPY_SRC});const module=d.createShaderModule({code:wgsl+`\n@group(0) @binding(0) var input:texture_2d<f32>;@vertex fn vs(@builtin(vertex_index) i:u32)->@builtin(position) vec4<f32>{let p=array<vec2<f32>,3>(vec2(-1.0,-1.0),vec2(3.0,-1.0),vec2(-1.0,3.0));return vec4(p[i],0.0,1.0);}@fragment fn fs(@builtin(position) p:vec4<f32>)->@location(0) vec4<f32>{return ${fn}(input,p.xy/512.0${index?',0.2,false':''});}`});const pipeline=d.createRenderPipeline({layout:'auto',vertex:{module,entryPoint:'vs'},fragment:{module,entryPoint:'fs',targets:[{format:'rgba16float'}]}});const bind=d.createBindGroup({layout:pipeline.getBindGroupLayout(0),entries:[{binding:0,resource:input.createView()}]});const e=d.createCommandEncoder();const pass=e.beginRenderPass({colorAttachments:[{view:texture.createView(),loadOp:'clear',storeOp:'store'}]});pass.setPipeline(pipeline);pass.setBindGroup(0,bind);pass.draw(3);pass.end();d.queue.submit([e.finish()]);const a=await read(texture),b=await read(full[index].texture);const half=x=>{const sign=x>>15?-1:1,exp=(x>>10)&31,m=x&1023;return sign*(exp===0?m*2**-24:(1+m/1024)*2**(exp-15));};let max=0,sum=0,bad=0;for(let i=0;i<a.length;i++){const diff=Math.abs(half(a[i])-half(b[i]));max=Math.max(max,diff);sum+=diff;bad+=diff>.005;}result.comparisons.push({fn,max,mean:sum/a.length,bad:bad/a.length});}return result;},readFileSync('src/tsl/fsr1.wgsl','utf8'));
expect(report.comparisons).toHaveLength(2);for(const result of report.comparisons){expect(result.max).toBeLessThan(.001);expect(result.bad).toBe(0);}
writeFileSync(info.outputPath('fsr-kernels.json'),JSON.stringify(report,null,2));
});
