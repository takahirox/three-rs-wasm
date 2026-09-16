import{test,expect}from'@playwright/test';
import{writeFileSync,readFileSync}from'node:fs';
import{PNG}from'pngjs';
const manifest=JSON.parse(readFileSync(new URL('../../tests/gltf-pbr/manifest.json',import.meta.url)));
test('M2 reference is repeatable with original PBR materials and HDR',async({page},info)=>{
 test.setTimeout(240000);await page.goto('/reference/three-js/gltf-pbr.html');
 const canvas=page.locator('canvas');await expect(canvas).toHaveAttribute('data-ready','true',{timeout:120000});
 const results=[];
 for(let m=0;m<2;m++)for(let v=0;v<manifest.views.length;v++){
  await page.evaluate(([m,v])=>window.renderFixture(m,v),[m,v]);const a=await canvas.screenshot();
  await page.evaluate(([m,v])=>window.renderFixture(m,v),[m,v]);const b=await canvas.screenshot();
  const p=PNG.sync.read(a),q=PNG.sync.read(b);let maximum=0;for(let i=0;i<p.data.length;i++)maximum=Math.max(maximum,Math.abs(p.data[i]-q.data[i]));
  results.push({model:manifest.models[m].name,view:manifest.views[v].name,maximum});expect(maximum).toBe(0);
  writeFileSync(info.outputPath(`${m}-${v}-reference.png`),a);
 }
 writeFileSync(info.outputPath('repeatability.json'),JSON.stringify(results,null,2));
});
