import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.COMPUTE_DPR||1)});
const view='canvas';
// Per example: capture times, [control index, reference value, Rust value] at its time, input script.
// Scripted input: [action, ...arguments, capture time or null]. Drags are [x0,y0,x1,y1,button].
const cases={
 // A new sphere and canvas texture every frame (as the page builds and disposes them);
 // each capture is one more animate() on both sides.
 webgpu_test_memory:{transient:true,canvasBytes:262144,times:[0,1,2],parameters:[[1,true,1],[3,false,0],[2,false,0],[2,true,1],[4,null,1],[0,false,0],[0,true,1],[6,null,1],[5,null,1]],restore:[[3,true,1],[1,false,0]],at:0},
 webgpu_postprocessing_godrays:{times:[0],parameters:[[0,30,30],[1,.4,.4],[2,.8,.8],[3,1,1],[4,4,4],[5,1,1],[6,false,0],[6,true,1]],restore:[[0,60,60],[1,.7,.7],[2,.5,.5],[3,2,2],[4,2,2],[5,2,2]],at:0,drag:[[256,256],[300,280]],settle:true},
 // TRAA accumulates over frames: each capture renders 60 frames, past three.js r186's first
 // resolves into its not yet resized 1 × 1 targets (see docs/shadow-volume-examples.md).
 webgpu_volume_lighting_traa:{frames:60,times:[0,1,2.5,3],parameters:[[2,6,6],[3,5,5],[4,150,150],[6,1,1],[1,false,0],[1,true,1],[0,false,0],[0,true,1]],restore:[[2,12,12],[3,3,3],[4,100,100],[6,2,2]],at:3},
 webgpu_volume_lighting_rectarea:{times:[0,1,2.5],parameters:[[0,.5,.5],[1,6,6],[2,.2,.2],[3,false,0],[3,true,1],[4,1.5,1.5],[5,.5,.5]],restore:[[0,.25,.25],[1,12,12],[2,.6,.6],[4,1,1],[5,2,2]],at:2.5,drag:[[256,256],[300,280]]},
 webgpu_volume_lighting:{times:[0,1,2.5],parameters:[[0,.5,.5],[1,6,6],[2,.2,.2],[3,false,0],[3,true,1],[4,5,5],[5,40,40],[6,1.5,1.5],[7,.5,.5]],restore:[[0,.25,.25],[1,12,12],[2,.6,.6],[4,3,3],[5,100,100],[6,1,1],[7,2,2]],at:2.5,drag:[[256,256],[300,280]]},
 webgpu_instancing_morph:{textureStream:true,times:[0,1,2.5,7],parameters:[],at:2.5,antialias:true},
 webgpu_shadowmap_csm:{times:[0],parameters:[[3,'uniform',0],[3,'logarithmic',1],[3,'practical',2],[2,300,300],[4,.5,.5],[6,-.3,-.3],[7,50,50],[8,100,100],[1,false,0],[1,true,1],[10,true,1],[12,false,0],[13,false,0],[11,false,0],[0,true,1]],restore:[[0,false,0],[11,true,1],[12,true,1],[13,true,1],[10,false,0],[8,1,1],[7,200,200],[6,-1,-1],[4,-1,-1],[2,1000,1000]],at:0,antialias:true,drag:[[256,256],[300,280]],wheel:[256,256,-200]},
 webgpu_caustics:{times:[0,1,2.5],parameters:[[0,5,5],[0,20,20],[2,'glass',1]],restore:[[2,'duck',0]],at:2.5,antialias:true,drag:[[256,256],[300,280]]},
 // The dragons' first frames after a load can differ slightly (docs/compute-examples.md):
 // each capture renders two frames.
 webgpu_shadowmap_opacity:{times:[0],parameters:[],at:0,antialias:true,frames:2,drag:[[256,256],[300,280]],wheel:[256,256,-300]},
 webgpu_lights_projector:{times:[0,1,2.5],parameters:[[2,300,300],[3,15,15],[4,.8,.8],[5,.2,.2],[6,1,1],[7,.5,.5],[8,false,0],[8,true,1],[0,'texture',2],[1,0xff8800,0xff8800]],restore:[[0,'procedural',0],[1,0xffffff,0xffffff],[2,100,100],[3,0,0],[4,Math.PI/6,Math.PI/6],[5,1,1],[6,2,2],[7,1,1]],at:2.5,antialias:true,drag:[[256,256],[300,280]]},
 webgpu_shadowmap_vsm:{times:[0,1,2.5],parameters:[[0,10,10],[1,3,3],[2,0,0],[3,20,20],[4,false,0]],restore:[[0,4,4],[1,8,8],[2,4,4],[3,8,8],[4,true,1]],at:2.5,antialias:true,drag:[[256,256],[300,280]]},
 webgpu_lights_dynamic:{times:[0,1,2.5],parameters:[[2,0,1],[2,0,1],[3,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[2,0,1],[4,0,1]],restore:[[2,0,1],[2,0,1]],at:2.5,antialias:true,drag:[[256,256],[300,280]],settle:true},
 webgpu_lights_clustered:{textureStream:true,times:[0,1,2.5],parameters:[[0,2,2],[2,.5,.5],[3,10,10],[1,false,0]],restore:[[0,1,1],[2,0,0],[3,20,20],[1,true,1]],at:2.5,drag:[[256,256],[300,280]],settle:true},
 webgpu_tsl_vfx_linkedparticles:{times:[0,.1,.2,.3,.5],parameters:[[16,2,2],[15,.2,.2],[17,.5,.5],[4,2,2]],restore:[[16,.75,.75],[15,.5,.5],[17,.1,.1],[4,1,1]],at:.5,antialias:true},
 webgpu_tsl_compute_attractors_particles:{times:[0,1,2.5],parameters:[[2,4,4],[3,.05,.05],[4,5,5],[5,.02,.02],[6,5,5],[10,false,0],[10,true,1]],restore:[[2,8,8],[3,.1,.1],[4,2.75,2.75],[5,.008,.008],[6,8,8]],at:2.5,antialias:true,drag:[[60,450],[100,470]],script:[['drag',256,256,300,280,0,2.5]]},
 webgpu_compute_birds:{times:[0,1,2.5],parameters:[[0,30,30],[1,40,40],[2,10,10]],restore:[[0,15,15],[1,20,20],[2,20,20]],at:2.5,antialias:true,drag:[[256,256],[300,280]],limits:[.04,4]},
};
const official=kind=>kind;
// The original streams the 10,000 instance matrices and colors each frame.
const streams=[];
// Perform one scripted input step; returns its capture time (or null).
const act=async(page,runtime,step)=>{const [action,...args]=step;const time=args.pop();const button=i=>['left','middle','right'][i];
 if(action==='drag'){const [x0,y0,x1,y1,b]=args;await page.mouse.move(x0,y0);await page.mouse.down({button:button(b)});await page.mouse.move(x1,y1,{steps:5});await page.mouse.up({button:button(b)});}
 else if(action==='wheel'){const [x,y,delta]=args;await page.mouse.move(x,y);await page.mouse.wheel(0,delta);}
 else if(action==='key'){const [key,down]=args;if(down)await page.keyboard.down(key);else await page.keyboard.up(key);}
 else if(action==='video'){const [seconds]=args;await page.evaluate(async seconds=>{const v=document.getElementById('video');v.pause();v.currentTime=seconds;await new Promise(r=>v.addEventListener('seeked',r,{once:true}));await new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)));},seconds);}
 else if(action==='lock'){await page.evaluate(runtime=>{if(runtime!=='rust')fixtureLockControls.isLocked=true;else app.tsl_parameter(0,1);},runtime);}
 else if(action==='look'){const [x,y]=args;await page.evaluate(([x,y])=>document.dispatchEvent(new MouseEvent('mousemove',{movementX:x,movementY:y})),[x,y]);}
 else if(action==='type'){await page.keyboard.type(args[0]);}
 else if(action==='press'){await page.keyboard.press(args[0]);}
 else if(action==='move'){const [x,y]=args;await page.mouse.move(x,y);}
 else if(action==='down'||action==='up'){const [b,x,y]=args;await page.mouse.move(x,y);await page.mouse[action]({button:button(b)});}
 else if(action==='param'){const [i,reference,rust]=args;await page.evaluate(({runtime,i,reference,rust})=>{if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,i,reference,rust});}
 return time;
};
// Scoped bounds, documented in docs/compute-examples.md: the birds' velocity pass reads
// other birds' velocities while they are written, as the original's does, so the original
// differs from itself between runs; the birds are bounded with and without MSAA (spec.limits).
const msaaLimits={};
const frames=(page,runtime,t,n)=>page.evaluate(async({runtime,t,n})=>{for(let i=0;i<n;i++){const c=document.querySelector('canvas'),previous=c.dataset.frames;if(runtime!=='rust')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===previous)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t,n});
for(const [kind,spec] of Object.entries(cases))for(const samples of spec.antialias?[1,4]:[1])test(`Compute examples official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(300000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(511,0);
  await page.goto(runtime!=='rust'?`/reference/three-js/compute-examples.html?id=${official(kind)}&samples=${samples}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
  await page.waitForFunction(v=>{const c=document.querySelector(v);const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info,#stats,#selectBox,#blocker{display:none!important}'+(spec.hide?spec.hide+'{visibility:hidden!important}':'')});
  const shots=[];
  const capture=async(t,parameter=null)=>{
   if(parameter)await page.evaluate(({runtime,parameter})=>{const [i,reference,rust]=parameter;if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,parameter});
   await frames(page,runtime,t,spec.frames??1);
   if(spec.pick)for(let k=0;k<3;k++){await page.waitForTimeout(100);await frames(page,runtime,t,1);}
   expect(await page.evaluate(()=>window.fixtureError)).toBeNull();
   shots.push(PNG.sync.read(await page.locator(view).screenshot()));
  };
  const t=spec.at;
  for(const time of spec.times)await capture(time);
  for(const [i,reference,rust,time] of spec.parameters)await capture(time??spec.at,[i,reference,rust]);
  for(const step of spec.script??[]){const time=await act(page,runtime,step);if(time!==null)await capture(time);}
  for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await capture(t);}
  if(spec.draw){const [first,...rest]=spec.draw;await page.mouse.move(...first);await page.mouse.down();for(const p of rest)await page.mouse.move(...p,{steps:6});await page.mouse.up();await capture(t);}
  for(const [x,y,time] of spec.motion??[]){await page.mouse.move(x,y);await capture(time);}
  for(const [x,y,shift] of spec.clicks??[]){if(shift)await page.keyboard.down('Shift');await page.mouse.click(x,y);if(shift)await page.keyboard.up('Shift');await capture(t);}
  const settle=async()=>{await frames(page,runtime,t,240);await capture(t);};
  if(spec.drag){const [[x0,y0],[x1,y1]]=spec.drag;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:5});await page.mouse.up();await settle();}
  if(spec.slide){const [[x0,y0],[x1,y1]]=spec.slide;await page.mouse.move(x0,y0);await page.mouse.down();await page.mouse.move(x1,y1,{steps:4});await page.mouse.up();await capture(t);}
  // Damped controls keep moving after input: those captures wait for the motion to settle.
  const after=()=>spec.settle?settle():capture(t);
  const wheel=async([x,y,delta])=>{await page.mouse.move(x,y);await page.mouse.wheel(0,delta);await after();};
  if(spec.wheel)await wheel(spec.wheel);
  if(spec.pan){const [[x0,y0],[x1,y1]]=spec.pan;await page.mouse.move(x0,y0);await page.mouse.down({button:'right'});await page.mouse.move(x1,y1,{steps:4});await page.mouse.up({button:'right'});await after();}
  if(!spec.noResize){await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await frames(page,runtime,t,1);await capture(t);}images[runtime]=shots;
 }
 const results=[];
 for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail ||=d>6;}if(fail)bad++;}results.push({state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 const [limit,meanLimit]=(samples===4&&msaaLimits[kind])||spec.limits||[.005,.6];
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(limit);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(meanLimit);}
});

