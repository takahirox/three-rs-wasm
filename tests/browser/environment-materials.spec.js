import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.ENVIRONMENT_DPR||1)});
const controls={materials_basic:[[0,0x88bbff],[1,1],[2,.6],[3,true],[4,.5],[3,false],[1,0]],materials_envmaps:[[0,1],[1,true],[2,true],[3,true],[4,true],[5,true],[0,0],[1,false]],materials_displacementmap:[[0,0],[1,.8],[2,0],[3,.8],[4,0],[5,0],[5,3],[6,-1],[0,1],[4,1]],materials_bumpmap:[[0,false],[0,true],[1,30],[1,0],[1,10]],materials_blending:[]};
const official=kind=>(['materials_basic','materials_envmaps','materials_displacementmap'].includes(kind)?'webgpu_':'webgl_')+kind;
// The WebGPU bump diagnostic retains the source scene and material inputs; only the backend changes.
for(const [kind,parameters] of Object.entries(controls))for(const samples of (kind==='materials_bumpmap'?[1,4]:[1]))for(const diagnostic of (kind==='materials_bumpmap'?['','gpuBump']:['']))test(`Environment materials official rendering: ${kind} samples=${samples} ${diagnostic}`,async({page},info)=>{
 test.setTimeout(240000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;window.modules=[];const fn=GPUDevice.prototype.createShaderModule;GPUDevice.prototype.createShaderModule=function(d){modules.push(d.code);return fn.call(this,d);};const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.goto(runtime==='reference'?`/reference/three-js/environment-materials.html?id=${official(kind)}&samples=${samples}&${diagnostic}=1`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info,.texture-label,#lbl_left,#lbl_right{display:none!important}'});
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
  if(['materials_envmaps','materials_displacementmap','materials_bumpmap'].includes(kind)){await page.mouse.move(200,200);await page.mouse.down();await page.mouse.move(250,225,{steps:5});await page.mouse.up();if(['materials_displacementmap','materials_bumpmap'].includes(kind))for(let i=0;i<180;i++)await capture(2.25,null,false);await capture(2.25);await page.mouse.wheel(0,-100);await capture(2.25);}
  if(kind==='materials_basic'){await page.mouse.move(296,276);for(let i=0;i<180;i++)await capture(2.25,null,false);await capture(2.25);}
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await capture(2.5,null,false);await capture(2.5);images[runtime]=frames;
  writeFileSync(info.outputPath(`${runtime}-shaders.json`),JSON.stringify(await page.evaluate(()=>modules)));
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);for(const r of results){const nativeBump=kind==='materials_bumpmap'&&!diagnostic;const limit=nativeBump?.08:.005,meanLimit=nativeBump?1.5:.6;expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(limit);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(meanLimit);}
});

for(const kind of Object.keys(controls)){const id=`${official(kind)}`;
 test(`Environment materials GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{for(const t of [0,1,2,4,0]){let prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}for(const parameter of controls[kind]){const prev=await page.locator('canvas').first().getAttribute('data-frames');await page.evaluate(([i,v])=>app.tsl_parameter(i,Number(v)),parameter);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,prev);}};
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator('canvas').first()).not.toHaveAttribute('data-error',/.+/);
 });
}



test('Environment materials resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(120000);
 await page.addInitScript(()=>{
  window.attributeBuffers=[];const createBuffer=GPUDevice.prototype.createBuffer;GPUDevice.prototype.createBuffer=function(d){if(d.label==='application GPU buffer')attributeBuffers.push(d.size);return createBuffer.call(this,d);};
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,transformBytes:0,textureBytes:0};resetWork();
  // Tag upstream scene meshes separately from its background sphere and presentation quad.
  const setPipeline=GPURenderPassEncoder.prototype.setPipeline;GPURenderPassEncoder.prototype.setPipeline=function(p){this.scenePipeline=['material pipeline','shadow depth','TSL shadow'].includes(p.label)||p.label?.startsWith('renderPipeline_fixture-scene_');return setPipeline.call(this,p);};
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(this.scenePipeline)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];work.draws.push({count:a[0]===0?count*6:count,instances:1});return fn.apply(this,a);};}
  for(const key of ['drawArraysInstanced','drawElementsInstanced']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){work.draws.push({count:key==='drawArraysInstanced'?a[2]:a[1],instances:key==='drawArraysInstanced'?a[3]:a[4]});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX)){if(b.label==='resident draw data')work.transformBytes+=size??data.byteLength;else work.attributeBytes+=size??data.byteLength;}return write.apply(this,arguments);};
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const kind of Object.keys(controls)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.goto(runtime==='reference'?`/reference/three-js/environment-materials.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;});
   for(const [i,t] of [3,1,2,3].entries()){const prev=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(async({runtime,t,i})=>{if(i===3)resetWork();if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);},{runtime,t,i});if(runtime==='rust')await page.waitForFunction(p=>document.querySelector('canvas').dataset.frames!==p,prev);}
   pair[runtime]=await page.evaluate(()=>({...work,attributeBuffers}));
  }
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  expect.soft(sort(pair.rust.draws).filter(n=>!['materials_basic','materials_envmaps'].includes(kind)||n!==36),kind).toEqual(sort(pair.reference.draws));
  expect.soft(pair.rust.attributeBytes,kind).toBe(0);expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});
