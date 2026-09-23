import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.OBJECTS_DPR||1)});
// The drawing example adds a second, DOM drawing canvas; compare the renderer canvas.
const view='canvas:not(#drawing-canvas)';
// Per example: capture times, [control index, reference value, Rust value] at its time, input script.
const cases={
 instancing_raycast:{times:[0,.4,1,2],parameters:[[0,500,500],[0,1000,1000]],at:2,antialias:true,hover:[[256,256],[300,280],[210,230],[330,330]],drag:[[20,20],[60,32]]},
 math_orientation_transform:{times:[0,.4,1,2,2.5,4.2,6.1],parameters:[[0,true,1]],at:6.3,antialias:true},
 panorama_cube:{times:[0],parameters:[],at:0,drag:[[256,256],[300,280]]},
 materials_texture_canvas:{times:[0,.4,1,2],parameters:[],at:2,antialias:true,draw:[[400,30],[450,80],[500,40],[420,100]]},
 raycaster_sprite:{times:[0],parameters:[],at:0,antialias:true,hover:[[330,190],[440,250],[150,260],[256,256],[455,285]],drag:[[256,420],[300,440]],wheel:-240,pan:[[256,256],[236,236]]},
};
const official=kind=>'webgl_'+kind;
// WebGL/WebGPU MSAA resolve bounds, documented in docs/interactive-objects.md. The same
// scenes must also pass the ordinary threshold with MSAA disabled on both sides.
const msaaLimits={instancing_raycast:[.0125,.35],math_orientation_transform:[.095,2.3]};
const frames=(page,runtime,t,n)=>page.evaluate(async({runtime,t,n})=>{for(let i=0;i<n;i++){const c=document.querySelector('canvas:not(#drawing-canvas)'),previous=c.dataset.frames;if(runtime==='reference')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===previous)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t,n});
for(const [kind,spec] of Object.entries(cases))for(const samples of spec.antialias?[1,4]:[1])test(`Interactive objects official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(300000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(511,0);
  await page.goto(runtime==='reference'?`/reference/three-js/interactive-objects.html?id=${official(kind)}&samples=${samples}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
  await page.waitForFunction(v=>{const c=document.querySelector(v);const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const shots=[];
  const capture=async(t,parameter=null)=>{
   if(parameter)await page.evaluate(({runtime,parameter})=>{const [i,reference,rust]=parameter;if(runtime==='reference')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,parameter});
   await frames(page,runtime,t,1);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   shots.push(PNG.sync.read(await page.locator(view).screenshot()));
  };
  const t=spec.at;
  for(const time of spec.times)await capture(time);
  for(const parameter of spec.parameters)await capture(spec.at,parameter);
  for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await capture(t);}
  if(spec.draw){const [first,...rest]=spec.draw;await page.mouse.move(...first);await page.mouse.down();for(const p of rest)await page.mouse.move(...p,{steps:6});await page.mouse.up();await capture(t);}
  const settle=async()=>{await frames(page,runtime,t,240);await capture(t);};
  if(spec.drag){const [[x0,y0],[x1,y1]]=spec.drag;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:5});await page.mouse.up();await settle();}
  if(spec.wheel){await page.mouse.move(256,256);await page.mouse.wheel(0,spec.wheel);await capture(t);}
  if(spec.pan){const [[x0,y0],[x1,y1]]=spec.pan;await page.mouse.move(x0,y0);await page.mouse.down({button:'right'});await page.mouse.move(x1,y1,{steps:4});await page.mouse.up({button:'right'});await capture(t);}
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await frames(page,runtime,t,1);await capture(t);images[runtime]=shots;
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 const [limit,meanLimit]=(samples===4&&msaaLimits[kind])||[.005,.6];
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(limit);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(meanLimit);}
});

for(const [kind,spec] of Object.entries(cases)){const id=official(kind);
 test(`Interactive objects GPU residency: ${kind}`,async({page},info)=>{
  test.setTimeout(120000);
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(v=>Number(document.querySelector(v)?.dataset.frames)>0,view);
  const reports=[];
  for(const resize of [false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{
    for(const t of [0,1,2,4,0])await frames(page,'rust',t,1);
    for(const [i,,v] of spec.parameters){await page.evaluate(([i,v])=>app.tsl_parameter(i,v),[i,v]);await frames(page,'rust',spec.at,1);}
    for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await frames(page,'rust',spec.at,1);}
    // Drawing rewrites the resident canvas texture in place; controls move only the camera.
    if(spec.draw){await page.mouse.move(...spec.draw[0]);await page.mouse.down();await page.mouse.move(...spec.draw[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);}
    if(spec.drag){await page.mouse.move(...spec.drag[0]);await page.mouse.down();await page.mouse.move(...spec.drag[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,3);}
   };
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator(view)).not.toHaveAttribute('data-error',/.+/);
 });
}

test('Interactive objects resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,transformBytes:0,textureBytes:0};resetWork();
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(a[0]>3||a[1]>1)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];work.draws.push({count,instances:1});return fn.apply(this,a);};}
  for(const key of ['drawArraysInstanced','drawElementsInstanced']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){work.draws.push({count:key==='drawArraysInstanced'?a[2]:a[1],instances:key==='drawArraysInstanced'?a[3]:a[4]});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX)){if(b.label==='resident draw data')work.transformBytes+=size??data.byteLength;else work.attributeBytes+=size??data.byteLength;}return write.apply(this,arguments);};
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const [kind,spec] of Object.entries(cases)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(511,0);
   await page.goto(runtime==='reference'?`/reference/three-js/interactive-objects.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(v=>{const c=document.querySelector(v);if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view);
   for(const [n,t] of [1,2,3,1,2,3].entries()){if(n===5)await page.evaluate(()=>resetWork());await frames(page,runtime,t,1);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(pair.reference.draws));
  // The orientation cone turns only through its transform record.
  expect.soft(pair.rust.attributeBytes,kind).toBe(0);expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});
