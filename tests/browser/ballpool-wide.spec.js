import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';

// webgpu_postprocessing_ssgi_ballpool at aspect 4.2: 1,083 balls exceed InstanceNode's
// uniform-array limit ( 1,024 matrices ), so both runtimes draw them from instanced
// vertex attributes. After 40 frames of falling the outputs must match
// ( docs/ballpool.md ).
test('ballpool instanced-attribute path matches the original',async({page})=>{
 test.setTimeout(300000);const shots={};const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:2100,height:500});
  await page.goto(runtime==='rust'?'/web/gallery/example.html?id=webgpu_postprocessing_ssgi_ballpool&still=1':'/reference/three-js/compute-examples.html?id=webgpu_postprocessing_ssgi_ballpool&samples=1');
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c?.dataset.ready==='true'||Number(c?.dataset.frames)>0;},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  for(let k=1;k<=40;k++)await page.evaluate(async({runtime,t})=>{const c=document.querySelector('canvas'),p=c.dataset.frames;if(runtime!=='rust')await renderFixture(t);else{app.gallery_time(t);while(c.dataset.frames===p)await new Promise(r=>requestAnimationFrame(r));}},{runtime,t:k/60});
  shots[runtime]=PNG.sync.read(await page.locator('canvas').screenshot());
 }
 const a=shots.rust,b=shots.reference;expect([a.width,a.height]).toEqual([b.width,b.height]);
 let bad=0,sum=0;for(let p=0;p<a.data.length;p+=4){let fail=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[p+c]-b.data[p+c]);sum+=d;fail||=d>6;}if(fail)bad++;}
 expect(errors).toEqual([]);
 expect(bad/(a.width*a.height)).toBeLessThanOrEqual(.005);
 expect(sum/(a.width*a.height*3)).toBeLessThanOrEqual(.6);
});
