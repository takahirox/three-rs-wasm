import {test,expect} from '@playwright/test';
import {PNG} from 'pngjs';
import {writeFileSync} from 'node:fs';
test.use({deviceScaleFactor:Number(process.env.TSL_PROCEDURAL_DPR||1)});
test('TSL compute audio uses matching GPU pitch/delay and analyser rendering',async({page},info)=>{
 test.setTimeout(180000);
 await page.addInitScript(()=>{
  window.audioWaves=[];window.audioErrors=[];
  const copy=AudioBuffer.prototype.copyToChannel;AudioBuffer.prototype.copyToChannel=function(wave,...rest){audioWaves.push(Array.from(wave));return copy.call(this,wave,...rest);};
  // Feed both analysers the same known frequency bins; DSP waveforms are checked separately.
  AnalyserNode.prototype.getByteFrequencyData=function(bytes){for(let i=0;i<bytes.length;i++)bytes[i]=(i*17+(i>>3)*3)%256;};
  const request=GPUAdapter.prototype.requestDevice;GPUAdapter.prototype.requestDevice=async function(...args){const d=await request.apply(this,args);d.addEventListener('uncapturederror',e=>audioErrors.push(e.error.message));return d;};
 });
 const data={};
 for(const runtime of ['reference','rust']){
  await page.setViewportSize({width:512,height:512});await page.goto(runtime==='reference'?'/reference/three-js/tsl-audio.html':'/web/gallery/example.html?id=webgpu_compute_audio&still=1');
  await page.getByRole('button',{name:'Play',exact:true}).click();
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return audioWaves.length>0&&(c?.dataset.ready==='true'||Number(c?.dataset.frames)>0);},null,{timeout:90000});
  await page.addStyleTag({content:'#notice,#settings,#info{display:none!important}'});
  const frames=[];
  for(const parameter of [null,[0,.75],[1,.65],[2,.2]]){
   const before=await page.evaluate(()=>audioWaves.length);
   if(parameter){await page.evaluate(async({runtime,parameter:[i,v]})=>{if(runtime==='reference'){fixtureParameter(i,v);await fixturePlay();}else{app.tsl_parameter(i,v);app.audio_play();}},{runtime,parameter});await page.waitForFunction(n=>audioWaves.length>n,before);}
   if(runtime==='reference')await page.evaluate(()=>renderFixture());else await page.waitForTimeout(50);
   frames.push(PNG.sync.read(await page.locator('canvas').screenshot()));
  }
  data[runtime]={waves:await page.evaluate(()=>audioWaves),frames};expect(await page.evaluate(()=>audioErrors)).toEqual([]);
 }
 const report=[];
 for(let i=0;i<4;i++){
  const a=data.rust.waves[i],b=data.reference.waves[i];expect(a.length).toBe(b.length);
  let max=0,square=0;for(let j=0;j<a.length;j++){const d=Math.abs(a[j]-b[j]);max=Math.max(max,d);square+=d*d;}
  report.push({state:i,samples:a.length,max,rms:Math.sqrt(square/a.length)});expect(max).toBeLessThan(.00001);
  const x=data.rust.frames[i],y=data.reference.frames[i];let bad=0;for(let p=0;p<x.data.length;p+=4){if([0,1,2].some(c=>Math.abs(x.data[p+c]-y.data[p+c])>6))bad++;}expect(bad/(x.width*x.height)).toBeLessThanOrEqual(.005);
  for(const runtime of ['reference','rust'])writeFileSync(info.outputPath(`${i}-${runtime}.png`),PNG.sync.write(data[runtime].frames[i]));
 }
 writeFileSync(info.outputPath('audio-comparison.json'),JSON.stringify(report,null,2));
});

test('TSL audio GPU work and readback remain resident across plays and resize',async({page},info)=>{
 test.setTimeout(120000);
 await page.addInitScript(()=>{
  window.work={creates:0,dispatches:[],copies:[],attributeBytes:0};window.plays=0;
  const copyAudio=AudioBuffer.prototype.copyToChannel;AudioBuffer.prototype.copyToChannel=function(...a){plays++;return copyAudio.apply(this,a);};
  for(const name of ['createBuffer','createTexture','createBindGroup','createShaderModule','createRenderPipeline','createRenderPipelineAsync','createComputePipeline','createComputePipelineAsync']){const fn=GPUDevice.prototype[name];GPUDevice.prototype[name]=function(...a){work.creates++;return fn.apply(this,a);};}
  const dispatch=GPUComputePassEncoder.prototype.dispatchWorkgroups;GPUComputePassEncoder.prototype.dispatchWorkgroups=function(x,y=1,z=1){work.dispatches.push([x,y,z]);return dispatch.call(this,x,y,z);};
  const copy=GPUCommandEncoder.prototype.copyBufferToBuffer;GPUCommandEncoder.prototype.copyBufferToBuffer=function(...a){work.copies.push(a[4]);return copy.apply(this,a);};
  const write=GPUQueue.prototype.writeBuffer;GPUQueue.prototype.writeBuffer=function(buffer,offset,data,dataOffset,size){if(buffer.usage&(GPUBufferUsage.STORAGE|GPUBufferUsage.VERTEX|GPUBufferUsage.INDEX))work.attributeBytes+=size===undefined?data.byteLength-(dataOffset??0)*(data.BYTES_PER_ELEMENT??1):size*(data.BYTES_PER_ELEMENT??1);return write.apply(this,arguments);};
 });
 const reports=[];
 for(const runtime of ['reference','rust']){
  await page.goto(runtime==='reference'?'/reference/three-js/tsl-audio.html':'/web/gallery/example.html?id=webgpu_compute_audio&still=1');await page.getByRole('button',{name:'Play',exact:true}).click();
  await page.waitForFunction(()=>{const c=document.querySelector('canvas');if(c?.dataset.error)throw Error(c.dataset.error);return plays>0&&(c?.dataset.ready==='true'||Number(c?.dataset.frames)>0);});
  const play=async(reset=false)=>{const prev=await page.evaluate(()=>plays);await page.evaluate(async({runtime,reset})=>{if(reset)work={creates:0,dispatches:[],copies:[],attributeBytes:0};if(runtime==='reference')await fixturePlay();else app.audio_play();},{runtime,reset});await page.waitForFunction(n=>plays>n,prev);if(runtime==='reference')await page.evaluate(()=>renderFixture());else await page.waitForTimeout(50);};
  for(const resize of [false,true]){
   if(resize){await page.setViewportSize({width:640,height:400});await page.waitForTimeout(100);}
   await play();await play(true);const measured=await page.evaluate(()=>work);reports.push({runtime,resize,...measured});
   expect(measured.dispatches).toEqual([[5614,1,1]]);expect(measured.copies).toContain(359242*4);expect(measured.attributeBytes).toBe(0);
   if(runtime==='rust')expect(measured.creates).toBe(0);
  }
 }
 writeFileSync(info.outputPath('audio-gpu-work.json'),JSON.stringify(reports,null,2));
});
