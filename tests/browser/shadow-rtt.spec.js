import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.SHADOW_DPR||1)});
const view='canvas';
// Per example: capture times, [control index, reference value, Rust value] at its time, input script.
// Scripted input: [action, ...arguments, capture time or null]. Drags are [x0,y0,x1,y1,button].
const cases={
 webgl_shadowmesh:{times:[0,1,2.5,4],parameters:[[0,0,1,4.5]],restore:[[0,0,1]],script:[['wait',5.5]],at:5.5},
 webgl_instancing_dynamic:{times:[0,1,3.5,4.5,5.5,7],parameters:[],at:7,antialias:true},
 webgl_depth_texture:{times:[0,1],limits:[.005,.8],rebuilds:true,parameters:[[0,1027,1],[1,1015,2],[2,4,4]],restore:[[0,1026,0],[1,1012,0],[2,0,0]],at:1,settle:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
 webgl_rtt:{times:[0,1,2],parameters:[],script:[['move',400,300,null],['wait',3],['move',100,100,null],['wait',4]],at:4,noResize:true},
 webgl_materials_normalmap:{times:[0],parameters:[[1,0.5,0.5],[0,false,0]],restore:[[0,true,1],[1,1,1]],at:0,settle:true,drag:[[256,256],[330,300]],wheel:[256,256,-300]},
};
const official=kind=>kind;
// The original streams the 10,000 instance matrices and colors each frame.
const streams=['webgl_instancing_dynamic'];
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
// WebGL/WebGPU MSAA resolve bounds, documented in docs/lights-probes.md. The same
// scenes must also pass the ordinary threshold with MSAA disabled on both sides.
const msaaLimits={webgl_instancing_dynamic:[.06,.8]};
const frames=(page,runtime,t,n)=>page.evaluate(async({runtime,t,n})=>{for(let i=0;i<n;i++){const c=document.querySelector('canvas'),previous=c.dataset.frames;if(runtime!=='rust')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===previous)await new Promise(r=>requestAnimationFrame(r));}}},{runtime,t,n});
for(const [kind,spec] of Object.entries(cases))for(const samples of spec.antialias?[1,4]:[1])test(`Shadows and render targets official rendering: ${kind} samples=${samples}`,async({page},info)=>{
 test.setTimeout(300000);const images={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.addInitScript(()=>{window.fixtureError=null;const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...a){const d=await request.apply(this,a);d.addEventListener('uncapturederror',e=>window.fixtureError=e.error.message);return d;};addEventListener('error',e=>window.fixtureError=e.message);addEventListener('unhandledrejection',e=>window.fixtureError=String(e.reason));});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.mouse.move(511,0);
  await page.goto(runtime!=='rust'?`/reference/three-js/shadow-rtt.html?id=${official(kind)}&samples=${samples}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
  await page.waitForFunction(v=>{const c=document.querySelector(v);const error=window.fixtureError||c?.dataset.error||document.querySelector('main p')?.textContent;if(error)throw Error(error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view,{timeout:90000});
  if(runtime==='rust'&&samples===1)await page.evaluate(()=>app.set_samples(1));
  await page.addStyleTag({content:'#notice,#settings,#info,#stats,#selectBox,#blocker{display:none!important}'});
  const shots=[];
  const capture=async(t,parameter=null)=>{
   if(parameter)await page.evaluate(({runtime,parameter})=>{const [i,reference,rust]=parameter;if(runtime!=='rust')fixtureParameter(i,reference);else app.tsl_parameter(i,rust);},{runtime,parameter});
   await frames(page,runtime,t,1);
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
 test(`Shadows and render targets GPU residency: ${kind}`,async({page},info)=>{
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

test('Shadows and render targets resident geometry and official draw workload',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.resetWork=()=>window.work={draws:[],attributeBytes:0,transformBytes:0,textureBytes:0};resetWork();
  for(const key of ['draw','drawIndexed']){const fn=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...a){if(a[0]>3||a[1]>1)work.draws.push({count:a[0],instances:a[1]??1});return fn.apply(this,a);};}
  for(const key of ['drawArrays','drawElements']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const count=key==='drawArrays'?a[2]:a[1];// WebGL points become six-vertex WebGPU billboards.
// Draws of three or fewer vertices (the port's fullscreen clear and present triangles, and
// the two-vertex helper line) are left out on both sides.
if(count>3)work.draws.push({count:a[0]===0?count*6:count,instances:1});return fn.apply(this,a);};}
  for(const key of ['drawArraysInstanced','drawElementsInstanced']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){work.draws.push({count:key==='drawArraysInstanced'?a[2]:a[1],instances:key==='drawArraysInstanced'?a[3]:a[4]});return fn.apply(this,a);};}
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(b,offset,data,dataOffset,size){if(b.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX)){if(b.label==='resident draw data'||b.label==='skin/morph input')work.transformBytes+=size??data.byteLength;else work.attributeBytes+=size??data.byteLength;}return write.apply(this,arguments);};
  // WebGL attribute uploads: the original streams its dynamic attributes with bufferSubData.
  for(const key of ['bufferData','bufferSubData']){const fn=WebGL2RenderingContext.prototype[key];WebGL2RenderingContext.prototype[key]=function(...a){const data=key==='bufferData'?a[1]:a[2];if(typeof data==='object'&&data)work.attributeBytes+=key==='bufferSubData'&&a[4]?a[4]*data.BYTES_PER_ELEMENT:data.byteLength;return fn.apply(this,a);};}
  const texture=GPUQueue.prototype.writeTexture;GPUQueue.prototype.writeTexture=function(dest,data,...args){work.textureBytes+=data.byteLength;return texture.call(this,dest,data,...args);};
 });
 const report=[];
 for(const [kind,spec] of Object.entries(cases)){
  const pair={kind};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(511,0);
   await page.goto(runtime==='reference'?`/reference/three-js/shadow-rtt.html?id=${official(kind)}`:`/web/gallery/example.html?id=${official(kind)}&still=1`);
   await page.waitForFunction(v=>{const c=document.querySelector(v);if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},view);
   for(const [n,t] of [1,2,3,1,2,3].entries()){if(n===5)await page.evaluate(()=>resetWork());await frames(page,runtime,t,1);}
   pair[runtime]=await page.evaluate(()=>work);
  }
  report.push(pair);const sort=a=>a.map(x=>x.count*x.instances).sort((a,b)=>a-b);
  // The port draws the depth post pass with a fullscreen triangle; WebGL uses a 6-index quad.
  const referenceDraws=kind==='webgl_depth_texture'?pair.reference.draws.filter(d=>d.count!==6):pair.reference.draws;
  expect.soft(sort(pair.rust.draws),kind).toEqual(sort(referenceDraws));
  // Each instance's matrix and color stream together (80 bytes) as resident draw data;
  // WebGL streams the 64-byte matrices, and the colors only during a tween.
  if(streams.includes(kind))expect.soft(pair.rust.attributeBytes+pair.rust.transformBytes,kind).toBeLessThanOrEqual(pair.reference.attributeBytes*1.25);
  else expect.soft(pair.rust.attributeBytes,kind).toBe(0);
  expect.soft(pair.rust.textureBytes,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(report,null,2));
});

for(const kind of [])test(`Shadows and render targets on-demand scene stays idle: ${kind}`,async({page})=>{
 await page.goto(`/web/gallery/example.html?id=${kind}`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
 await page.waitForTimeout(300);const frames=await page.locator('canvas').getAttribute('data-frames');await page.waitForTimeout(300);expect(await page.locator('canvas').getAttribute('data-frames')).toBe(frames);
 await page.setViewportSize({width:640,height:400});await page.waitForFunction(prev=>document.querySelector('canvas').dataset.frames!==prev,frames);
});


