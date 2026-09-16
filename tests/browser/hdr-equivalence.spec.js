import {test,expect} from '@playwright/test';
import {readFileSync,writeFileSync} from 'node:fs';
const provenance=JSON.parse(readFileSync(new URL('../../web/environments.json',import.meta.url)));
test('Rust HDR decoding preserves official UltraHDR reconstruction within RGBE precision',async({page},info)=>{
 await page.goto('/reference/three-js/gltf-pbr.html');await expect(page.locator('canvas')).toHaveAttribute('data-ready','true',{timeout:120000});
 const result=await page.evaluate(async()=>{
  const {DataUtils}=await import('/.cache/three-r186/src/Three.WebGPU.js');const a=window.reference.environment.image.data;
  const response=await fetch('/.cache/gltf-pbr-decoded-hdr.bin');if(!response.ok)throw new Error('Run cargo test --test pbr to produce Rust-decoded HDR evidence');const b=new Uint16Array(await response.arrayBuffer());
  if(a.length!==b.length)throw new Error('HDR dimensions differ');let maximum=0,hdrPixels=0;
  for(let i=0;i<a.length;i+=4){const source=[0,1,2].map(c=>DataUtils.fromHalfFloat(a[i+c]));const peak=Math.max(...source);if(peak>1)hdrPixels++;
   for(let c=0;c<3;c++)maximum=Math.max(maximum,Math.abs(source[c]-DataUtils.fromHalfFloat(b[i+c]))/Math.max(peak,1e-32));
   if(b[i+3]!==15360)throw new Error('Invalid HDR alpha');
  }return {maximum,hdrPixels,pixels:a.length/4};
 });
 writeFileSync(info.outputPath('hdr-equivalence.json'),JSON.stringify(result,null,2));expect(result.maximum).toBeLessThanOrEqual(provenance.error_bound);expect(result.hdrPixels).toBeGreaterThan(1000);
});
