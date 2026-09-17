import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
test('all 14 original RobotExpressive clips match Three.js world-space skin/morph samples',async({page},testInfo)=>{
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto('/reference/three-js/core-animation.html');
 await page.waitForFunction(()=>window.samples);
 expect(errors).toEqual([]);
 const reference=await page.evaluate(()=>window.samples);
 const rust=JSON.parse(readFileSync('.cache/core-animation-samples.json'));
 expect(rust.length).toBe(reference.length);
 let maximum=0,samples=0;
 for(const frame of rust){
  const original=reference.find(f=>f.clip===frame.clip&&f.time===frame.time);expect(original).toBeTruthy();
  const available=[...original.meshes];
  for(const mesh of frame.meshes){
   const candidates=available.filter(m=>m.name===mesh.name&&m.vertices===mesh.vertices);
   expect(candidates.length,`${frame.clip}: ${mesh.name}/${mesh.vertices}`).toBeGreaterThan(0);
   const error=other=>Math.max(...mesh.points.flatMap((p,i)=>p.map((v,c)=>Math.abs(v-other.points[i][c]))));
   candidates.sort((a,b)=>error(a)-error(b));const match=candidates[0];maximum=Math.max(maximum,error(match));samples+=mesh.points.length;
   expect(error(match),`${frame.clip} t=${frame.time}: ${mesh.name}`).toBeLessThan(0.0001);
   available.splice(available.indexOf(match),1);
  }
  expect(available).toHaveLength(0);
 }
 await testInfo.attach('animation-comparison',{body:JSON.stringify({maximum,samples,clips:14}),contentType:'application/json'});
});
