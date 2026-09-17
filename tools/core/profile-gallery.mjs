// Sequential real-browser audit of every runnable gallery entry.
import {chromium} from '@playwright/test';
import {readFile,writeFile} from 'node:fs/promises';
import {timestamps} from './gpu-timestamps.js';
const catalog=JSON.parse(await readFile(new URL('../../web/gallery/catalog.json',import.meta.url)));
const entries=catalog.examples.filter(e=>e.port).map(e=>({id:e.id,url:`/web/gallery/example.html?id=${e.id}`}));
entries.push({id:'BoomBox',url:'/web/?example=5'},{id:'mvp_perspective',url:'/web/?example=0'},{id:'mvp_orthographic',url:'/web/?example=1'});
const selected=process.env.ONLY?entries.filter(e=>process.env.ONLY.split(',').includes(e.id)):entries;
const browser=await chromium.launch({headless:true,executablePath:process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined,args:['--enable-unsafe-webgpu']});
const report={date:new Date().toISOString(),mode:process.env.GPU?'GPU timestamps (includes diagnostic allocations)':'CPU/API',warmupMs:3000,sampleMs:3000,results:[]};
try {
 for(const entry of selected){
  const page=await browser.newPage({viewport:{width:1280,height:800},deviceScaleFactor:2});
  const errors=[];page.on('pageerror',e=>errors.push(String(e)));
  if(process.env.GPU)await page.addInitScript(timestamps);
  await page.addInitScript(()=>{
   const fresh=()=>({frames:[],cpu:[],calls:{},writeBytes:0});window.measure=fresh();
   const raf=requestAnimationFrame.bind(window);
   window.requestAnimationFrame=cb=>raf(t=>{const start=performance.now();cb(t);window.measure.cpu.push(performance.now()-start);window.measure.frames.push(t);});
   for(const [proto,names] of [[GPUDevice.prototype,['createBuffer','createBindGroup','createRenderPipeline','createTexture']],[GPUQueue.prototype,['writeBuffer','writeTexture','submit']]])for(const name of names){const original=proto[name];proto[name]=function(...args){const m=window.measure;m.calls[name]=(m.calls[name]||0)+1;if(name==='writeBuffer'){const data=args[2],unit=ArrayBuffer.isView(data)?(data.BYTES_PER_ELEMENT||1):1;m.writeBytes+=args[4]===undefined?data.byteLength-(args[3]||0)*unit:args[4]*unit;}return original.apply(this,args);};}
  });
  try{
   await page.goto('http://127.0.0.1:8173'+entry.url);
   await page.waitForFunction(()=>window.app&&document.querySelector('canvas')?.dataset.frames>5,{},{timeout:90000}).catch(async()=>{await page.waitForFunction(()=>window.app&&document.body.dataset.status==='running',{}, {timeout:1000});});
   await page.waitForTimeout(report.warmupMs);
   await page.evaluate(()=>{window.measure={frames:[],cpu:[],calls:{},writeBytes:0};window.sampleGPU=true;window.transferStart=JSON.parse(window.app.transfer_counts());window.resourceStart=Array.from(window.app.resource_counts());});
   await page.waitForTimeout(report.sampleMs);
   const result=await page.evaluate(async()=>{
    const m=window.measure,stats=a=>{a.sort((a,b)=>a-b);return {p50:a[Math.floor(a.length*.5)],p95:a[Math.floor(a.length*.95)],max:a.at(-1)};};
    const adapter=await navigator.gpu.requestAdapter(),a=adapter.info,gpu=window.gpuTimings;
    return {browser:navigator.userAgent,adapter:{vendor:a.vendor,architecture:a.architecture},canvas:[document.querySelector('canvas').width,document.querySelector('canvas').height],frames:m.frames.length,canvasError:document.querySelector('canvas').dataset.error,interval:stats(m.frames.slice(1).map((v,i)=>v-m.frames[i])),cpu:stats(m.cpu),calls:m.calls,writeBytes:m.writeBytes,transferDelta:JSON.parse(window.app.transfer_counts()).map((v,i)=>v-window.transferStart[i]),resourceStart:window.resourceStart,resourceEnd:Array.from(window.app.resource_counts()),gpu:gpu?Object.fromEntries([...new Set(gpu.passes.map(p=>p.label))].map(label=>[label,stats(gpu.passes.filter(p=>p.label===label).map(p=>p.ms))])):undefined};
   });
   if(result.frames<2||result.canvasError)throw new Error(result.canvasError||'No animation frames during measurement');
   report.results.push({id:entry.id,...result,errors});
   console.log(entry.id,JSON.stringify({cpu:result.cpu,calls:result.calls,gpu:result.gpu,errors}));
  }catch(e){report.results.push({id:entry.id,error:String(e),errors});console.log(entry.id,String(e));}
  finally{await page.close();}
  if(process.env.OUT)await writeFile(process.env.OUT,JSON.stringify(report,null,2)+'\n');
 }
}finally{await browser.close();}
if(report.results.some(r=>r.error||r.errors.length))process.exitCode=1;
