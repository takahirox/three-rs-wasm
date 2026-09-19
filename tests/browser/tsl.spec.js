import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const ids=['webgpu_tsl_interoperability','webgpu_texturegrad','webgpu_procedural_texture'];
for(const id of ids){
 test(`TSL official rendering and controls: ${id}`,async({page},info)=>{
  test.setTimeout(180000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));
  const images={};
  for(const runtime of ['reference','rust']){
   await page.setViewportSize({width:512,height:512});
   await page.goto(runtime==='reference'?`/reference/three-js/tsl.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
   await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true'||Number(document.querySelector('canvas')?.dataset.frames)>2,null,{timeout:60000});
   if(runtime==='rust')await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
   const capture=async()=>{await page.waitForTimeout(100);return PNG.sync.read(await page.locator('canvas').screenshot());};
   const states=[];
   for(const time of [0,1,3]){
    await page.evaluate(({runtime,time})=>runtime==='reference'?renderFixture(time):app.gallery_time(time),{runtime,time});
    states.push(await capture());
   }
   const controls=id==='webgpu_tsl_interoperability'?[[1,0,15],[2,1,.9],[3,2,2],[4,3,.4],[5,4,30],[6,5,4],[7,6,3]]:id==='webgpu_procedural_texture'?[[1,0,3.3],[2,1,1.5],[3,2,false],[1,0,8],[2,1,0],[3,2,true]]:[];
   for(const [index,referenceIndex,value] of controls){
    await page.evaluate(({runtime,index,referenceIndex,value})=>{if(runtime==='rust'){const input=document.getElementById(typeof value==='boolean'?'tsl-auto':`tsl-${index}`);if(typeof value==='boolean')input.checked=value;else input.value=value;input.dispatchEvent(new Event(typeof value==='boolean'?'change':'input',{bubbles:true}));}else {fixtureParameter(referenceIndex,value);return renderFixture(3);}}, {runtime,index,referenceIndex,value});
    states.push(await capture());
   }
   await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);
   if(runtime==='reference')await page.evaluate(()=>renderFixture(3));
   states.push(await capture());images[runtime]=states;
  }
  const results=[];
  for(let i=0;i<images.rust.length;i++){
   const a=images.rust[i],b=images.reference[i];expect([a.width,a.height]).toEqual([b.width,b.height]);let count=0,sum=0;
   for(let p=0;p<a.data.length;p+=4){let bad=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;bad ||= d>6;}if(bad)count++;}
   results.push({state:i,fraction:count/(a.width*a.height),meanError:sum/(a.width*a.height*3)});
   writeFileSync(info.outputPath(`${i}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${i}-reference.png`),PNG.sync.write(b));
  }
  writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
  for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
 });
 test(`TSL GPU residency: ${id}`,async({page})=>{
  const errors=[];page.on('pageerror',e=>errors.push(String(e)));
  await page.addInitScript(()=>{window.creates=0;for(const name of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createRenderPipelineAsync']){const fn=GPUDevice.prototype[name];GPUDevice.prototype[name]=function(...args){creates++;return fn.apply(this,args);};}});
  await page.goto(`/web/gallery/example.html?id=${id}`);
  await page.waitForFunction(()=>Number(document.querySelector('canvas')?.dataset.frames)>12,null,{timeout:60000});
  for(const resized of [false,true]){
   if(resized){await page.setViewportSize({width:640,height:400});await page.waitForTimeout(200);}
   const read=()=>page.evaluate(()=>({creates,transfers:JSON.parse(app.transfer_counts()).slice(0,3),resources:Array.from(app.resource_counts())}));
   const before=await read();await page.waitForTimeout(350);expect(await read()).toEqual(before);
  }
  expect(errors).toEqual([]);await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
 });
}

test('TSL gallery loads matching versioned glue and Wasm instead of stale URLs',async({page})=>{
 const requests=[];page.on('request',request=>requests.push(new URL(request.url())));
 // Emulate an obsolete entrypoint still available at the previous cached URL.
 await page.route('**/pkg/three_rs_wasm.js',route=>route.fulfill({contentType:'text/javascript',body:'export default async function(){}; export class BrowserApp {static create(){throw Error("invalid example id");}}'}));
 await page.goto('/web/gallery/#webgpu_texturegrad');
 const frame=page.frameLocator('#viewer');
 await expect(frame.locator('body')).toHaveAttribute('data-backend','rust-wasm-webgpu');
 await expect.poll(async()=>Number(await frame.locator('canvas').getAttribute('data-frames'))).toBeGreaterThan(2);
 const glue=requests.find(url=>url.pathname.endsWith('/pkg/three_rs_wasm.js'));
 const wasm=requests.find(url=>url.pathname.endsWith('/pkg/three_rs_wasm_bg.wasm'));
 expect(glue.searchParams.get('v')).toBeTruthy();
 expect(wasm.searchParams.get('v')).toBe(glue.searchParams.get('v'));
 await expect(frame.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
});
