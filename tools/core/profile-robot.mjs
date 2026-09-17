// Browser-side frame/CPU/API diagnostics; no resolution or animation compromises.
import {chromium} from '@playwright/test';
import {writeFile} from 'node:fs/promises';
import {timestamps} from './gpu-timestamps.js';
const browser = await chromium.launch({headless:!process.env.HEADED, executablePath:process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined,args:['--enable-unsafe-webgpu']});
try {
 const page=await browser.newPage({viewport:{width:1280,height:800},deviceScaleFactor:Number(process.env.DPR||2)});
 if(process.env.GPU)await page.addInitScript(timestamps);
 await page.addInitScript(()=>{
  window.measure={frames:[],cpu:[],calls:{}};
  const raf=window.requestAnimationFrame.bind(window);
  window.requestAnimationFrame=cb=>raf(t=>{const start=performance.now();cb(t);window.measure.cpu.push(performance.now()-start);window.measure.frames.push(t);});
  for(const [proto,names] of [[GPUDevice.prototype,['createBuffer','createBindGroup','createRenderPipeline','createTexture']],[GPUQueue.prototype,['writeBuffer','submit']]]) for(const name of names){const original=proto[name];proto[name]=function(...args){window.measure.calls[name]=(window.measure.calls[name]||0)+1;return original.apply(this,args);};}
 });
 page.on('pageerror',e=>console.error(String(e)));
 await page.goto(process.env.URL||'http://127.0.0.1:8173/web/gallery/example.html?id=webgl_animation_skinning_morph');
 await page.waitForFunction(()=>document.body.dataset.status==='running');
 await page.waitForTimeout(5000);
 const client=await page.context().newCDPSession(page);
 await client.send('Profiler.enable');await client.send('Profiler.start');
 await page.evaluate(()=>{window.measure={frames:[],cpu:[],calls:{}};window.sampleGPU=true;});
 await page.waitForTimeout(5000);
 const result=await page.evaluate(async()=>{
  const m=window.measure;const stats=a=>{a.sort((a,b)=>a-b);return {p50:a[Math.floor(a.length*.5)],p95:a[Math.floor(a.length*.95)],max:a.at(-1)};};
  const adapter=await navigator.gpu.requestAdapter();
  const a=adapter.info;
  const gpu=window.gpuTimings;
  return {browser:navigator.userAgent,adapter:{vendor:a.vendor,architecture:a.architecture,device:a.device,description:a.description},canvas:[document.querySelector('canvas').width,document.querySelector('canvas').height],frames:m.frames.length,interval:stats(m.frames.slice(1).map((v,i)=>v-m.frames[i])),cpu:stats(m.cpu),calls:m.calls,transfers:window.app?.transfer_counts(),gpu:gpu?{supported:gpu.supported,passes:Object.fromEntries([...new Set(gpu.passes.map(p=>p.label))].map(label=>[label,stats(gpu.passes.filter(p=>p.label===label).map(p=>p.ms))]))}:undefined};
 });
 const {profile}=await client.send('Profiler.stop');
 const counts=new Map();for(const id of profile.samples||[]) counts.set(id,(counts.get(id)||0)+1);
 result.hot=profile.nodes.map(n=>({name:n.callFrame.functionName,url:n.callFrame.url,count:counts.get(n.id)||0})).sort((a,b)=>b.count-a.count).slice(0,18);
 console.log(JSON.stringify(result,null,2));
 if(process.env.PROFILE) await writeFile(process.env.PROFILE,JSON.stringify(profile));
} finally {await browser.close();}
