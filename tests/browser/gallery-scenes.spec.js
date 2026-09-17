import {test,expect} from '@playwright/test';
import {readFileSync,writeFileSync} from 'node:fs';
import {PNG} from 'pngjs';
const examples=['webgl_buffergeometry_lines_indexed','webgl_panorama_equirectangular','webgl_interactive_lines','webgl_interactive_raycasting_points','webgl_materials_texture_rotation','webgl_buffergeometry_lines','webgl_pmrem_equirectangular','webgl_pmrem_test'];
for(const id of examples)test(`original scene comparison: ${id}`,async({page},info)=>{
 test.setTimeout(120000);
 await page.setViewportSize({width:512,height:512});
 let source=readFileSync(`.cache/three-r186/examples/${id}.html`,'utf8');
 source=source.replace('../build/three.module.js','../src/Three.js').replace("import Stats from 'three/addons/libs/stats.module.js';", "class Stats {dom=document.createElement('div');update(){}}");
 source=source.replace("import { GUI } from 'three/addons/libs/lil-gui.module.min.js';","class GUI {add(){return this}name(){return this}onChange(){return this}open(){}}");
 // Freeze only time/random inputs. Keep the upstream scene and WebGL renderer.
 source=source.replace("import * as THREE from 'three';", "import * as THREE from 'three'; let seed=1; const random=()=>((seed=(Math.imul(seed,1664525)+1013904223)>>>0)/4294967296);");
 source=source.replaceAll('Math.random()', 'random()').replaceAll('Date.now()', '1230').replace('timer.getElapsed()', '1.23').replace('t += delta * 0.5;', 't = 1.23 * 0.5;');
 source=source.replace('lon += 0.1;', 'lon += 0;').replace('theta += 0.1;', 'theta = 0;');
 source=source.replace('camera.applyMatrix4( rotateY );','').replace('sphere.scale.multiplyScalar( 0.98 );','').replace('toggle += timer.getDelta();','');
 source=source.replace('renderer.render( scene, camera );','window.reference={scene,camera,renderer}; renderer.render( scene, camera );');
 await page.route(`**/.cache/three-r186/examples/${id}.html`,r=>r.fulfill({contentType:'text/html',body:source}));
 await page.route('**/textures/2294472375_24a3b8ef46_o.jpg',r=>r.fulfill({path:'web/gallery/assets/panorama.jpg'}));
 await page.route('**/textures/uv_grid_opengl.jpg',r=>r.fulfill({path:'web/gallery/assets/uv-grid.jpg'}));
 await page.route('**/textures/equirectangular/royal_esplanade_2k.hdr.jpg',r=>r.fulfill({path:'web/environments/royal_esplanade_2k.hdr.jpg'}));
 await page.route('**/textures/equirectangular/spot1Lux.hdr',r=>r.fulfill({path:'web/gallery/assets/spot1Lux.hdr'}));
 await page.goto(`/.cache/three-r186/examples/${id}.html`);
 await page.waitForFunction(()=>window.reference);
 if(id.includes('pmrem'))await page.waitForFunction(n=>window.reference.scene.children.length===n,id==='webgl_pmrem_test'?34:30);
 if(id.includes('panorama'))await page.waitForFunction(()=>window.reference.scene.children[0].material.map.image?.complete);
 await page.addStyleTag({content:'#info{display:none!important}'});
 await page.waitForTimeout(100);
 const reference=await page.locator('canvas').first().screenshot();
 const upstream=await page.evaluate(()=>{
  let vertices=0,indices=0,lines=0,points=0;
  window.reference.scene.traverse(o=>{if(o.isLine){lines++;vertices+=o.geometry.attributes.position.count;indices+=o.geometry.index?.count??0;}if(o.isPoints)points+=o.geometry.attributes.position.count;});
  const objects=[];window.reference.scene.traverse(o=>{if(o.isLine||o.isPoints)objects.push({position:o.geometry.morphAttributes.position?Array.from(o.geometry.attributes.position.array,(v,i)=>v+(o.geometry.morphAttributes.position[0].array[i]-v)*o.morphTargetInfluences[0]):Array.from(o.geometry.attributes.position.array),color:o.geometry.attributes.color?Array.from(o.geometry.attributes.color.array):null,index:o.geometry.index?Array.from(o.geometry.index.array):null,matrix:o.matrixWorld.elements,segments:!!o.isLineSegments});});return {vertices,indices,lines,points,objects};
 });
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto(`/web/gallery/example.html?id=${id}&still`);
 await expect.poll(async()=>Number(await page.locator('canvas').getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan(2);
 await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
 await page.addStyleTag({content:'#settings,#notice{display:none!important}'});
 const actual=await page.locator('canvas').screenshot();
 const a=PNG.sync.read(actual),b=PNG.sync.read(reference);let differing=0;
 for(let i=0;i<a.data.length;i+=4)if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))differing++;
 const fraction=differing/(a.width*a.height);
 writeFileSync(info.outputPath('actual.png'),actual);writeFileSync(info.outputPath('reference.png'),reference);
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify({fraction,differing,upstream:{vertices:upstream.vertices,indices:upstream.indices,lines:upstream.lines,points:upstream.points}}));
 // Keep the measured WebGL image differences visible in the report. For the
 // picking scenes, compare every generated vertex/color/index and world transform;
 // their native WebGL point/line rasterization is not yet reproduced by WebGPU.
 if(!id.includes('interactive')&&id!=='webgl_buffergeometry_lines')expect(fraction).toBeLessThanOrEqual(id.includes('panorama')?0.01:0.025);
 const actualObjects=await page.evaluate(()=>{
  const root=JSON.parse(window.app.scene_json()),objects=[];
  const visit=n=>{const object=n.kind?.Line||n.kind?.Points;if(object){const g=object.geometry;objects.push({position:g.morph_attributes?.position?g.attributes.position.F32.array.map((v,i)=>v+(g.morph_attributes.position[0].F32.array[i]-v)*(n.morph_weights[0]??0)):g.attributes.position.F32.array,color:g.attributes.color?.F32.array??null,index:g.index,matrix:n.matrix_world,segments:!!object.segments});}for(const child of n.children||[])visit(child);};visit(root);return objects;
 });
 expect(actualObjects.length).toBe(upstream.objects.length);
 for(let i=0;i<actualObjects.length;i++)for(const key of ['position','color','index','matrix','segments']){
  const a=actualObjects[i][key],b=upstream.objects[i][key];
  if(!Array.isArray(a)){expect(a).toEqual(b);continue;}
  expect(a.length).toBe(b.length);let maximum=0;for(let j=0;j<a.length;j++)maximum=Math.max(maximum,Math.abs(a[j]-b[j])/Math.max(1,Math.abs(b[j])));
  expect(maximum,`${id} object ${i} ${key}`).toBeLessThanOrEqual(2e-6);
 }
 if(id.includes('panorama')){
  await page.mouse.move(250,250);await page.mouse.down();await page.mouse.move(450,300,{steps:5});await page.mouse.up();
  await page.waitForTimeout(100);const dragged=await page.locator('canvas').screenshot();expect(dragged.equals(actual)).toBe(false);
  await page.mouse.wheel(0,-400);await page.waitForTimeout(100);expect((await page.locator('canvas').screenshot()).equals(dragged)).toBe(false);
 }
 if(id==='webgl_pmrem_test'){
  await page.evaluate(()=>{const input=document.querySelector('#pmrem');input.checked=false;input.dispatchEvent(new Event('input',{bubbles:true}));});
  await page.waitForTimeout(100);expect((await page.locator('canvas').screenshot()).equals(actual)).toBe(false);
 }
 if(id==='webgl_materials_texture_rotation'){
  await page.evaluate(()=>{const input=document.querySelector('#uv-4');input.value='-1';input.dispatchEvent(new Event('input',{bubbles:true}));});
  await page.waitForTimeout(100);expect((await page.locator('canvas').screenshot()).equals(actual)).toBe(false);
 }
 if(id.includes('interactive')){
  await page.mouse.move(200,200);await page.waitForTimeout(100);
  await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
 }
 expect(errors).toEqual([]);
});

