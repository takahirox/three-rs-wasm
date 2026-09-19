import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const controls={fog_height:[[0,.075],[1,-3],[0,.001],[1,5]],sprites:[],instance_sprites:[[0,false],[0,true]],tsl_galaxy:[[0,.15],[1,0x11ffff],[2,0xff0044],[0,.01]],postprocessing_afterimage:[[0,.95],[1,false],[1,true],[0,.25]]};
for(const kind of Object.keys(controls)){
 const id=`webgpu_${kind}`;
 test(`TSL particle official rendering and controls: ${kind}`,async({page},info)=>{
  test.setTimeout(180000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
  for(const runtime of ['reference','rust']){
   await page.mouse.move(-10,-10);
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl-particles.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:60000});
   await page.addStyleTag({content:'html,body{background:#000}#notice,#settings{display:none!important}'});
   const frames=[];const capture=async(t,parameter=null,save=true)=>{
    const previous=await page.locator('canvas').getAttribute('data-frames');
    await page.evaluate(async({runtime,t,parameter})=>{
     if(parameter){const [index,value]=parameter;if(runtime==='reference')fixtureParameter(index,value);else{const input=document.querySelector(`#particle-${index}`);if(input.type==='checkbox')input.checked=value;else input.value=input.type==='color'?'#'+value.toString(16).padStart(6,'0'):value;input.dispatchEvent(new Event('input',{bubbles:true}));}}
     if(runtime==='reference')await renderFixture(t);else app.gallery_time(t);
    },{runtime,t,parameter});
    if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);
    if(save)frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
   };
   for(const t of [0,.4,1,2])await capture(t);
   for(const parameter of controls[kind])await capture(2.25,parameter);
   if(['fog_height','tsl_galaxy','instance_sprites'].includes(kind)){
    if(kind==='instance_sprites')await page.mouse.move(360,200);
    else {await page.mouse.move(256,256);await page.mouse.down();await page.mouse.move(296,276);await page.mouse.up();}
    // Match settled input, independent of each backend's event/rAF scheduling.
    for(let i=0;i<140;i++){
     const previous=await page.locator('canvas').getAttribute('data-frames');
     await page.evaluate(async runtime=>{if(runtime==='reference')await renderFixture(2.25);else app.gallery_time(2.25);},runtime);
     if(runtime==='rust')await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,previous);
    }
    await capture(2.25);
   }
   if(kind==='postprocessing_afterimage')await capture(2.5,null,false);
   await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);
   // Rust renders the resize automatically. Give the reference that same
   // first frame before capturing the second frame of empty resized history.
   if(kind==='postprocessing_afterimage'&&runtime==='reference')await page.evaluate(()=>renderFixture(2.5));
   await capture(2.5);images[runtime]=frames;
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
 test(`TSL particle GPU residency: ${kind}`,async({page},info)=>{
  await page.addInitScript(()=>{window.creates=0;for(const key of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createComputePipeline']){const original=GPUDevice.prototype[key];GPUDevice.prototype[key]=function(...args){creates++;return original.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
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


test('TSL particles retain GPU animation and upstream draw counts',async({page},info)=>{
 await page.addInitScript(()=>{
  window.work={passes:0,instances:[],attributeWrites:0};
  const begin=GPUCommandEncoder.prototype.beginRenderPass;
  GPUCommandEncoder.prototype.beginRenderPass=function(...args){work.passes++;return begin.apply(this,args);};
  for(const key of ['draw','drawIndexed']){const draw=GPURenderPassEncoder.prototype[key];GPURenderPassEncoder.prototype[key]=function(...args){if(args[0]>3)work.instances.push(args[1]??1);return draw.apply(this,args);};}
  const write=GPUQueue.prototype.writeBuffer;
  GPUQueue.prototype.writeBuffer=function(buffer,...args){if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX))work.attributeWrites++;return write.call(this,buffer,...args);};
 });
 const reports=[];
 for(const [kind,count,draws,passes] of [['fog_height',100,1,2],['sprites',1,200,2],['instance_sprites',10000,1,2],['tsl_galaxy',20000,1,2],['postprocessing_afterimage',50000,1,3]]){
  await page.goto(`/web/gallery/example.html?id=webgpu_${kind}&still=1`);await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>0);
  const old=await page.locator('canvas').getAttribute('data-frames');await page.evaluate(()=>{work={passes:0,instances:[],attributeWrites:0};app.gallery_time(.7);});await page.waitForFunction(n=>document.querySelector('canvas').dataset.frames!==n,old);
  const work=await page.evaluate(()=>window.work);reports.push({kind,...work});expect(work.passes,kind).toBe(passes);expect(work.instances,kind).toEqual(Array(draws).fill(count));expect(work.attributeWrites,kind).toBe(0);
 }
 writeFileSync(info.outputPath('gpu-work.json'),JSON.stringify(reports,null,2));
});
