import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.VOLUMES_DPR||1)});
const view='canvas';
// Per example: capture times, [control index, reference value, Rust value] at its time, input script.
// Scripted input: [action, ...arguments, capture time or null]. Drags are [x0,y0,x1,y1,button].
const cases={
 webgl_postprocessing_ssao:{times:[0,1,2.5],parameters:[[0,'SSAO Only',1],[0,'SSAO Only + Blur',2],[0,'Depth',3],[0,'Normal',4],[0,'Default',0],[1,16,16],[2,.01,.01],[3,.2,.2],[4,false,0]],restore:[[1,8,8],[2,.005,.005],[3,.1,.1],[4,true,1]],at:2.5},
 webgpu_clipping_stencil:{times:[0,1,2.5],parameters:[[2,.3,.3],[5,-.2,-.2],[3,true,1],[8,.4,.4],[1,true,1],[4,true,1],[7,true,1],[0,false,0],[9,true,1]],restore:[[2,0,0],[5,0,0],[3,false,0],[8,0,0],[1,false,0],[4,false,0],[7,false,0],[0,true,1],[9,false,0]],at:2.5,antialias:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 // The slider's line and handle and the labels are page overlays, hidden in both.
 webgl_test_wide_gamut:{times:[0],parameters:[],at:0,antialias:true,hide:'.slider:before,.slider:after,.label',slide:[[256,256],[150,256]]},
 webgl_materials_channels:{times:[0],parameters:[[0,'standard',0],[0,'velocity',2],[0,'depthBasic',3],[0,'depthRGBA',4],[0,'depthRGB',5],[0,'depthRG',6],[0,'normal',1],[2,'front',0],[2,'back',1],[1,'ortho',1],[0,'depthRGBA',4],[0,'standard',0],[2,'double',2],[0,'normal',1],[1,'perspective',0]],restore:[],at:0,settle:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_video_kinect:{limits:[.008,.6],frames:3,times:[],parameters:[],at:1,script:[['video',2,0],['param',0,1500,1500,0],['param',1,6000,6000,0],['param',2,4,4,0],['param',3,2000,2000,0],['param',0,850,850,0],['param',1,4000,4000,0],['param',2,2,2,0],['param',3,1000,1000,0],['video',5.5,0],['move',400,300,null],['wait',.5],['wait',1.5],['move',60,100,null],['wait',3]]},
 webgl_loader_texture_dds:{times:[0,.8,1.7,3.1],parameters:[],at:3.1,antialias:true},
 webgpu_display_stereo:{backgroundSphere:true,times:[0,1,2.5],parameters:[[1,.1,.1],[0,'Anaglyph',1],[2,'Grey',1],[3,'Magenta / Cyan',1],[4,5,5],[2,'Compromise',6],[3,'Magenta / Green',2],[0,'ParallaxBarrier',2]],restore:[[0,'Stereo',0],[1,.064,.064],[2,'Dubois',4],[3,'Red / Cyan',0],[4,3,3]],at:2.5,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_shadowmap_viewer:{times:[0,.5,1.3,2],parameters:[],at:2,antialias:true,drag:[[256,300],[330,340]],wheel:[256,300,-300]},
 webgl_materials_envmaps_exr:{backgroundBox:true,times:[0,1,2],parameters:[[1,.5,.5],[2,1,1],[3,.6,.6],[0,'PNG',1]],restore:[[0,'EXR',0],[1,0,0],[2,0,0],[3,1,1]],at:2,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_texture2darray:{times:[0,1,2.5,4,10],parameters:[],at:4},
 webgl_texture2darray_compressed:{times:[0,.05,.13,.27,.41],parameters:[],at:.41},
 webgl_rendertarget_texture2darray:{times:[0,1,2.5,4,10],parameters:[[0,.3,.3],[0,.75,.75]],restore:[[0,1,1]],at:4},
 webgl_texture2darray_layerupdate:{times:[0],parameters:[[0,3,3],[1,1,1],[2,null,1],[0,4,4],[1,2,2],[2,null,1],[0,1,1],[1,0,0],[2,null,1]],restore:[[0,0,0],[1,0,0],[2,null,1],[0,1,1],[1,1,1],[2,null,1],[0,2,2],[1,2,2],[2,null,1],[0,0,0],[1,0,0]],at:0,antialias:true},
 webgl_loader_nrrd:{times:[0,1],parameters:[[0,60,60],[1,200,200],[2,100,100],[3,500,500],[4,3000,3000],[5,200,200],[6,2500,2500],[0,240,240],[2,0,0]],restore:[[0,120,120],[1,120,120],[2,42,42],[3,0,0],[4,3952,3952],[5,0,0],[6,3952,3952]],at:1,antialias:true,script:[['drag',256,256,330,300,0,2],['drag',256,256,300,200,2,2.5],['wheel',256,256,-400,3],['wait',4]]},
 webgl_texture3d:{limits:[.02,.5],times:[0],parameters:[[3,'mip',0],[2,'gray',0],[0,0.2,0.2],[1,0.8,0.8],[3,'iso',1],[4,0.3,0.3],[2,'viridis',1]],restore:[[0,0,0],[1,1,1],[4,0.15,0.15]],at:0,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
};
const official=kind=>kind;
// The original streams the 10,000 instance matrices and colors each frame.
// webgpu_display_stereo streams its 500 instance matrices each frame.
const streams=['webgpu_display_stereo'];
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
// WebGL/WebGPU MSAA resolve bounds, documented in docs/texture-volumes.md. The same
// scenes must also pass the ordinary threshold with MSAA disabled on both sides.
const msaaLimits={webgl_loader_nrrd:[.012,.45],webgl_shadowmap_viewer:[.025,.8]};
const frames=(page,runtime,t,n)=>page.evaluate(async({runtime,t,n})=>{for(let i=0;i<n;i++){const c=document.querySelector('canvas'),previous=c.dataset.frames;if(runtime!=='rust')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===previous)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t,n});
for(const [kind,spec] of Object.entries(cases))for(const samples of spec.antialias?[1,4]:[1])test(`Texture arrays and volumes official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(300000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(511,0);
  await page.goto(runtime!=='rust'?`/reference/three-js/texture-volumes.html?id=${official(kind)}&samples=${samples}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
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
 test(`Texture arrays and volumes GPU residency: ${kind}`,async({page},info)=>{
  test.setTimeout(120000);
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(v=>Number(document.querySelector(v)?.dataset.frames)>0,view);
  const reports=[];
  for(const resize of spec.noResize?[false]:[false,true]){
   if(resize)await page.setViewportSize({width:640,height:400});
   const cycle=async()=>{
    for(const t of [0,1,2,4,0])await frames(page,'rust',t,1);
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
   await cycle();const before=await read();await cycle();const after=await read();reports.push({resize,before,after});expect(after).toEqual(before);
  }
  writeFileSync(info.outputPath('residency.json'),JSON.stringify(reports,null,2));await expect(page.locator(view)).not.toHaveAttribute('data-error',/.+/);
 });
}

test('Texture arrays and volumes resident geometry and official draw workload',async({page},info)=>{
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
   await page.goto(runtime==='reference'?`/reference/three-js/texture-volumes.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(v=>{const c=document.querySelector(v);if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view);
   // Per-frame solvers (the IK chain) converge before the measured frame.
   if(spec.frames)await frames(page,runtime,1,spec.frames);
   for(const [n,t] of [1,2,3,1,2,3].entries()){if(n===5)await page.evaluate(()=>resetWork());await frames(page,runtime,t,1);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  // WebGL draws an equirectangular background as a 36-index box; the port's
  // background is a fullscreen triangle, left out with the other tiny draws.
  if(spec.backgroundBox){const i=pair.reference.draws.findIndex(d=>d.count===36&&d.instances===1);if(i>=0)pair.reference.draws.splice(i,1);}
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  // The port draws equirectangular backgrounds with a fullscreen triangle; WebGL uses a
  // 36-index box and WebGPU a 5,952-index sphere. The stereo port draws its cube
  // background as a 36-index box (once per eye). All scene mesh draws must still match.
  const background=spec.backgroundSphere?5952:null;
  const referenceDraws=pair.reference.draws.filter(d=>d.count!==background);
  expect.soft(sort(pair.rust.draws.filter(d=>!spec.backgroundSphere||d.count!==36)),kind).toEqual(sort(referenceDraws));
  // Each instance's matrix and color stream together (80 bytes) as resident draw data;
  // WebGL streams the 64-byte matrices, and the colors only during a tween.
  if(streams.includes(kind))expect.soft(pair.rust.attributeBytes+pair.rust.transformBytes,kind).toBeLessThanOrEqual(pair.reference.attributeBytes*1.25);
  else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
  expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});

for(const kind of ['webgl_texture2darray_layerupdate','webgl_texture3d'])test(`Texture arrays and volumes on-demand scene stays idle: ${kind}`,async({page})=>{
 await page.goto(`/web/gallery/example.html?id=${kind}`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
 await page.waitForTimeout(300);const frames=await page.locator('canvas').getAttribute('data-frames');await page.waitForTimeout(300);expect(await page.locator('canvas').getAttribute('data-frames')).toBe(frames);
 await page.setViewportSize({width:640,height:400});await page.waitForFunction(prev=>document.querySelector('canvas').dataset.frames!==prev,frames);
});

// misc_uv_tests draws UVsDebug canvases with the Canvas 2D API: every section's
// title and canvas pixels must match the original page exactly.
test('UV mapping tests: UVsDebug canvases match the original',async({page})=>{
 test.setTimeout(120000);const read=()=>page.evaluate(()=>[...document.querySelectorAll('h3')].map(h=>{const c=h.parentNode.querySelector('canvas');const d=c.getContext('2d').getImageData(0,0,c.width,c.height).data;let hash=0;for(let i=0;i<d.length;i++)hash=(Math.imul(hash,31)+d[i])>>>0;return {title:h.textContent,size:[c.width,c.height],hash,pixels:Array.from(d.filter((_,i)=>i%4===0))};}));
 await page.goto('/reference/three-js/uv-tests.html');await page.waitForFunction(()=>document.querySelectorAll('canvas').length===9);const reference=await read();
 await page.goto('/web/gallery/example.html?id=misc_uv_tests&still=1');await page.waitForFunction(()=>document.querySelectorAll('#uv-tests canvas').length===9,null,{timeout:90000});const rust=await read();
 expect(rust.map(r=>r.title)).toEqual(reference.map(r=>r.title));
 for(let i=0;i<reference.length;i++){let bad=0;for(let p=0;p<reference[i].pixels.length;p++)if(Math.abs(reference[i].pixels[p]-rust[i].pixels[p])>6)bad++;expect({title:reference[i].title,size:rust[i].size,bad}).toEqual({title:reference[i].title,size:reference[i].size,bad:0});}
});

// webgl_effects_ascii writes its frame as characters into a table: the cells of
// the original and the port are compared, with the resource counts across a
// repeated cycle. TrackballControls steps once per 60 fps step.
test('ASCII effect: the character table matches the original',async({page},info)=>{
 test.setTimeout(180000);await page.setViewportSize({width:512,height:512});
 const cells=()=>page.evaluate(()=>document.querySelector('td')?.innerHTML.split('<br>').map(l=>l.replaceAll('&nbsp;',' ')));
 const results={};
 for(const runtime of ['reference','rust']){
  await page.mouse.move(511,0);
  await page.goto(runtime==='reference'?'/reference/three-js/texture-volumes.html?id=webgl_effects_ascii':'/web/gallery/example.html?id=webgl_effects_ascii&still=1');
  await page.waitForFunction(runtime==='reference'?()=>window.renderFixture&&window.reference:()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  const frame=t=>page.evaluate(async({runtime,t})=>{if(runtime==='reference')await renderFixture(t);else{const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}},{runtime,t});
  const shots=[];
  for(const t of [0,.4,1.1,2]){await frame(t);shots.push(await cells());}
  await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(330,300,{steps:5});await page.mouse.up();
  for(let k=1;k<=90;k++)await frame(2+k/60);shots.push(await cells());
  await page.mouse.move(256,256);await page.mouse.wheel(0,-300);for(let k=91;k<=150;k++)await frame(2+k/60);shots.push(await cells());
  results[runtime]=shots;
 }
 const report=results.reference.map((reference,state)=>{const rust=results.rust[state];let total=0,bad=0;expect(rust.length,`state ${state} rows`).toBe(reference.length);for(let y=0;y<reference.length;y++){const a=reference[y],b=rust[y]??'';for(let x=0;x<Math.max(a.length,b.length);x++){total++;if(a[x]!==b[x])bad++;}}return {state,total,bad,fraction:bad/total};});
 writeFileSync(info.outputPath('ascii.json'),JSON.stringify({report,results},null,1));
 for(const r of report)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.01);
 // Steady-state residency: a second time cycle creates no GPU objects or transfers.
 const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
 const cycle=()=>page.evaluate(async()=>{for(const t of [0,1,2,4,0]){const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}});
 await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
});

// webgl_postprocessing_glitch: GlitchPass advances its state and draws random
// numbers once per composer frame, so both runtimes are brought to the same frame
// count and then compared frame by frame, before and after "Glitch me wild".
test('Glitch pass: frames match the original frame by frame',async({page},info)=>{
 test.setTimeout(240000);await page.setViewportSize({width:512,height:512});
 const picks=[1,2,3,5,8,13,21,34,55,70],wild=[71,72,73],after=[74,80];
 const shots={};let frames=0;
 for(const runtime of ['rust','reference']){
  if(runtime==='rust'){await page.goto('/web/gallery/example.html?id=webgl_postprocessing_glitch&still=1');await page.click('#startButton');await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});await page.waitForTimeout(500);frames=Number(await page.locator('canvas').getAttribute('data-frames'));}
  else{await page.goto('/reference/three-js/texture-volumes.html?id=webgl_postprocessing_glitch&samples=1');await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true',null,{timeout:90000});for(let i=1;i<frames;i++)await page.evaluate(()=>renderFixture(0));}
  await page.addStyleTag({content:'#notice,#settings,#info,#overlay{display:none!important}'});
  // The checkbox changes in the same task as the frame it applies to.
  const frame=wild=>page.evaluate(async({runtime,wild})=>{if(wild!==null){if(runtime==='reference'){const e=document.getElementById('wildGlitch');e.checked=wild;e.dispatchEvent(new Event('change'));}else app.tsl_parameter(0,wild?1:0);}if(runtime==='reference')await renderFixture(0);else{const c=document.querySelector('canvas'),p=c.dataset.frames;app.gallery_time(0);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}},{runtime,wild});
  const list=[];
  for(let k=1;k<=after.at(-1);k++){await frame(k===wild[0]?true:k===after[0]?false:null);if([...picks,...wild,...after].includes(k))list.push(PNG.sync.read(await page.locator('canvas').screenshot()));}
  shots[runtime]=list;
  if(runtime==='rust'){
   // Steady state: further frames, with and without wild glitches, create no GPU objects or transfers.
   const read=()=>page.evaluate(()=>({transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(let k=0;k<6;k++)await frame(k===2?true:k===4?false:null);};
   await cycle();const before=await read();await cycle();expect(await read()).toEqual(before);
   // The cycle's frames advance GlitchPass: the reference is aligned to the total.
   frames=Number(await page.locator('canvas').getAttribute('data-frames'))-after.at(-1)-12;
  }
 }
 const results=shots.reference.map((b,state)=>{const a=shots.rust[state];let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));return {state,frames,fraction:bad/(a.width*a.height),meanError:sum/(a.width*a.height*3)};});
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 // The RGB shift and displacement sample the scene target between texels: along
 // polygon edges the filtered values differ between the backends (docs/texture-volumes.md).
 for(const r of results){expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.02);expect(r.meanError,JSON.stringify(r)).toBeLessThanOrEqual(.6);}
});

