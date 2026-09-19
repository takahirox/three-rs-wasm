import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync,readFileSync} from 'node:fs';
const cases=[
 {id:'webgl_buffergeometry',example:26,times:[0,3,11]},
 {id:'webgl_buffergeometry_rawshader',example:27,times:[0,.3,2.1]},
 {id:'webgl_morphtargets_horse',example:24,times:[0,.25,.7,2.4]},
 {id:'webgl_morphtargets_sphere',example:25,times:[0,1,2,3.5]},
 {id:'webgl_buffergeometry_indexed',example:22,times:[0,3,11]},
 {id:'webgl_lines_colors',example:23,times:[0,3,11]},
 ...(process.env.EXPANDED_PENDING?[{id:'webgl_loader_gltf_instancing',example:16,times:[0]}]:[]),
 {id:'webgl_geometries',example:17,times:[0,3,11]},
 {id:'webgl_lines_dashed',example:19,times:[0,3,11]},
 {id:'webgl_geometry_colors',example:21,times:[0]},
 ...['webgpu_morphtargets'].map(id=>({id,example:18,times:[0,0,0,0],weights:[[0,0],[1,0],[0,1],[.4,.7]]})),
];
for(const entry of cases)test(`additional scene and GPU resources: ${entry.id}`,async({page},info)=>{
 test.setTimeout(120000);await page.setViewportSize({width:512,height:512});
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto(`/reference/three-js/expanded.html?id=${entry.id}`);
 const canvas=page.locator('canvas');
 await expect(canvas).toHaveAttribute('data-ready','true',{timeout:90000});
 const references=[];
 for(let i=0;i<entry.times.length;i++){await page.evaluate(([t,w])=>window.renderFixture(t,w),[entry.times[i],entry.weights?.[i]]);references.push(PNG.sync.read(await canvas.screenshot()));}
 // Pending ports are reachable for validation, but never shown in the product gallery.
 const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));const row=catalog.examples.find(e=>e.id===entry.id);
 if(entry.example===16){row.port={example:entry.example,limitations:[]};row.status='partial';await page.route('**/web/gallery/catalog.json',r=>r.fulfill({json:catalog}));}
 else expect(row.port?.example).toBe(entry.example);
 await page.goto(`/web/gallery/example.html?id=${entry.id}&still=1`);
 await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
 await expect.poll(()=>canvas.getAttribute('data-frames').then(Number),{timeout:90000}).toBeGreaterThan(3);
 const results=[];
 for(let v=0;v<entry.times.length;v++){
  await page.evaluate(t=>window.app.gallery_time(t),entry.times[v]);
  if(entry.weights)await page.evaluate(w=>window.app.gallery_morph(...w),entry.weights[v]);
  const frame=Number(await canvas.getAttribute('data-frames'));await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(frame+2);
  const actual=PNG.sync.read(await canvas.screenshot()),reference=references[v];
  expect([actual.width,actual.height]).toEqual([reference.width,reference.height]);
  let different=0;for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-reference.data[i+c])>6))different++;
  writeFileSync(info.outputPath(`${v}-actual.png`),PNG.sync.write(actual));writeFileSync(info.outputPath(`${v}-reference.png`),PNG.sync.write(reference));
  results.push({time:entry.times[v],weights:entry.weights?.[v],fraction:different/(actual.width*actual.height)});
 }
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(results,null,2));
 const before=await page.evaluate(()=>({transfer:JSON.parse(window.app.transfer_counts()),textures:Array.from(window.app.resource_counts())}));await page.waitForTimeout(250);
 expect(await page.evaluate(()=>({transfer:JSON.parse(window.app.transfer_counts()),textures:Array.from(window.app.resource_counts())}))).toEqual(before);
 await expect(canvas).not.toHaveAttribute('data-error',/.+/);expect(errors).toEqual([]);
 for(const result of results)expect(result.fraction,JSON.stringify(result)).toBeLessThanOrEqual(.005);
});

