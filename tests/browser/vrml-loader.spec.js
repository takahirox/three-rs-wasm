import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
// The loader port is in webgl_loader_vrml's module ( tools/gallery/modules.py ).
const PACKAGE=JSON.parse(readFileSync(new URL('../../web/examples/modules.json',import.meta.url),'utf8')).examples.webgl_loader_vrml;
// The Rust VRMLLoader against the pinned r186 VRMLLoader in the same engine:
// every VRML asset webgl_loader_vrml loads, in the canonical form of
// vrml::canonical ( numbers as their bits, buffers by hash ).
const ASSETS=['creaseAngle','crystal','house','elevationGrid1','elevationGrid2','extrusion1','extrusion2','extrusion3','lines','linesTransparent','meshWithLines','meshWithTexture','pixelTexture','points','camera','multilineString'];
test('VRMLLoader port matches the original bit for bit',async({page})=>{
 test.setTimeout(120000);
 await page.goto('/reference/three-js/vrml-loader.html');await expect(page).toHaveTitle('ready');
 const results=await page.evaluate(async ([assets,pkg])=>{
  const {loadRuntime}=await import('/web/examples/runtime.js');const rust=await loadRuntime(pkg);
  const results=[];
  for(const asset of assets){
   const text=await (await fetch(`/.cache/three-r187/examples/models/vrml/${asset}.wrl`)).text();
   const expected=window.vrmlCanonical(text,'models/vrml/');
   const actual=rust.vrml_canonical(text,'models/vrml/',innerWidth,innerHeight);
   let at=-1;for(let i=0;i<Math.max(expected.length,actual.length);i++)if(expected[i]!==actual[i]){at=i;break;}
   results.push({asset,equal:expected===actual,length:expected.length,context:at<0?null:{expected:expected.slice(Math.max(0,at-120),at+80),actual:actual.slice(Math.max(0,at-120),at+80)}});
  }
  return results;
 },[ASSETS,PACKAGE]);
 for(const r of results)expect(r,JSON.stringify(r)).toMatchObject({equal:true});
});
