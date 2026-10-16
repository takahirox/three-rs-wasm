import {test,expect} from '@playwright/test';
import {writeFileSync} from 'node:fs';
// The Rust port decodes the UltraHDR JPEG at run time, as UltraHDRLoader does.
test('Rust UltraHDR decoding matches the official UltraHDRLoader reconstruction',async({page},info)=>{
 await page.goto('/reference/three-js/gltf-pbr.html');await expect(page.locator('canvas')).toHaveAttribute('data-ready','true',{timeout:120000});
 const result=await page.evaluate(async()=>{
  const {DataUtils}=await import('/.cache/three-r186/src/Three.WebGPU.js');const a=window.reference.environment.image.data;
  const rust=await import('/web/pkg/core/three_rs_wasm.js');await rust.default({module_or_path:'/web/pkg/core/three_rs_wasm_bg.wasm'});
  const bytes=new Uint8Array(await (await fetch('/web/environments/royal_esplanade_2k.hdr.jpg')).arrayBuffer());
  const b=await rust.decode_environment(bytes);
  if(a.length!==b.length)throw new Error('HDR dimensions differ');let maximum=0,hdrPixels=0;
  for(let i=0;i<a.length;i+=4){const source=[0,1,2].map(c=>DataUtils.fromHalfFloat(a[i+c]));const peak=Math.max(...source);if(peak>1)hdrPixels++;
   for(let c=0;c<3;c++)maximum=Math.max(maximum,Math.abs(source[c]-DataUtils.fromHalfFloat(b[i+c]))/Math.max(peak,1e-32));
   if(b[i+3]!==a[i+3])throw new Error('Invalid HDR alpha');
  }return {maximum,hdrPixels,pixels:a.length/4};
 });
 writeFileSync(info.outputPath('hdr-equivalence.json'),JSON.stringify(result,null,2));console.log(JSON.stringify(result));
 expect(result.maximum).toBe(0);expect(result.hdrPixels).toBeGreaterThan(1000);
});