test('morph sliders, orbit, pan and disabled zoom match the original',async({page},info)=>{
 test.setTimeout(120000);await page.setViewportSize({width:512,height:512});
 const gesture=async()=>{
  await page.mouse.move(240,250);await page.mouse.down();await page.mouse.move(290,280,{steps:5});await page.mouse.up();
  await page.mouse.move(250,250);await page.mouse.down({button:'right'});await page.mouse.move(235,265,{steps:3});await page.mouse.up({button:'right'});
  await page.mouse.wheel(0,300);await page.waitForTimeout(150);
 };
 await page.goto('/reference/three-js/expanded.html?id=webgpu_morphtargets');const canvas=page.locator('canvas');await expect(canvas).toHaveAttribute('data-ready','true');
 await page.evaluate(()=>window.renderFixture(0,[.25,.75]));await gesture();
 const reference=PNG.sync.read(await canvas.screenshot());
 writeFileSync(info.outputPath('reference-camera.json'),JSON.stringify(await page.evaluate(()=>({position:window.reference.camera.position.toArray(),quaternion:window.reference.camera.quaternion.toArray(),size:[document.querySelector('canvas').clientWidth,document.querySelector('canvas').clientHeight]}))));
 const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));Object.assign(catalog.examples.find(e=>e.id==='webgpu_morphtargets'),{status:'partial',port:{example:18,limitations:[]}});
 await page.route('**/web/gallery/catalog.json',r=>r.fulfill({json:catalog}));
 await page.goto('/web/gallery/example.html?id=webgpu_morphtargets');await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(3);
 const before=await page.evaluate(()=>JSON.parse(app.transfer_counts()));
 for(const [id,value] of [['spherify',.25],['twist',.75]])await page.locator('#'+id).evaluate((input,value)=>{input.value=value;input.dispatchEvent(new Event('input',{bubbles:true}));},value);
 await page.addStyleTag({content:'#notice,#settings{display:none!important}'});await gesture();
 const after=await page.evaluate(()=>JSON.parse(app.transfer_counts()));expect(after.slice(0,3)).toEqual(before.slice(0,3));expect(after[3]-before[3]).toBeGreaterThan(0);expect(after[3]-before[3]).toBeLessThanOrEqual(32);
 writeFileSync(info.outputPath('actual-camera.json'),JSON.stringify(await page.evaluate(()=>({nodes:JSON.parse(app.scene_json()).children.map(n=>({kind:Object.keys(n.kind),position:n.position,quaternion:n.quaternion})),size:[document.querySelector('canvas').clientWidth,document.querySelector('canvas').clientHeight]}))));
 const actual=PNG.sync.read(await canvas.screenshot());let different=0;
 for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-reference.data[i+c])>6))different++;
 writeFileSync(info.outputPath('actual.png'),PNG.sync.write(actual));writeFileSync(info.outputPath('reference.png'),PNG.sync.write(reference));
 expect(different/(actual.width*actual.height)).toBeLessThanOrEqual(.005);
});

for(const [id,example] of [['webgl_geometry_colors',21],['webgl_lines_colors',23]])test(`${id} pointer camera follows the original`,async({page},info)=>{
 test.setTimeout(120000);await page.setViewportSize({width:512,height:512});
 const target=[64,id==='webgl_lines_colors'?266:66];
 await page.goto(`/reference/three-js/expanded.html?id=${id}&animate=1`);const canvas=page.locator('canvas');await expect(canvas).toHaveAttribute('data-ready','true');
 await page.mouse.move(320,190);
 await expect.poll(()=>page.evaluate(([x,y])=>Math.hypot(reference.camera.position.x-x,reference.camera.position.y-y),target)).toBeLessThan(.01);
 await page.evaluate(async()=>{reference.renderer.setAnimationLoop(null);await renderFixture(3);});const expected=PNG.sync.read(await canvas.screenshot());
 const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));Object.assign(catalog.examples.find(e=>e.id===id),{status:'partial',port:{example,limitations:[]}});await page.route('**/web/gallery/catalog.json',r=>r.fulfill({json:catalog}));
 await page.goto(`/web/gallery/example.html?id=${id}`);await page.addStyleTag({content:'#notice,#settings{display:none!important}'});await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(3);
 await page.mouse.move(319,190);await page.mouse.move(320,190);
 await expect.poll(()=>page.evaluate(([x,y])=>{const camera=JSON.parse(app.scene_json()).children.find(n=>'Camera' in n.kind);return Math.hypot(camera.position[0]-x,camera.position[1]-y);},target)).toBeLessThan(.01);
 await page.evaluate(()=>app.gallery_time(3));await page.waitForTimeout(100);const actual=PNG.sync.read(await canvas.screenshot());let different=0;
 for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-expected.data[i+c])>6))different++;
 writeFileSync(info.outputPath('actual.png'),PNG.sync.write(actual));writeFileSync(info.outputPath('reference.png'),PNG.sync.write(expected));expect(different/(actual.width*actual.height)).toBeLessThanOrEqual(.005);
});

test('indexed mesh wireframe toggle matches the original',async({page},info)=>{
 await page.setViewportSize({width:512,height:512});await page.goto('/reference/three-js/expanded.html?id=webgl_buffergeometry_indexed');const canvas=page.locator('canvas');await expect(canvas).toHaveAttribute('data-ready','true');
 await page.evaluate(async()=>{reference.scene.traverse(o=>{if(o.isMesh)o.material.wireframe=true;});await renderFixture(3);});const expected=PNG.sync.read(await canvas.screenshot());
 const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));Object.assign(catalog.examples.find(e=>e.id==='webgl_buffergeometry_indexed'),{status:'partial',port:{example:22,limitations:[]}});await page.route('**/web/gallery/catalog.json',r=>r.fulfill({json:catalog}));
 await page.goto('/web/gallery/example.html?id=webgl_buffergeometry_indexed&still=1');await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(3);await page.locator('#wireframe').check();await page.evaluate(()=>app.gallery_time(3));await page.addStyleTag({content:'#notice,#settings{display:none!important}'});await page.waitForTimeout(100);
 const actual=PNG.sync.read(await canvas.screenshot());let different=0;for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-expected.data[i+c])>6))different++;
 writeFileSync(info.outputPath('actual.png'),PNG.sync.write(actual));writeFileSync(info.outputPath('reference.png'),PNG.sync.write(expected));expect(different/(actual.width*actual.height)).toBeLessThanOrEqual(.005);
});

