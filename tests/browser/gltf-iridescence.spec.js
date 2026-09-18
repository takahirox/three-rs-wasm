import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {readFileSync,writeFileSync} from 'node:fs';
const id='webgpu_loader_gltf_iridescence';
async function candidate(page){
 const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));
 expect(catalog.examples.find(e=>e.id===id)?.port?.example).toBe(29);
}

test('Iridescence glTF: original material, automatic orbit and manual camera views',async({page},info)=>{
 test.setTimeout(120000);await candidate(page);const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 const screenshots={};
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});
  await page.goto(runtime==='reference'?`/reference/three-js/gltf-examples.html?id=${id}`:`/web/gallery/example.html?id=${id}&still=1`);
  const canvas=page.locator('canvas');
  if(runtime==='reference')await expect(canvas).toHaveAttribute('data-ready','true',{timeout:60000});
  else {await expect.poll(()=>canvas.getAttribute('data-frames').then(Number),{timeout:60000}).toBeGreaterThan(2);await page.addStyleTag({content:'#notice,#settings{display:none!important}'});}
  const result=[];
  for(const action of [
   async()=>{},
   async()=>page.evaluate(runtime=>runtime==='reference'?renderFixture(3):app.gallery_time(3),runtime),
   async()=>page.evaluate(runtime=>runtime==='reference'?renderFixture(11):app.gallery_time(11),runtime),
   async()=>{await page.mouse.move(250,250);await page.mouse.down();await page.mouse.move(280,260,{steps:5});await page.mouse.up();},
   async()=>{await page.mouse.move(250,250);await page.mouse.down({button:'right'});await page.mouse.move(240,255,{steps:3});await page.mouse.up({button:'right'});},
   async()=>page.mouse.wheel(0,100),
   async()=>page.setViewportSize({width:640,height:400}),
  ]){await action();await page.waitForTimeout(100);result.push(PNG.sync.read(await canvas.screenshot()));}
  screenshots[runtime]=result;
 }
 const results=[];
 for(let n=0;n<screenshots.rust.length;n++){
  const a=screenshots.rust[n],b=screenshots.reference[n];expect([a.width,a.height]).toEqual([b.width,b.height]);let different=0;
  for(let i=0;i<a.data.length;i+=4)if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))different++;
  results.push({state:n,fraction:different/(a.width*a.height)});
  writeFileSync(info.outputPath(`${n}-actual.png`),PNG.sync.write(a));writeFileSync(info.outputPath(`${n}-reference.png`),PNG.sync.write(b));
 }
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 for(const r of results)expect(r.fraction,JSON.stringify(r)).toBeLessThanOrEqual(.005);
});
test('Iridescence retains transmission draw resources and compact mipmapped extension textures',async({page})=>{
 await candidate(page);
 await page.addInitScript(()=>{
  window.allocations={creates:0,physical:[],destroyed:0};
  for(const name of ['createBuffer','createBindGroup','createTexture','createRenderPipeline']){
   const f=GPUDevice.prototype[name];GPUDevice.prototype[name]=function(...a){allocations.creates++;if(name==='createTexture'&&a[0].label==='physical extension maps')allocations.physical.push(a[0]);return f.apply(this,a);};
  }
  const destroy=GPUDevice.prototype.destroy;GPUDevice.prototype.destroy=function(){allocations.destroyed++;return destroy.call(this);};
 });
 await page.goto(`/web/gallery/example.html?id=${id}`);const canvas=page.locator('canvas');
 await expect.poll(()=>canvas.getAttribute('data-frames').then(Number),{timeout:60000}).toBeGreaterThan(10);
 const firstImage=await canvas.screenshot();
 const before=await page.evaluate(()=>({creates:allocations.creates,transfer:JSON.parse(app.transfer_counts()),textures:Array.from(app.resource_counts()),physical:allocations.physical}));
 const mapped=before.physical.filter(t=>t.size.width>1);expect(mapped).toHaveLength(1);
 expect(mapped[0].size).toEqual({width:2048,height:2048,depthOrArrayLayers:1});expect(mapped[0].mipLevelCount).toBe(12);
 await page.waitForTimeout(400);
 const after=await page.evaluate(()=>({creates:allocations.creates,transfer:JSON.parse(app.transfer_counts()),textures:Array.from(app.resource_counts())}));
 expect(after.creates).toBe(before.creates);expect(after.transfer.slice(0,3)).toEqual(before.transfer.slice(0,3));expect(after.textures).toEqual(before.textures);expect((await canvas.screenshot()).equals(firstImage)).toBe(false);
 // Hold a pointer down without moving it: the original stops auto-rotation during interaction.
 await page.mouse.move(250,250);await page.mouse.down();await page.waitForTimeout(80);
 const held=await canvas.screenshot();await page.waitForTimeout(100);expect((await canvas.screenshot()).equals(held)).toBe(true);await page.mouse.up();
 await page.evaluate(()=>dispatchEvent(new Event('pagehide')));expect(await page.evaluate(()=>allocations.destroyed)).toBe(1);
});
