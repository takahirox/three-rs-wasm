import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.STEREO_DPR||1)});
const view='canvas';
// Per example: capture times, [control index, reference value, Rust value] at its time, input script.
const cases={
 effects_stereo:{times:[0,.4,1,2],parameters:[],at:2,motion:[[400,300,2.2],[400,300,2.6],[100,150,3]]},
 effects_anaglyph:{times:[0,.4,1,2],parameters:[],at:2,motion:[[400,300,2.2],[400,300,2.6],[100,150,3]]},
 effects_parallaxbarrier:{times:[0,.4,1,2],parameters:[],at:2,motion:[[400,300,2.2],[400,300,2.6],[100,150,3]]},
 loader_pcd:{times:[0],parameters:[[0,.008,.008],[1,16746496,16746496],[2,'ascii/simple.pcd',0],[2,'binary_compressed/pcl_logo.pcd',3],[2,'binary/Zaghetto_8bit.pcd',2]],at:0,antialias:true,drag:[[256,256],[300,280]],wheel:-240},
 loader_imagebitmap:{times:[0,.35,.65,.95,1.35,1.65,1.95,2.5],parameters:[],at:2.5,antialias:true},
};
const official=kind=>'webgl_'+kind;
// WebGL/WebGPU MSAA resolve bounds, documented in docs/stereo-loaders.md. The same
// scenes must also pass the ordinary threshold with MSAA disabled on both sides.
const msaaLimits={loader_pcd:[.12,7.5],loader_imagebitmap:[.035,.7]};
// Bounds with MSAA disabled, for the cases whose original path departs from exact rasterization
// or amplifies 8-bit linear eye targets (docs/stereo-loaders.md).
const pointLimits={effects_anaglyph:[.0125,.25],effects_parallaxbarrier:[.0065,.2],loader_pcd:[.0085,2.2]};
// Analytic coverage of the initial Zaghetto cloud: centered, rotated by PI about x, seen from
// (0, 0, 1) with a 30 degree vertical field of view. Values are the resolved 8-bit white level.
const pcdCoverage=(width,height,samples)=>{
 const data=readFileSync('web/gallery/assets/pcd/binary/Zaghetto.pcd');const start=data.indexOf('DATA binary\n')+12;const n=59750;
 const p=new Float32Array(n*3);for(let i=0;i<n*3;i++)p[i]=data.readFloatLE(start+i*4);
 const low=[0,1,2].map(k=>Math.min(...p.filter((_,i)=>i%3===k))),high=[0,1,2].map(k=>Math.max(...p.filter((_,i)=>i%3===k)));
 const offsets=samples===4?[[.375,.125],[.875,.375],[.125,.625],[.625,.875]]:[[.5,.5]];
 const masks=new Uint8Array(width*height),t=Math.tan(Math.PI/12),aspect=width/height;
 for(let i=0;i<n;i++){
  const x=Math.fround(p[i*3]-(low[0]+high[0])/2),y=-Math.fround(p[i*3+1]-(low[1]+high[1])/2),z=-Math.fround(p[i*3+2]-(low[2]+high[2])/2);
  const w=1-z,cx=(x/(t*aspect*w)*.5+.5)*width,cy=(.5-y/(t*w)*.5)*height,size=.005*height*.5/w;
  for(let py=Math.floor(cy-size);py<=cy+size;py++)for(let px=Math.floor(cx-size);px<=cx+size;px++){
   if(px<0||py<0||px>=width||py>=height)continue;
   offsets.forEach(([ox,oy],j)=>{const sx=px+ox,sy=py+oy;if(sx>=cx-size/2&&sx<cx+size/2&&sy>=cy-size/2&&sy<cy+size/2)masks[py*width+px]|=1<<j;});
  }
 }
 return Array.from(masks,m=>{let c=0;for(;m;m>>=1)c+=m&1;return Math.round(c/offsets.length*255);});
};
const frames=(page,runtime,t,n)=>page.evaluate(async({runtime,t,n})=>{for(let i=0;i<n;i++){const c=document.querySelector('canvas'),previous=c.dataset.frames;if(runtime!=='rust')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===previous)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t,n});
for(const [kind,spec] of Object.entries(cases))for(const samples of spec.antialias?[1,4]:[1])test(`Stereo and loaders official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(300000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(511,0);
  await page.goto(runtime!=='rust'?`/reference/three-js/stereo-loaders.html?id=${official(kind)}&samples=${samples}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
  await page.waitForFunction(v=>{const c=document.querySelector(v);const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const shots=[];
  const capture=async(t,parameter=null)=>{
   if(parameter)await page.evaluate(({runtime,parameter})=>{const [i,reference,rust]=parameter;if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,parameter});
   await frames(page,runtime,t,1);
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   shots.push(PNG.sync.read(await page.locator(view).screenshot()));
  };
  const t=spec.at;
  for(const time of spec.times)await capture(time);
  for(const parameter of spec.parameters)await capture(spec.at,parameter);
  for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await capture(t);}
  if(spec.draw){const [first,...rest]=spec.draw;await page.mouse.move(...first);await page.mouse.down();for(const p of rest)await page.mouse.move(...p,{steps:6});await page.mouse.up();await capture(t);}
  for(const [x,y,time] of spec.motion??[]){await page.mouse.move(x,y);await capture(time);}
  for(const [x,y,shift] of spec.clicks??[]){if(shift)await page.keyboard.down('Shift');await page.mouse.click(x,y);if(shift)await page.keyboard.up('Shift');await capture(t);}
  const settle=async()=>{await frames(page,runtime,t,240);await capture(t);};
  if(spec.drag){const [[x0,y0],[x1,y1]]=spec.drag;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:5});await page.mouse.up();await settle();}
  if(spec.slide){const [[x0,y0],[x1,y1]]=spec.slide;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:4});await page.mouse.up();await capture(t);}
  if(spec.wheel){await page.mouse.move(256,256);await page.mouse.wheel(0,spec.wheel);await capture(t);}
  if(spec.pan){const [[x0,y0],[x1,y1]]=spec.pan;await page.mouse.move(x0,y0);await page.mouse.down({button:'right'});await page.mouse.move(x1,y1,{steps:4});await page.mouse.up({button:'right'});await capture(t);}
  await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await frames(page,runtime,t,1);await capture(t);images[runtime]=shots;
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 if(kind==='loader_pcd'){
  // The first state is Zaghetto at size 0.005 (about 1.3 px points). Against exact square
  // coverage at the pixel centers (or the four standard 4x sample positions), the port must
  // agree almost everywhere; the original's WebGL points depart from it (docs/stereo-loaders.md).
  const coverage=pcdCoverage(images.rust[0].width,images.rust[0].height,samples);
  const miss=image=>{let n=0;for(let i=0;i<coverage.length;i++)if(Math.abs(image.data[i*4]-coverage[i])>40)n++;return n/coverage.length;};
  const analytic={rust:miss(images.rust[0]),reference:miss(images.reference[0])};
  writeFileSync(info.outputPath('coverage.json'),JSON.stringify(analytic,null,2));
  expect(analytic.rust,JSON.stringify(analytic)).toBeLessThanOrEqual(samples===4?.004:.0015);
  expect(analytic.reference,JSON.stringify(analytic)).toBeGreaterThan(4*analytic.rust);
 }
 const [limit,meanLimit]=(samples===4?msaaLimits[kind]:pointLimits[kind])||[.005,.6];
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(limit);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(meanLimit);}
});

