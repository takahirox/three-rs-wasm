import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const controls={direct:[[0,.7]],radial_blur:[[0,.5],[1,.8],[2,16],[2,64],[3,2],[4,false],[4,true]],fxaa:[[0,false],[0,true]],ssaa:[[0,0],[0,2],[0,5],[1,'blue'],[2,.3],[3,45]],transition:[[1,false],[2,.3],[2,.7],[3,false],[3,true],[4,0],[4,2],[4,4],[6,.3],[2,0],[2,1]]};
for(const kind of Object.keys(controls)){
 const id=`webgpu_postprocessing_${kind}`;
 test(`TSL filter official rendering and controls: ${kind}`,async({page},info)=>{
  test.setTimeout(180000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
  for(const runtime of ['reference','rust']){
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-filters.html?kind=${kind}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true'||Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:60000});
   await page.addStyleTag({content:'html,body{background:#000}#notice,#settings{display:none!important}'});
   const frames=[];const capture=async t=>{await page.evaluate(({runtime,t})=>runtime==='reference'?renderFixture(t):app.gallery_time(t),{runtime,t});await page.waitForTimeout(70);frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));};
   for(const t of (kind==='transition'?[0,.4,1,2.25,3.25,3.5,4,5.5,6.25,7,8,9.5,10.5]:[0,.4,1,2.25,3.25]))await capture(t);
   // DirectRenderPipeline keeps moving in the official example. Advance its pose
   // for the uniform control too; static legacy materials skip refresh upstream.
   for(const [index,value] of controls[kind]){
    await page.evaluate(({runtime,index,value})=>{if(runtime==='reference')fixtureParameter(index,value);else{const input=document.querySelector(`#filter-${index}`);if(input.type==='checkbox')input.checked=value;else input.value=typeof value==='string'?['black','white','blue','green','red'].indexOf(value):value;input.dispatchEvent(new Event('input',{bubbles:true}));}},{runtime,index,value});await capture(kind==='direct'?3.5:3.25);
   }
   await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);await capture(3.25);images[runtime]=frames;
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
 test(`TSL filter GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  if(kind==='transition')await page.evaluate(()=>{app.tsl_parameter(1,0);app.tsl_parameter(2,.5);});
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

test('TSL filters preserve instancing, SSAA sample counts and inline output',async({page})=>{
 await page.addInitScript(()=>{
  window.frameWork={passes:0,instances:[]};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;
  GPUCommandEncoder.prototype.beginRenderPass=function(...args){frameWork.passes++;return begin.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const draw=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if(args[0]>3)frameWork.instances.push(args[1]??1);return draw.apply(this,args);};}
 });
 for(const [kind,count,passes] of [['direct',1,2],['radial_blur',100,3],['fxaa',100,4],['ssaa',120,18],['transition',500,4]]){
  await page.goto(`/web/gallery/example.html?id=webgpu_postprocessing_${kind}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  if(kind==='transition')await page.evaluate(()=>{app.tsl_parameter(1,0);app.tsl_parameter(2,.5);});
  await page.waitForTimeout(50);
  const render=async()=>{const old=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(()=>{frameWork={passes:0,instances:[]};app.gallery_time(.4);});await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,old);return page.evaluate(()=>frameWork);};
  let work=await render();expect(work.passes,kind).toBe(passes);expect(work.instances.length,kind).toBeGreaterThan(0);expect(work.instances.every(n=>n===count),kind).toBe(true);
  if(kind==='ssaa'){expect(work.instances).toHaveLength(8);await page.evaluate(()=>app.tsl_parameter(0,5));await page.waitForTimeout(50);work=await render();expect(work.instances).toHaveLength(32);}
 }
});
