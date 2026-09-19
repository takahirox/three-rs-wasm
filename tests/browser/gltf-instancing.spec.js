import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
const id='webgl_loader_gltf_instancing';
// This isolates the material/instancing math using the same GPU backend and 1x
// samples. It does NOT accept the remaining original WebGL/MSAA image difference.
test('Instancing material regression on a common backend, plus GPU residency',async({page},info)=>{
 test.setTimeout(120000);
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));const images={};
 await page.addInitScript(()=>{window.creates=0;for(const n of ['createBuffer','createTexture','createBindGroup','createRenderPipeline']){const f=GPUDevice.prototype[n];GPUDevice.prototype[n]=function(...a){creates++;return f.apply(this,a);};}});
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});
  await page.goto(runtime==='reference'?`/reference/three-js/expanded.html?id=${id}&backend=webgpu&samples=1`:`/web/gallery/example.html?id=${id}`);
  await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true'||Number(document.querySelector('canvas')?.dataset.frames)>0,null,{timeout:90000});
  if(runtime==='reference')expect(await page.evaluate(()=>{let scale;reference.scene.traverse(o=>{if(o.isMesh)scale=o.material.normalScale.y;});return scale;})).toBe(1);
  if(runtime==='rust'){await page.evaluate(()=>app.set_samples(1));await page.addStyleTag({content:'#notice,#settings{display:none!important}'});}
  const states=[];
  for(const action of [async()=>{},async()=>{await page.mouse.move(250,250);await page.mouse.down();await page.mouse.move(280,262,{steps:5});await page.mouse.up();},async()=>{await page.mouse.move(250,250);await page.mouse.down({button:'right'});await page.mouse.move(240,255,{steps:3});await page.mouse.up({button:'right'});},async()=>page.mouse.wheel(0,100),async()=>page.setViewportSize({width:640,height:400})]){
   await action();await page.waitForTimeout(120);states.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  }
  images[runtime]=states;
  if(runtime==='rust'){
   const before=await page.evaluate(()=>({creates,frames:document.querySelector('canvas').dataset.frames,transfer:JSON.parse(app.transfer_counts()),resources:Array.from(app.resource_counts())}));await page.waitForTimeout(250);
   expect(await page.evaluate(()=>({creates,frames:document.querySelector('canvas').dataset.frames,transfer:JSON.parse(app.transfer_counts()),resources:Array.from(app.resource_counts())}))).toEqual(before);
   await page.mouse.wheel(0,-100);await page.waitForTimeout(100);
   expect(await page.evaluate(()=>JSON.parse(app.transfer_counts()).slice(0,3))).toEqual(before.transfer.slice(0,3));expect(await page.evaluate(()=>creates)).toBe(before.creates);
  }
 }
 const results=[];for(let state=0;state<images.rust.length;state++){const a=images.rust[state],b=images.reference[state];let count=0;for(let i=0;i<a.data.length;i+=4)if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))count++;results.push({state,fraction:count/(a.width*a.height)});writeFileSync(info.outputPath(`${state}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${state}-reference.png`),PNG.sync.write(b));}
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
});