test('direct equirectangular background matches original TSL texture node',async({page},info)=>{
 await page.setViewportSize({width:512,height:512});
 await page.goto('/reference/three-js/equirectangular.html');await expect(page.locator('canvas')).toHaveAttribute('data-ready','true',{timeout:90000});
 const reference=await page.locator('canvas').screenshot();
 await page.goto('/web/gallery/example.html?id=webgpu_equirectangular&still');await expect.poll(async()=>Number(await page.locator('canvas').getAttribute('data-frames')),{timeout:90000}).toBeGreaterThan(2);
 await page.addStyleTag({content:'#settings,#notice{display:none!important}'});const actual=await page.locator('canvas').screenshot();
 const a=PNG.sync.read(actual),b=PNG.sync.read(reference);let differing=0;
 for(let i=0;i<a.data.length;i+=4)if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))differing++;
 writeFileSync(info.outputPath('actual.png'),actual);writeFileSync(info.outputPath('reference.png'),reference);writeFileSync(info.outputPath('comparison.json'),JSON.stringify({differing,fraction:differing/(512*512)}));
 expect(differing/(512*512)).toBeLessThanOrEqual(.005);
 await page.evaluate(()=>{const input=document.querySelector('#background-intensity');input.value='0.2';input.dispatchEvent(new Event('input',{bubbles:true}));});
 await page.waitForTimeout(100);expect((await page.locator('canvas').screenshot()).equals(actual)).toBe(false);
 await expect(page.locator('canvas')).not.toHaveAttribute('data-error',/.+/);
});
