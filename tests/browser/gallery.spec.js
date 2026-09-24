import {test,expect} from '@playwright/test';
import {readFileSync,writeFileSync} from 'node:fs';
import {PNG} from 'pngjs';
const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));
const files=readFileSync('web/gallery/files.json','utf8');
test('equivalent WebGL scenes are excluded and point to the preferred WebGPU scene',async({page})=>{
 const listed=Object.values(JSON.parse(files)).flat();
 const excluded=catalog.examples.filter(e=>e.preferred_example);
 expect(excluded).toHaveLength(14);
 for(const entry of excluded){
  expect(entry.status).toBe('excluded');expect(entry.port).toBeNull();
  expect(listed).not.toContain(entry.id);
  const preferred=catalog.examples.find(e=>e.id===entry.preferred_example);
  expect(preferred).toBeDefined();
  if(preferred.port)expect(listed).toContain(preferred.id);
 }
 expect(listed).toContain('webgpu_lights_rectarealight');
 expect(listed).toContain('webgl_loader_gltf_instancing');
 expect(listed).toContain('webgl_loader_gltf_avif');
 await page.goto('/web/gallery/example.html?id=webgl_loader_gltf');
 await expect(page.locator('#diagnostic')).toBeVisible();
 await page.locator('a[href="example.html?id=webgpu_loader_gltf"]').click();
 await expect(page.locator('body')).toHaveAttribute('data-backend','rust-wasm-webgpu',{timeout:90000});
});
for(const mode of [{name:'desktop-light',width:1280,height:900,colorScheme:'light'},{name:'mobile-dark',width:390,height:844,colorScheme:'dark'}]){
 test(`official gallery layout: ${mode.name}`,async({page},info)=>{
  await page.setViewportSize(mode);await page.emulateMedia({colorScheme:mode.colorScheme});
  await page.route('**/.cache/three-r186/examples/files.json',route=>route.fulfill({contentType:'application/json',body:files}));
  await page.route('**/.cache/three-r186/examples/screenshots/*.jpg',route=>route.fulfill({path:`web/gallery/screenshots/${route.request().url().split('/').at(-1)}`}));
  await page.route('**/.cache/three-r186/examples/files/thumbnails.svg',route=>route.fulfill({path:'web/gallery/files/thumbnails.svg'}));
  const ready=async()=>{await expect(page.locator('.card')).toHaveCount(catalog.examples.filter(e=>e.port&&e.status!=='excluded').length);await page.evaluate(()=>document.fonts.ready);await page.waitForFunction(()=>[...document.querySelectorAll('.card img')].filter(i=>i.getBoundingClientRect().top<innerHeight&&i.getBoundingClientRect().bottom>0).every(i=>i.complete&&i.naturalWidth));};
  await page.goto('/.cache/three-r186/examples/');await ready();const reference=await page.screenshot({animations:"disabled"});
  await page.goto('/web/gallery/');await ready();const actual=await page.screenshot({animations:"disabled"});
  const a=PNG.sync.read(actual),b=PNG.sync.read(reference);let different=0;const diff=new PNG({width:a.width,height:a.height});
  for(let i=0;i<a.data.length;i+=4){let d=0;for(let c=0;c<3;c++){d=Math.max(d,Math.abs(a.data[i+c]-b.data[i+c]));diff.data[i+c]=Math.min(255,Math.abs(a.data[i+c]-b.data[i+c])*8);}diff.data[i+3]=255;if(d>6)different++;}
  for(const [name,data] of [['actual',actual],['reference',reference],['difference',PNG.sync.write(diff)]])writeFileSync(info.outputPath(`${name}.png`),data);
  writeFileSync(info.outputPath('comparison.json'),JSON.stringify({different,fraction:different/(a.width*a.height)}));
  expect(different/(a.width*a.height)).toBeLessThanOrEqual(.005);
  await page.locator('#filterInput').fill('loader gltf');await expect(page.locator('.card:not(.hidden)')).not.toHaveCount(0);
  const visible=await page.locator('.card:not(.hidden) .title').allTextContents();expect(visible.every(s=>s.includes('loader')&&s.includes('gltf'))).toBe(true);
  await page.locator('#clearSearchButton').click();await expect(page.locator('.card.hidden')).toHaveCount(0);
  await page.locator('#previewsToggler').click();await expect(page.locator('#content')).toHaveClass('minimal');
 });
}
test('unported examples show source evidence rather than a fake reproduction',async({page})=>{
 const requests=[];page.on('request',r=>requests.push(r.url()));
 await page.goto('/web/gallery/');
 await expect(page.locator('.card')).toHaveCount(catalog.examples.filter(e=>e.port).length);
 await expect(page.locator('.card').filter({hasText:'compute / birds'})).toHaveCount(0);
 await page.goto('/web/gallery/example.html?id=webgpu_compute_birds');
 await expect(page.locator('#diagnostic')).toBeVisible();
 await expect(page.locator('#diagnostic')).toContainText('GPU compute');
 expect(requests.some(url=>url.includes('three.webgpu')||url.includes('three.module')||url.includes('three_rs_wasm'))).toBe(false);
 await page.goto('/web/gallery/example.html?id=unknown');await expect(page.locator('#diagnostic')).toContainText('Unknown example');
});
for(const entry of catalog.examples.filter(e=>e.port))test(`Rust gallery runtime: ${entry.id}`,async({page})=>{
 test.setTimeout(120000);const errors=[];const requests=[];page.on('pageerror',e=>errors.push(String(e)));page.on('request',r=>requests.push(r.url()));
 await page.goto(`/web/gallery/#${entry.id}`);const viewer=page.frameLocator('#viewer'),canvas=viewer.locator('canvas').first();
 // Audio needs a user gesture before it loads, as the original's start button does.
 if(entry.port.example===139)await viewer.getByRole('button',{name:'Play',exact:true}).click();
 await expect.poll(async()=>Number(await canvas.getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan([16,28,38,153,156,157,193,195,206,213,220,224,226,235,236,237,240,242].includes(entry.port.example) ? 0 : 2);
 await expect(viewer.locator('body')).toHaveAttribute('data-backend','rust-wasm-webgpu');await expect(canvas).not.toHaveAttribute('data-error',/.+/);
 if(entry.port.example===6)await expect(canvas).toHaveAttribute('data-meshes','30');
 const rendered=entry.port.example===93?viewer.locator('canvas').nth(1):canvas;
 const png=PNG.sync.read(await rendered.screenshot());expect(new Set(png.data).size).toBeGreaterThan(80);
 const dims=await rendered.boundingBox();const initial=await rendered.screenshot();
 if([3,4,5,6].includes(entry.port.example)){await page.mouse.move(dims.x+dims.width/2,dims.y+dims.height/2);await page.mouse.down();await page.mouse.move(dims.x+dims.width/2+80,dims.y+dims.height/2+35,{steps:5});await page.mouse.up();await page.waitForTimeout(100);expect((await canvas.screenshot()).equals(initial)).toBe(false);}
 await page.setViewportSize({width:1000,height:700});await expect.poll(async()=>Number(await canvas.getAttribute('width'))).toBe(700);
 expect(requests.some(url=>url.includes('three.webgpu')||url.includes('three.module'))).toBe(false);expect(errors).toEqual([]);
 if(entry.port.example===4)await viewer.locator('#model').selectOption('5');
 await page.evaluate(()=>location.hash=location.hash==='#webgl_geometry_cube'?'webgpu_pmrem_equirectangular':'webgl_geometry_cube');await expect.poll(async()=>Number(await page.frameLocator('#viewer').locator('canvas').first().getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan(2);expect(errors).toEqual([]);
});
test('PMREM sphere grid matches original physical materials and background node',async({page},info)=>{
 await page.setViewportSize({width:256,height:256});
 await page.goto('/reference/three-js/pmrem-grid.html');await expect(page.locator('canvas')).toHaveAttribute('data-ready','true',{timeout:90000});const reference=await page.locator('canvas').screenshot();
 await page.goto('/web/gallery/example.html?id=webgpu_pmrem_equirectangular');await expect.poll(async()=>Number(await page.locator('canvas').getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan(2);
 await page.addStyleTag({content:'#settings,#notice{display:none!important}'});const actual=await page.locator('canvas').screenshot();
 const a=PNG.sync.read(actual),b=PNG.sync.read(reference);let different=0;for(let i=0;i<a.data.length;i+=4)if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))different++;
 writeFileSync(info.outputPath('actual.png'),actual);writeFileSync(info.outputPath('reference.png'),reference);writeFileSync(info.outputPath('comparison.json'),JSON.stringify({different,fraction:different/(256*256)}));expect(different/(256*256)).toBeLessThanOrEqual(.005);
});