for(const [kind,spec] of Object.entries(cases)){const id=official(kind);
 test(`Compute examples GPU residency: ${kind}`,async({page},info)=>{
  test.setTimeout(120000);
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(v=>Number(document.querySelector(v)?.dataset.frames)>0,view);
  const reports=[];
  for(const resize of spec.noResize?[false]:[false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{
    // Accumulating motion driven by the clock deltas uses a cycle whose deltas cancel.
    for(const t of spec.cycle??[0,1,2,4,0])await frames(page,'rust',t,1);
    // Parameter cycles end where they began, so the second cycle revisits the same states.
    for(const [i,,v] of spec.rebuilds?[]:[...spec.parameters,...(spec.restore??[])]){await page.evaluate(([i,v])=>app.tsl_parameter(i,v),[i,v]);await frames(page,'rust',spec.at,1);}
    // Typing rebuilds the text geometry, as the original does: the cycle drags only.
    for(const step of spec.residency??spec.script??[]){const time=await act(page,'rust',step);await frames(page,'rust',time??spec.at,1);}
    if(spec.wheel){await page.mouse.move(spec.wheel[0],spec.wheel[1]);await page.mouse.wheel(0,spec.wheel[2]);await frames(page,'rust',spec.at,1);await page.mouse.wheel(0,-spec.wheel[2]);await frames(page,'rust',spec.at,1);}
    for(const [x,y] of spec.hover??[]){await page.mouse.move(x,y);await frames(page,'rust',spec.at,1);}
    for(const [x,y,shift] of spec.clicks??[]){await page.mouse.click(x,y);await frames(page,'rust',spec.at,1);}
    if(spec.slide){await page.mouse.move(...spec.slide[0]);await page.mouse.down();await page.mouse.move(...spec.slide[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);await page.mouse.move(...spec.slide[1]);await page.mouse.down();await page.mouse.move(...spec.slide[0],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,1);}
    if(spec.drag){await page.mouse.move(...spec.drag[0]);await page.mouse.down();await page.mouse.move(...spec.drag[1],{steps:3});await page.mouse.up();await frames(page,'rust',spec.at,3);}
   };
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   // A streamed surface keeps changing with the advancing clock: its buffers reach their
   // grown capacity after a further cycle.
   if(spec.streamsGeometry)await cycle();
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});
   // A surface rebuilt each frame streams its attributes as the original does (bounded by
   // the workload test); it must still create nothing and keep residency flat.
   if(spec.streamsGeometry){delete before.transfers;delete after.transfers;}
   // A page that builds and disposes its mesh every frame creates on every frame; what
   // stays flat is the resident set.
   if(spec.transient){for(const r of [before,after]){delete r.creates;delete r.transfers;}}
   expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator(view)).not.toHaveAttribute('data-error',/.+/);
 });
}

