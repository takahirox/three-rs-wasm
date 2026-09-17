// Same scene, assets, animation and viewport; logical allocations exclude driver overhead.
import {chromium} from '@playwright/test';
import {readFileSync,writeFileSync} from 'node:fs';
import {timestamps} from './gpu-timestamps.js';
const id=process.env.EXAMPLE||'webgl_geometries';
const ports={webgl_morphtargets_horse:24,webgl_morphtargets_sphere:25,webgl_buffergeometry_indexed:22,webgl_lines_colors:23,webgl_geometries:17,webgl_loader_gltf_instancing:16,webgl_morphtargets:18,webgpu_morphtargets:18,webgl_lines_dashed:19,webgl_lights_rectarealight:20,webgl_geometry_colors:21};
if(!(id in ports))throw new Error('Unknown expanded port');
const browser=await chromium.launch({executablePath:process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined,args:['--enable-unsafe-webgpu']});
const report={id,date:new Date().toISOString(),warmupMs:3000,sampleMs:3000,gpuInstrumentation:!!process.env.GPU,results:[]};
try {for(const runtime of ['three','rust']){
 const backend=id==='webgl_morphtargets_sphere'&&runtime==='three'?'WebGL2':'WebGPU';
 const page=await browser.newPage({viewport:{width:512,height:512},deviceScaleFactor:1});const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 if(process.env.GPU)await page.addInitScript(timestamps);
 await page.addInitScript(()=>{
  const fresh=()=>({cpu:[],frames:[],calls:{},writeBytes:0,drawCalls:0,triangles:0});window.measure=fresh();window.buffers=new Map();window.textures=new Map();
  const raf=requestAnimationFrame.bind(window);window.requestAnimationFrame=cb=>raf(t=>{window.gpuProfileFrame=(window.gpuProfileFrame||0)+1;const start=performance.now();cb(t);window.measure.cpu.push(performance.now()-start);window.measure.frames.push(t);});
  for(const [proto,names] of [[GPUDevice.prototype,['createBuffer','createTexture','createBindGroup','createRenderPipeline']],[GPUQueue.prototype,['writeBuffer','writeTexture','submit']],[GPURenderPassEncoder.prototype,['draw','drawIndexed']]])for(const name of names){const original=proto[name];proto[name]=function(...a){const m=window.measure;m.calls[name]=(m.calls[name]||0)+1;
   if(name==='draw'||name==='drawIndexed'){m.drawCalls++;m.triangles+=a[0]*(a[1]??1)/3;}
   if(name==='writeBuffer'){const data=a[2],unit=ArrayBuffer.isView(data)?data.BYTES_PER_ELEMENT||1:1;m.writeBytes+=a[4]===undefined?data.byteLength-(a[3]||0)*unit:a[4]*unit;}
   const result=original.apply(this,a);
   if(name==='createBuffer')window.buffers.set(result,a[0].size);
   if(name==='createTexture'){const d=a[0],size=Array.isArray(d.size)?d.size:[d.size.width,d.size.height||1,d.size.depthOrArrayLayers||1];window.textures.set(result,{size,format:d.format,samples:d.sampleCount||1,mips:d.mipLevelCount||1});}
   return result;
  };}
  for(const [proto,map] of [[GPUBuffer.prototype,'buffers'],[GPUTexture.prototype,'textures']]){const destroy=proto.destroy;proto.destroy=function(){window[map].delete(this);return destroy.call(this);};}
 });
 if(backend==='WebGL2')await page.addInitScript(()=>{
  const bound=new Map(),proto=WebGL2RenderingContext.prototype;
  for(const name of ['createBuffer','bindBuffer','bufferData','bufferSubData','deleteBuffer','drawArrays','drawElements','drawArraysInstanced','drawElementsInstanced','uniform1f','uniform1i','uniform2f','uniform3f','uniform4f','uniform1fv','uniform2fv','uniform3fv','uniform4fv','uniformMatrix3fv','uniformMatrix4fv']){
   const original=proto[name];proto[name]=function(...args){const m=window.measure;m.calls[name]=(m.calls[name]||0)+1;const result=original.apply(this,args);
    if(name==='bindBuffer')bound.set(args[0],args[1]);
    if(name==='bufferData'){const size=typeof args[1]==='number'?args[1]:args[1]?.byteLength||0;window.buffers.set(bound.get(args[0]),size);m.writeBytes+=typeof args[1]==='number'?0:size;}
    if(name==='bufferSubData')m.writeBytes+=args[2]?.byteLength||0;
    if(name==='deleteBuffer')window.buffers.delete(args[0]);
    if(name.startsWith('draw')){m.drawCalls++;const count=name.includes('Arrays')?args[2]:args[1],instances=name.endsWith('Instanced')?args.at(-1):1;if(args[0]===4)m.triangles+=count*instances/3;}
    return result;
   };
  }
 });
 if(runtime==='rust'){
  const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));Object.assign(catalog.examples.find(e=>e.id===id),{status:'partial',port:{example:ports[id],limitations:[]}});
  await page.route('**/web/gallery/catalog.json',r=>r.fulfill({json:catalog}));
 }
 await page.goto('http://127.0.0.1:8173'+(runtime==='three'?`/reference/three-js/expanded.html?id=${id}&animate=1`:`/web/gallery/example.html?id=${id}`));
 await page.waitForFunction(runtime==='three'?()=>document.querySelector('canvas')?.dataset.ready==='true':()=>document.querySelector('canvas')?.dataset.frames>5,null,{timeout:90000});
 if(['webgl_morphtargets','webgpu_morphtargets'].includes(id))await page.evaluate(async runtime=>{if(runtime==='rust')app.gallery_morph(.25,.75);else await window.renderFixture(0,[.25,.75]);},runtime);
 if(backend==='WebGL2'&&process.env.GPU)await page.evaluate(()=>{
  const renderer=reference.renderer,gl=renderer.getContext(),ext=gl.getExtension('EXT_disjoint_timer_query_webgl2');window.glTimerAvailable=!!ext;if(!ext)return;
  window.gpuTimings={passes:[]};const pending=[],render=renderer.render.bind(renderer);
  renderer.render=(...args)=>{
   while(pending.length&&gl.getQueryParameter(pending[0].query,gl.QUERY_RESULT_AVAILABLE)){const item=pending.shift(),ns=gl.getQueryParameter(item.query,gl.QUERY_RESULT);if(item.sample&&!gl.getParameter(ext.GPU_DISJOINT_EXT))window.gpuTimings.passes.push({label:'WebGL scene',ms:ns/1e6,frame:item.frame});gl.deleteQuery(item.query);}
   const query=gl.createQuery();gl.beginQuery(ext.TIME_ELAPSED_EXT,query);render(...args);gl.endQuery(ext.TIME_ELAPSED_EXT);pending.push({query,sample:!!window.sampleGPU,frame:window.gpuProfileFrame});
  };
 });
 await page.waitForTimeout(report.warmupMs);
 await page.evaluate(()=>{window.measure={cpu:[],frames:[],calls:{},writeBytes:0,drawCalls:0,triangles:0};window.sampleGPU=true;window.before=window.app?{transfer:JSON.parse(app.transfer_counts()),resources:Array.from(app.resource_counts())}:null;});
 await page.waitForTimeout(report.sampleMs);
 const result=await page.evaluate(async()=>{
  const m=window.measure,stats=a=>{a.sort((a,b)=>a-b);return {p50:a[Math.floor(a.length*.5)],p95:a[Math.floor(a.length*.95)],max:a.at(-1)};};
  const adapter=await navigator.gpu.requestAdapter();
  const frameTimes=new Map();for(const p of window.gpuTimings?.passes||[])frameTimes.set(p.frame,(frameTimes.get(p.frame)||0)+p.ms);
  return {gpuTimerAvailable:window.glTimerAvailable??null,gpuFrame:frameTimes.size?stats([...frameTimes.values()]):null,browser:navigator.userAgent,adapter:{vendor:adapter.info.vendor,architecture:adapter.info.architecture},canvas:[document.querySelector('canvas').width,document.querySelector('canvas').height],frames:m.frames.length,cpu:stats(m.cpu),interval:stats(m.frames.slice(1).map((v,i)=>v-m.frames[i])),calls:m.calls,writeBytes:m.writeBytes,drawCalls:m.drawCalls,triangles:m.triangles,logicalBufferBytes:[...window.buffers.values()].reduce((a,b)=>a+b,0),textures:[...window.textures.values()],before:window.before,after:window.app?{transfer:JSON.parse(app.transfer_counts()),resources:Array.from(app.resource_counts())}:null,gpu:window.gpuTimings?Object.fromEntries([...new Set(window.gpuTimings.passes.map(p=>p.label))].map(label=>[label,stats(window.gpuTimings.passes.filter(p=>p.label===label).map(p=>p.ms))])):null};
 });report.results.push({runtime,backend,...result,errors});console.log(runtime,JSON.stringify({cpu:result.cpu,calls:result.calls,gpu:result.gpu,errors}));await page.close();
}}finally{await browser.close();}
for(const result of report.results){
 if(result.before&&result.after){
  if(JSON.stringify(result.before.transfer.slice(0,3))!==JSON.stringify(result.after.transfer.slice(0,3)))result.errors.push('Steady-state geometry or morph source reupload');
  if(JSON.stringify(result.before.resources)!==JSON.stringify(result.after.resources))result.errors.push('Steady-state texture resource growth');
  if(!report.gpuInstrumentation&&['createBuffer','createTexture','createBindGroup','createRenderPipeline'].some(name=>result.calls[name]))result.errors.push('Steady-state GPU allocation');
 }
}
writeFileSync(process.env.OUT||`docs/${id}-performance.json`,JSON.stringify(report,null,2)+'\n');
if(report.results.some(r=>r.errors.length||!r.frames))process.exitCode=1;
