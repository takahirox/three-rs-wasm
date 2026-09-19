import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const ids=['webgpu_compute_texture','webgpu_rtt','webgpu_postprocessing','webgpu_postprocessing_difference','webgpu_postprocessing_masking'];
for(const id of ids){
 test.describe(id,()=>{
 // RTT pointer coordinates are CSS pixels even when targets have twice the resolution.
 test.use({deviceScaleFactor:id==='webgpu_rtt'?2:1});
 test(`TSL pass rendering: ${id}`,async({page},info)=>{
  test.setTimeout(120000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
  for(const runtime of ['reference','rust']){
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-passes.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true'||Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:60000});
   await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
   const states=[];
   const frame=async time=>{await page.evaluate(({runtime,time})=>runtime==='reference'?renderFixture(time):app.gallery_time(time),{runtime,time});await page.waitForTimeout(100);states.push(PNG.sync.read(await page.locator('canvas').screenshot()));};
   for(const time of [0,.2,.4,.4,1])await frame(time);
   if(id==='webgpu_rtt')for(const [x,y] of [[128,128],[384,384]]){await page.mouse.move(x,y);await frame(1);}
   await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);
   // Both history targets must have been rendered at their new size before
   // comparing the settled output; resize-triggered frame counts differ.
   if(id==='webgpu_postprocessing_difference')for(let i=0;i<3;i++){await page.evaluate(({runtime})=>runtime==='reference'?renderFixture(1):app.gallery_time(1),{runtime});await page.waitForTimeout(30);}await frame(1);await frame(1.2);
   images[runtime]=states;
  }
  const results=[];
  for(let state=0;state<images.rust.length;state++){
   const a=images.rust[state],b=images.reference[state];expect([a.width,a.height]).toEqual([b.width,b.height]);let different=0,total=0;
   for(let p=0;p<a.data.length;p+=4){let bad=false;for(let c=0;c<3;c++){const error=Math.abs(a.data[p+c]-b.data[p+c]);bad ||= error>6;total+=error;}if(bad)different++;}
   results.push({state,fraction:different/(a.width*a.height),meanError:total/(a.width*a.height*3)});
   writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));
  }
  writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
  for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
 });
 test(`TSL pass GPU residency: ${id}`,async({page})=>{
  await page.addInitScript(()=>{window.creates=0;window.computeDispatches=0;for(const n of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[n];GPUDevice.prototype[n]=function(...args){creates++;return original.apply(this,args);};}const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(...args){computeDispatches++;return dispatch.apply(this,args);};});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  for(const resized of [false,true]){
   if(resized)await page.setViewportSize({width:640,height:400});await page.waitForTimeout(300);
   const snapshot=()=>page.evaluate(()=>({creates,computeDispatches,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const cycle=async()=>{for(const t of [0,.5,1,2,4,8,12,0]){const previous=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(t=>app.gallery_time(t),t);await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);}};
   await cycle();const before=await snapshot();await cycle();expect(await snapshot()).toEqual(before);
  }
  await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
  if(id==='webgpu_compute_texture')expect(await page.evaluate(()=>computeDispatches)).toBe(1);
 });
 });
}

test('Difference speed and Orbit controls update the displayed scene',async({page})=>{
 await page.goto('/web/gallery/example.html?id=webgpu_postprocessing_difference');
 await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>10);
 await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
 const capture=()=>page.locator('canvas').screenshot();
 const still=await capture();await page.waitForTimeout(100);expect(await capture()).toEqual(still);
 const speed=async value=>page.evaluate(value=>{const input=document.querySelector('#tsl-speed');input.value=value;input.dispatchEvent(new Event('input',{bubbles:true}));},value);
 await speed(1);await page.waitForTimeout(200);expect(await capture()).not.toEqual(still);
 await speed(0);await page.waitForTimeout(100);const stopped=await capture();await page.waitForTimeout(100);expect(await capture()).toEqual(stopped);
 await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(330,300,{steps:4});await page.mouse.up();await page.waitForTimeout(100);
 expect(await capture()).not.toEqual(stopped);
 await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
});
