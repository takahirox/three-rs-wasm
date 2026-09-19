import{test,expect}from'@playwright/test';
import{writeFileSync,readFileSync}from'node:fs';
import{PNG}from'pngjs';
const manifest=JSON.parse(readFileSync(new URL('../gltf-pbr/manifest.json',import.meta.url)));
for(let m=0;m<2;m++)test(`M2 PBR parity: ${manifest.models[m].name}`,async({page},info)=>{
 test.setTimeout(240000);
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));page.on('console',e=>{if((e.type()==='error'&&!e.text().includes('404'))||(e.type()==='warning'&&!e.text().startsWith('THREE.'))) {if(errors.length<3)errors.push(e.text());}});
 await page.goto('/reference/three-js/gltf-pbr.html');const canvas=page.locator('canvas');
 await expect(canvas).toHaveAttribute('data-ready','true',{timeout:120000});
 const references=[];
 for(let v=0;v<manifest.views.length;v++) {await page.evaluate(([m,v])=>window.renderFixture(m,v),[m,v]);references.push(await canvas.screenshot());}
 await page.goto(`/web/?example=${m+4}&fixture=1`);


 await expect.poll(async()=>Number(await canvas.getAttribute('data-frames')),{timeout:120000}).toBeGreaterThan(2);
 const results=[];
 for(let v=0;v<manifest.views.length;v++) {
  const view=manifest.views[v];await page.evaluate(v=>window.app.gltf_view(v.yaw,v.pitch,v.distance_factor,v.exposure,v.rotation,v.blur),view);
  const frames=Number(await canvas.getAttribute('data-frames'));await expect.poll(async()=>Number(await canvas.getAttribute('data-frames'))).toBeGreaterThan(frames+2);
  const actual=await canvas.screenshot();writeFileSync(info.outputPath(`${v}-actual.png`),actual);writeFileSync(info.outputPath(`${v}-reference.png`),references[v]);
  const a=PNG.sync.read(actual),b=PNG.sync.read(references[v]);expect([a.width,a.height]).toEqual([b.width,b.height]);
  let different=0,maximum=0;for(let i=0;i<a.data.length;i+=4) {let d=0;for(let c=0;c<3;c++)d=Math.max(d,Math.abs(a.data[i+c]-b.data[i+c]));maximum=Math.max(maximum,d);if(d>manifest.comparison.channel_tolerance)different++;}
  const difference=new PNG({width:a.width,height:a.height});for(let i=0;i<a.data.length;i+=4){for(let c=0;c<3;c++)difference.data[i+c]=Math.min(255,Math.abs(a.data[i+c]-b.data[i+c])*8);difference.data[i+3]=255;}writeFileSync(info.outputPath(`${v}-difference.png`),PNG.sync.write(difference));
  results.push({view:view.name,maximum,fraction:different/(a.width*a.height)});
 }
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));expect(errors).toEqual([]);
 for(const result of results)expect(result.fraction,JSON.stringify(result)).toBeLessThanOrEqual(manifest.comparison.maximum_different_pixel_fraction);
});

for(const size of [{width:640,height:360},{width:360,height:640}])test(`gallery glTF keeps proportions on initial load: ${size.width}x${size.height}`,async({page},info)=>{
 test.setTimeout(120000);
 await page.setViewportSize(size);
 await page.goto('/reference/three-js/gltf-pbr.html');
 const canvas=page.locator('canvas');
 await expect(canvas).toHaveAttribute('data-ready','true',{timeout:90000});
 await page.evaluate(async size=>{
  const {renderer,camera}=window.reference;
  renderer.setSize(size.width,size.height);
  camera.aspect=size.width/size.height;
  await window.renderFixture(0,0);
 },size);
 const reference=PNG.sync.read(await canvas.screenshot());
 await page.goto('/web/gallery/example.html?id=webgpu_loader_gltf');
 await expect.poll(()=>canvas.getAttribute('data-frames').then(Number),{timeout:90000}).toBeGreaterThan(2);
 await page.addStyleTag({content:'#settings,#notice{display:none!important}'});
 await page.evaluate(v=>window.app.gltf_view(v.yaw,v.pitch,v.distance_factor,v.exposure,v.rotation,v.blur),manifest.views[0]);
 const frame=Number(await canvas.getAttribute('data-frames'));
 await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(frame+2);
 const actual=PNG.sync.read(await canvas.screenshot());
 expect([actual.width,actual.height]).toEqual([reference.width,reference.height]);
 let different=0;
 for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-reference.data[i+c])>manifest.comparison.channel_tolerance))different++;
 writeFileSync(info.outputPath('actual.png'),PNG.sync.write(actual));
 writeFileSync(info.outputPath('reference.png'),PNG.sync.write(reference));
 expect(different/(actual.width*actual.height)).toBeLessThanOrEqual(manifest.comparison.maximum_different_pixel_fraction);
});