for(const [kind,spec] of Object.entries(cases)){const id=official(kind);
 test(`Stereo and loaders GPU residency: ${kind}`,async({page},info)=>{
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
    for(const [x,y,shift] of spec.clicks??[]){await page.mouse.click(x,y);await frames(page,'rust',spec.at,1);}
    if(spec.slide){await page.mouse.move(...spec.slide[0]);await page.mouse.down();await page.mouse.move(...spec.slide[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);await page.mouse.move(...spec.slide[1]);await page.mouse.down();await page.mouse.move(...spec.slide[0],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);}
    if(spec.drag){await page.mouse.move(...spec.drag[0]);await page.mouse.down();await page.mouse.move(...spec.drag[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,3);}
   };
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator(view)).not.toHaveAttribute('data-error',/.+/);
 });
}

test('Stereo and loaders resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,transformBytes:0,textureBytes:0};resetWork();
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(a[0]>3||a[1]>1)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];// WebGL points become six-vertex WebGPU billboards.
// Fullscreen triangles (the stereo composites and the port's present pass) are left out on both sides.
if(a[0]!==4||count>3)work.draws.push({count:a[0]===0?count*6:count,instances:1});return fn.apply(this,a);};}
  for(const key of ['drawArraysInstanced','drawElementsInstanced']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){work.draws.push({count:key==='drawArraysInstanced'?a[2]:a[1],instances:key==='drawArraysInstanced'?a[3]:a[4]});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX)){if(b.label==='resident draw data')work.transformBytes+=size??data.byteLength;else work.attributeBytes+=size??data.byteLength;}return write.apply(this,arguments);};
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const [kind,spec] of Object.entries(cases)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(511,0);
   await page.goto(runtime==='reference'?`/reference/three-js/stereo-loaders.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(v=>{const c=document.querySelector(v);if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view);
   for(const [n,t] of [1,2,3,1,2,3].entries()){if(n===5)await page.evaluate(()=>resetWork());await frames(page,runtime,t,1);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(pair.reference.draws));
  expect.soft(pair.rust.attributeBytes,kind).toBe(0);expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});


test('Stereo and loaders on-demand scene stays idle: loader_pcd',async({page})=>{
 await page.goto('/web/gallery/example.html?id=webgl_loader_pcd');await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
 await page.waitForTimeout(300);const frames=await page.locator('canvas').getAttribute('data-frames');await page.waitForTimeout(300);expect(await page.locator('canvas').getAttribute('data-frames')).toBe(frames);
 await page.setViewportSize({width:640,height:400});await page.waitForFunction(prev=>document.querySelector('canvas').dataset.frames!==prev,frames);
});
