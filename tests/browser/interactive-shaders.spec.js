import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.INTERACTIVE_DPR||1)});
// [control index, reference value, Rust value]
const controls={shader:[],postprocessing_procedural:[[0,'noiseRandom1D',0],[0,'noiseRandom2D',1],[0,'noiseRandom3D',2]],interactive_cubes:[],interactive_cubes_ortho:[],interactive_points:[]};
const antialias=kind=>kind.startsWith('interactive_cubes');
const pointers={interactive_cubes:[[300,240],[120,380]],interactive_cubes_ortho:[[300,240],[120,380]],interactive_points:[[256,256],[330,190],[40,40]]};
const official=kind=>'webgl_'+kind;
// WebGL/WebGPU MSAA resolve bounds, documented in docs/interactive-shaders.md. The same
// scenes must also pass the ordinary threshold with MSAA disabled on both sides.
const msaaLimits={interactive_cubes:[.035,1.1],interactive_cubes_ortho:[.095,2.6]};
const histogram=image=>{const h=Array.from({length:3},()=>new Float64Array(256));for(let p=0;p<image.data.length;p+=4)for(let c=0;c<3;c++)h[c][image.data[p+c]]++;return h;};
const histogramDistance=(a,b)=>{const x=histogram(a),y=histogram(b),n=a.width*a.height;return Math.max(...x.map((h,c)=>h.reduce((s,v,i)=>s+Math.abs(v-y[c][i]),0)/n));};
const channelMeans=image=>[0,1,2].map(c=>{let s=0;for(let p=c;p<image.data.length;p+=4)s+=image.data[p];return s/(image.data.length/4);});
for(const [kind,parameters] of Object.entries(controls))for(const samples of (antialias(kind)?[1,4]:[1]))test(`Interactive shaders official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(240000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 // The procedural noise also renders the original with vUv.y perturbed by one ULP.
 for(const runtime of kind==='postprocessing_procedural'?['reference','rust','perturbed']:['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(256,256);
  await page.goto(runtime==='rust'?`/web/gallery/example.html?id=${official(kind)}&still=1`:`/reference/three-js/interactive-shaders.html?id=${official(kind)}&samples=${samples}${runtime==='perturbed'?'&uvUlp=1':''}`);
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const frames=[];
  const capture=async(t,parameter=null,save=true)=>{
   const prev=await page.locator('canvas').getAttribute('data-frames');
   await page.evaluate(async({runtime,t,parameter})=>{if(parameter){const [i,reference,rust]=parameter;if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);}if(runtime!=='rust')await renderFixture(t);else app.gallery_time(t);},{runtime,t,parameter});
   if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   if(save)frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  };
  for(const t of [0,.4,1,2])await capture(t);
  for(const parameter of parameters)await capture(2.25,parameter);
  for(const [x,y] of pointers[kind]??[]){await page.mouse.move(x,y);await capture(2.25);}
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await capture(2.5,null,false);await capture(2.5);images[runtime]=frames;
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 if(images.perturbed)for(const r of results){const a=images.rust[r.state],b=images.reference[r.state],c=images.perturbed[r.state];let bad=0;for(let p=0;p<b.data.length;p+=4)if([0,1,2].some(k=>Math.abs(b.data[p+k]-c.data[p+k])>6))bad++;Object.assign(r,{width:a.width,height:a.height,ulpFraction:bad/(b.width*b.height),histogram:histogramDistance(a,b),ulpHistogram:histogramDistance(c,b),meanShift:Math.max(...channelMeans(a).map((m,k)=>Math.abs(m-channelMeans(b)[k])))});}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 const [limit,meanLimit]=(samples===4&&msaaLimits[kind])||[.005,.6];
 for(const r of results){
  // Hash noise at non-power-of-two heights: a one-ULP change of the original's vUv already
  // alters most pixels, so compare its distribution. Power-of-two sizes must match per pixel.
  if(images.perturbed&&r.height%256){expect(r.ulpFraction,JSON.stringify(r)).toBeGreaterThan(.2);expect(r.histogram,JSON.stringify(r)).toBeLessThanOrEqual(Math.max(1.5*r.ulpHistogram,.01));expect(r.meanShift,JSON.stringify(r)).toBeLessThanOrEqual(.5);continue;}
  expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(limit);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(meanLimit);
 }
});

for(const kind of Object.keys(controls)){const id=official(kind);
 test(`Interactive shaders GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const frame=async action=>{const prev=await page.locator('canvas').first().getAttribute('data-frames');await action();await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);};
   const cycle=async()=>{
    for(const t of [0,1,2,4,0])await frame(()=>page.evaluate(t=>app.gallery_time(t),t));
    for(const [i,,v] of controls[kind])await frame(()=>page.evaluate(([i,v])=>app.tsl_parameter(i,v),[i,v]));
    // Selection changes only material uniforms or the resident size buffer.
    for(const [x,y] of [...(pointers[kind]??[]),[256,256]]){await page.mouse.move(x,y);await frame(()=>page.evaluate(()=>app.gallery_time(2)));}
   };
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas').first()).not.toHaveAttribute('data-error',/.+/);
 });
}

test('Interactive shaders resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.attributeBuffers=[];const createBuffer=GPUDevice.prototype.createBuffer;GPUDevice.prototype.createBuffer=function(d){if(d.label==='application GPU buffer')attributeBuffers.push(d.size);return createBuffer.call(this,d);};
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,textureBytes:0};resetWork();
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(a[0]>3||a[1]>1)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];work.draws.push({count:a[0]===0?count*6:count,instances:1});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX))work.attributeBytes+=size??data.byteLength;return write.apply(this,arguments);};
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(256,256);
   await page.goto(runtime==='reference'?`/reference/three-js/interactive-shaders.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   // Warm pass first: per-object draw records are created when an object first becomes visible.
   for(const [n,t] of [1,2,3,1,2,3].entries()){const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(async({runtime,t,last})=>{if(last)resetWork();if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t,last:n===5});if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);}
   pair[runtime]=await page.evaluate(()=>({...work,attributeBuffers}));
  }
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(pair.reference.draws));
  expect.soft(pair.rust.attributeBytes,kind).toBe(0);expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});
