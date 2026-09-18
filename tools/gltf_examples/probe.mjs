// Real unpublished Rust scenes versus pinned original model/renderer workloads.
import {chromium} from '@playwright/test';
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {PNG} from 'pngjs';
const cases=[
 {id:'webgpu_loader_gltf_iridescence',example:29},
 {id:'webgpu_loader_gltf_anisotropy',example:30},
 {id:'webgpu_loader_gltf_sheen',example:31},
 {id:'webgpu_loader_gltf_transmission',example:32},
 {id:'webgl_loader_gltf_instancing',example:16},
];
const browser=await chromium.launch({executablePath:process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined,args:['--enable-unsafe-webgpu']});
const results=[];
try {for(const entry of cases){
 const result={...entry,states:[],errors:[]},images={};
 for(const runtime of ['reference','rust']){
  const page=await browser.newPage({viewport:{width:512,height:512},deviceScaleFactor:1});
  page.on('pageerror',e=>result.errors.push(`${runtime}: ${e}`));
  await page.addInitScript(()=>{
   window.costs={creates:0,writes:0,bytes:0,textures:[]};
   for(const name of ['createBuffer','createBindGroup','createTexture']){const original=GPUDevice.prototype[name];GPUDevice.prototype[name]=function(...args){costs.creates++;if(name==='createTexture')costs.textures.push(args[0]);return original.apply(this,args)}}
   const original=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(...a){costs.writes++;costs.bytes+=a[2].byteLength;return original.apply(this,a)};
  });
  try {
   if(runtime==='rust'){
    const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));Object.assign(catalog.examples.find(e=>e.id===entry.id),{status:'partial',port:{example:entry.example,limitations:[]}});
    await page.route('**/web/gallery/catalog.json',r=>r.fulfill({json:catalog}));
   }
   const ref=entry.example===16?'expanded':'gltf-examples';
   await page.goto('http://127.0.0.1:8173'+(runtime==='reference'?`/reference/three-js/${ref}.html?id=${entry.id}`:`/web/gallery/example.html?id=${entry.id}&still=1`));
   await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true'||Number(document.querySelector('canvas')?.dataset.frames)>2||document.querySelector('canvas')?.dataset.error||document.body.dataset.error||document.body.dataset.status==='error',null,{timeout:30000});
   const error=await page.evaluate(()=>document.querySelector('canvas')?.dataset.error||document.body.dataset.error||document.body.dataset.status==='error'&&document.body.innerText);
   if(error)throw new Error(error);
   if(runtime==='rust')await page.addStyleTag({content:'#notice,#settings{display:none!important}'});
   await page.waitForTimeout(150);
   const folder=`.cache/gltf-examples/results/${entry.id}`;mkdirSync(folder,{recursive:true});
   images[runtime]=PNG.sync.read(await page.locator('canvas').screenshot({path:`${folder}/${runtime}.png`}));
   result[runtime]=await page.evaluate(()=>({costs:{...costs},transfer:window.app?JSON.parse(app.transfer_counts()):null,resources:window.app?Array.from(app.resource_counts()):null}));
   if(runtime==='rust'){
    await page.waitForTimeout(250);
    result.steady=await page.evaluate(()=>({creates:costs.creates,writes:costs.writes,transfer:JSON.parse(app.transfer_counts()),resources:Array.from(app.resource_counts())}));
   }
  }catch(e){result.errors.push(`${runtime}: ${String(e).slice(0,400)}`);}
  finally{await page.close();}
 }
 if(images.rust&&images.reference){
  const a=images.rust,b=images.reference;let raw=0,area=0;
  for(let y=0;y<a.height;y++)for(let x=0;x<a.width;x++){
   const i=(y*a.width+x)*4;
   if([0,1,2].some(c=>Math.abs(a.data[i+c]-b.data[i+c])>6))raw++;
   if([0,1,2].some(c=>{let d=0,n=0;for(let dy=-1;dy<=1;dy++)for(let dx=-1;dx<=1;dx++){const xx=x+dx,yy=y+dy;if(xx<0||yy<0||xx>=a.width||yy>=a.height)continue;const j=(yy*a.width+xx)*4+c;d+=a.data[j]-b.data[j];n++;}return Math.abs(d/n)>6;}))area++;
  }
  result.comparison={rawFraction:raw/(a.width*a.height),areaFraction:area/(a.width*a.height),channelTolerance:6,maximumDifferentFraction:.005};
 }
 results.push(result);writeFileSync('docs/gltf-examples-render-attempts.json',JSON.stringify(results,null,2)+'\n');
 console.log(entry.id,JSON.stringify({comparison:result.comparison,errors:result.errors}));
}}finally{await browser.close();}