for(const [id,example] of [['webgl_morphtargets_sphere',25]])test(`${id} orbit, zoom and pan match the original`,async({page},info)=>{
 test.setTimeout(120000);await page.setViewportSize({width:512,height:512});
 const gesture=async()=>{
  await page.mouse.move(240,250);await page.mouse.down();await page.mouse.move(280,274,{steps:4});await page.mouse.up();
  await page.mouse.wheel(0,example===25?10000:400);await page.waitForTimeout(100);
  await page.mouse.move(250,250);await page.mouse.down({button:'right'});await page.mouse.move(235,260,{steps:3});await page.mouse.up({button:'right'});await page.waitForTimeout(100);
 };
 await page.goto(`/reference/three-js/expanded.html?id=${id}`);const canvas=page.locator('canvas');await expect(canvas).toHaveAttribute('data-ready','true');await gesture();writeFileSync(info.outputPath('reference-camera.json'),JSON.stringify(await page.evaluate(()=>({position:reference.camera.position.toArray(),quaternion:reference.camera.quaternion.toArray()}))));const reference=PNG.sync.read(await canvas.screenshot());
 await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(3);await page.addStyleTag({content:'#notice,#settings{display:none!important}'});await gesture();writeFileSync(info.outputPath('actual-camera.json'),JSON.stringify(await page.evaluate(()=>JSON.parse(app.scene_json()).children.find(n=>'Camera' in n.kind))));const actual=PNG.sync.read(await canvas.screenshot());let different=0;
 for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-reference.data[i+c])>6))different++;
 writeFileSync(info.outputPath('actual.png'),PNG.sync.write(actual));writeFileSync(info.outputPath('reference.png'),PNG.sync.write(reference));expect(different/(actual.width*actual.height)).toBeLessThanOrEqual(.005);
});

for(const id of ['webgl_buffergeometry','webgl_buffergeometry_rawshader'])test(`${id} wide viewport and animated residency`,async({page},info)=>{
 test.setTimeout(120000);await page.setViewportSize({width:800,height:450});
 await page.goto(`/reference/three-js/expanded.html?id=${id}`);
 const canvas=page.locator('canvas');await expect(canvas).toHaveAttribute('data-ready','true');
 await page.evaluate(async()=>{reference.renderer.setSize(800,450);reference.camera.aspect=800/450;reference.camera.updateProjectionMatrix();await renderFixture(1.2);});
 const reference=PNG.sync.read(await canvas.screenshot());
 await page.goto(`/web/gallery/example.html?id=${id}&still=1`);await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(3);
 await page.addStyleTag({content:'#notice,#settings{display:none!important}'});await page.evaluate(()=>app.gallery_time(1.2));
 const frame=Number(await canvas.getAttribute('data-frames'));await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(frame+2);
 const actual=PNG.sync.read(await canvas.screenshot());let different=0;
 expect([actual.width,actual.height]).toEqual([800,450]);
 for(let i=0;i<actual.data.length;i+=4)if([0,1,2].some(c=>Math.abs(actual.data[i+c]-reference.data[i+c])>6))different++;
 writeFileSync(info.outputPath('wide-actual.png'),PNG.sync.write(actual));writeFileSync(info.outputPath('wide-reference.png'),PNG.sync.write(reference));
 expect(different/(800*450)).toBeLessThanOrEqual(.005);
 // Run real animation rather than measuring only a frozen fixture.
 await page.goto(`/web/gallery/example.html?id=${id}`);await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(10);
 const counts=()=>page.evaluate(()=>({transfer:JSON.parse(app.transfer_counts()),resources:Array.from(app.resource_counts())}));
 const before=await counts();expect(before.transfer[0]).toBe(1); // Shared back/front geometry is uploaded once.
 const first=await canvas.screenshot(),start=Number(await canvas.getAttribute('data-frames'));
 await expect.poll(()=>canvas.getAttribute('data-frames').then(Number)).toBeGreaterThan(start+30);
 expect(await counts()).toEqual(before);expect((await canvas.screenshot()).equals(first)).toBe(false);
 await expect(canvas).not.toHaveAttribute('data-error',/.+/);
});
