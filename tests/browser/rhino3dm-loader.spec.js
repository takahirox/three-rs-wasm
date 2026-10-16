import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
// The Rust Rhino3dmLoader port against the pinned r186 Rhino3dmLoader in the
// same engine, both on rhino3dm 8.32.1 from jsDelivr: webgl_loader_3dm's
// Rhino_Logo.3dm, in the canonical form of rhino3dm_loader.rs ( numbers as
// their bits, buffers by hash ).
const packages=JSON.parse(readFileSync(new URL('../../web/gallery/packages.json',import.meta.url),'utf8'));
test('Rhino3dmLoader port matches the original bit for bit',async({page})=>{
 test.setTimeout(240000);
 await page.goto('/reference/three-js/rhino3dm-loader.html');await expect(page).toHaveTitle('ready');
 const result=await page.evaluate(async name=>{
  const rust=await import(`/web/pkg/${name}/three_rs_wasm.js`);await rust.default({module_or_path:`/web/pkg/${name}/three_rs_wasm_bg.wasm`});
  const bytes=new Uint8Array(await (await fetch('/.cache/three-r186/examples/models/3dm/Rhino_Logo.3dm')).arrayBuffer());
  const expected=await window.rhino3dmCanonical(bytes);
  const actual=await rust.rhino3dm_canonical(bytes);
  let at=-1;for(let i=0;i<Math.max(expected.length,actual.length);i++)if(expected[i]!==actual[i]){at=i;break;}
  return {equal:expected===actual,length:expected.length,context:at<0?null:{expected:expected.slice(Math.max(0,at-160),at+120),actual:actual.slice(Math.max(0,at-160),at+120)}};
 },packages.examples.webgl_loader_3dm);
 expect(result,JSON.stringify(result)).toMatchObject({equal:true});
});