test('Compute examples resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,transformBytes:0,textureBytes:0};resetWork();
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(a[0]>3||a[1]>1)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];// WebGL points become six-vertex WebGPU billboards.
// Draws of three or fewer vertices (the port's fullscreen clear and present triangles, and
// the two-vertex helper line) are left out on both sides.
if(count>3)work.draws.push({count:a[0]===0?count*6:count,instances:1});return fn.apply(this,a);};}
  for(const key of ['drawArraysInstanced','drawElementsInstanced']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){work.draws.push({count:key==='drawArraysInstanced'?a[2]:a[1],instances:key==='drawArraysInstanced'?a[3]:a[4]});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){// three's WebGPU InstanceNode keeps up to 64 KiB of instance matrices in a uniform
  // buffer: the reference's large uniform uploads count as streamed instance data.
  if(location.pathname.startsWith('/reference/')&&b.usage&GPUBufferUsage.UNIFORM&&(size??data.byteLength)>=16384)work.attributeBytes+=size??data.byteLength;
  if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX)){if(b.label==='resident draw data'||b.label==='skin/morph input')work.transformBytes+=size??data.byteLength;else work.attributeBytes+=size??data.byteLength;}return write.apply(this,arguments);};
  // WebGL attribute uploads: the original streams its dynamic attributes with bufferSubData.
  for(const key of ['bufferData','bufferSubData']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const data=key==='bufferData'?a[1]:a[2];if(typeof data==='object'&&data)work.attributeBytes+=key==='bufferSubData'&&a[4]?a[4]*data.BYTES_PER_ELEMENT:data.byteLength;return fn.apply(this,a);};}
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const [kind,spec] of Object.entries(cases)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(511,0);
   await page.goto(runtime==='reference'?`/reference/three-js/compute-examples.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(v=>{const c=document.querySelector(v);if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view);
   // Per-frame solvers (the IK chain) converge before the measured frame.
   if(spec.frames)await frames(page,runtime,1,spec.frames);
   for(const [n,t] of [1,2,3,1,2,3].entries()){if(n===5)await page.evaluate(()=>resetWork());await frames(page,runtime,t,1);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  // WebGL draws an equirectangular background as a 36-index box; the port's
  // background is a fullscreen triangle, left out with the other tiny draws.
  if(spec.backgroundBox){const i=pair.reference.draws.findIndex(d=>d.count===36&&d.instances===1);if(i>=0)pair.reference.draws.splice(i,1);}
  // The advanced composers' MaskPasses draw the head into both ping-pong buffers, three
  // times over two resolutions (six draws); the port draws one mask per resolution (two).
  if(spec.maskDraws){const [count,reference,rust]=spec.maskDraws;for(let k=0;k<reference-rust;k++){const i=pair.reference.draws.findIndex(d=>d.count===count);if(i>=0)pair.reference.draws.splice(i,1);}}
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  // The port draws equirectangular backgrounds with a fullscreen triangle; WebGL uses a
  // 36-index box and WebGPU a 5,952-index sphere. The stereo port draws its cube
  // background as a 36-index box (once per eye). All scene mesh draws must still match.
  const background=spec.backgroundSphere?5952:null;
  const referenceDraws=pair.reference.draws.filter(d=>d.count!==background);
  expect.soft(sort(pair.rust.draws.filter(d=>!spec.backgroundSphere||d.count!==36)),kind).toEqual(sort(referenceDraws));
  // Each instance's matrix and color stream together (80 bytes) as resident draw data;
  // WebGL streams the 64-byte matrices, and the colors only during a tween.
  if(spec.batch)expect.soft(pair.rust.attributeBytes,kind).toBeLessThanOrEqual(pair.reference.textureBytes*1.25);
  else if(streams.includes(kind))expect.soft(pair.rust.attributeBytes+pair.rust.transformBytes,kind).toBeLessThanOrEqual(pair.reference.attributeBytes*1.25);
  else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
  // ClusteredLighting's sorted light texture and the horses' morph influences are rewritten
  // each frame by the originals, as by the ports.
  // test_memory fills one new 256 × 256 canvas texture per frame, uploaded as the
  // original's CanvasTexture is.
  if(spec.canvasBytes)expect.soft(pair.rust.textureBytes,kind).toBe(spec.canvasBytes);
  else if(spec.textureStream)expect.soft(pair.rust.textureBytes,kind).toBeLessThanOrEqual(pair.reference.textureBytes);
  else expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});

// webgpu_compute_sort_bitonic draws two canvases side by side and steps both sorts on
// the page's 100 ms timers (1 s after a finished sort): the page is compared at times
// through the left sort's end and restart and the right sort's end. The steps are
// integer compute passes, so the grids match exactly; nothing is created while they run.
test('Bitonic sort: both sorts match the original step by step',async({page},info)=>{
 test.setTimeout(300000);await page.setViewportSize({width:512,height:512});
 await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...a){creates++;return original.apply(this,a);};}});
 const times=[0,.35,1.2,3.55,4.6,5.2,10.55,11.7,12.2];const shots={};
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/compute-examples.html?id=webgpu_compute_sort_bitonic':'/web/gallery/example.html?id=webgpu_compute_sort_bitonic&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info,#stats{display:none!important}'});
  const list=[];const counts=[];
  for(const t of times){await frames(page,runtime,t,1);list.push(PNG.sync.read(await page.screenshot()));if(runtime==='rust')counts.push(await page.evaluate(()=>creates));}
  // Display Mode: Elements.
  await page.evaluate(runtime=>runtime==='rust'?app.tsl_parameter(0,0):fixtureParameter(0,'Elements'),runtime);await frames(page,runtime,times.at(-1),1);list.push(PNG.sync.read(await page.screenshot()));
  shots[runtime]=list;
  if(runtime==='rust')expect(counts.at(-1)).toBe(counts[1]);
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

// webgpu_tsl_vfx_linkedparticles: the pointer circles the view while particles spawn
// toward it, so the trails, ribbons, bloom and turning light are compared as they grow.
test('Linked particles: trails and links follow the pointer as in the original',async({page},info)=>{
 test.setTimeout(300000);await page.setViewportSize({width:512,height:512});
 const at=[20,40,60,80];const shots={};
 for(const runtime of ['reference','rust']){
  await page.mouse.move(256,256);
  await page.goto(runtime==='reference'?'/reference/three-js/compute-examples.html?id=webgpu_tsl_vfx_linkedparticles':'/web/gallery/example.html?id=webgpu_tsl_vfx_linkedparticles&still=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info,#stats{display:none!important}'});
  const list=[];
  for(let k=1;k<=80;k++){const a=k*0.15;await page.mouse.move(256+150*Math.cos(a),256+120*Math.sin(a));await frames(page,runtime,k/30,1);if(at.includes(k))list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});
