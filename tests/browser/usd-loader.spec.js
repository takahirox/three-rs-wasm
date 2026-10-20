import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
import {fixtures} from '../../tools/core/usd-fixtures.mjs';
// The loader port is in webgl_loader_usdz's module ( tools/gallery/modules.py ).
const PACKAGE=JSON.parse(readFileSync(new URL('../../web/gallery/modules.json',import.meta.url),'utf8')).examples.webgl_loader_usdz;
// The Rust USDLoader against the pinned r186 USDLoader in the same engine:
// webgl_loader_usdz's saeukkang.usdz and the fixtures of
// tools/core/usd-fixtures.mjs, in the canonical form of usd::canonical
// ( numbers as their bits, buffers by hash ).
test('USDLoader port matches the original bit for bit',async({page})=>{
 test.setTimeout(180000);
 await page.goto('/reference/three-js/usd-loader.html');await expect(page).toHaveTitle('ready');
 const inputs=fixtures().map(f=>({name:f.name,path:f.path,bytes:Array.from(f.bytes)}));
 const results=await page.evaluate(async ([inputs,pkg])=>{
  const {loadRuntime}=await import('/web/gallery/runtime.js');const rust=await loadRuntime(pkg);
  inputs.push({name:'saeukkang.usdz',path:'',bytes:new Uint8Array(await (await fetch('/.cache/three-r186/examples/models/usdz/saeukkang.usdz')).arrayBuffer())});
  const results=[];
  for(const input of inputs){
   const bytes=new Uint8Array(input.bytes);
   const expected=await window.usdCanonical(bytes,input.path);
   const actual=rust.usd_canonical(bytes,input.path);
   let at=-1;for(let i=0;i<Math.max(expected.length,actual.length);i++)if(expected[i]!==actual[i]){at=i;break;}
   results.push({name:input.name,equal:expected===actual,length:expected.length,context:at<0?null:{expected:expected.slice(Math.max(0,at-120),at+80),actual:actual.slice(Math.max(0,at-120),at+80)}});
  }
  return results;
 },[inputs,PACKAGE]);
 for(const r of results)expect(r,JSON.stringify(r)).toMatchObject({equal:true});
});
