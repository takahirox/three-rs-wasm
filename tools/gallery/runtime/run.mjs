// Run each real upstream example in an isolated browser context, then capture the
// scene for the separate Rust renderer probe. No generated scene is a product port.
import {chromium} from '@playwright/test';
import {readFileSync,writeFileSync,mkdirSync,existsSync} from 'node:fs';
import config from '../../../playwright.config.js';
const catalog=JSON.parse(readFileSync('web/gallery/catalog.json'));
const root='.cache/gallery-runtime';mkdirSync(root,{recursive:true});
const wanted=process.argv.slice(2);
const cases=catalog.examples.filter(e=>e.status!=='excluded'&&(!wanted.length||wanted.includes(e.id)));
const browser=await chromium.launch({...config.use.launchOptions,args:[...config.use.launchOptions.args,'--mute-audio'],headless:true});
const init=readFileSync('tools/gallery/runtime/capture.js','utf8');
const timeout=12000;
function shim(module,webgpu){return `
import * as Original from '${module}';
export * from '${module}';
const probe=globalThis.__galleryProbe;probe.Material=Original.Material;
export class ${webgpu?'WebGPURenderer':'WebGLRenderer'} extends Original.${webgpu?'WebGPURenderer':'WebGLRenderer'} {
 constructor(...args){super(...args);const draw=this.render.bind(this);this.render=(scene,camera)=>{if(this.getRenderTarget())return draw(scene,camera);probe.reached(scene,camera,this);};${webgpu?`this.renderAsync=async(scene,camera)=>{if(this.getRenderTarget())return Original.WebGPURenderer.prototype.renderAsync.call(this,scene,camera);probe.reached(scene,camera,this);};this.compute=()=>probe.unsupported('GPU compute/storage dispatch');this.computeAsync=async()=>probe.unsupported('GPU compute/storage dispatch');`:''}}
}
${webgpu?`export class RenderPipeline extends Original.RenderPipeline {render(){probe.unsupported('Programmable postprocessing pipeline');} async renderAsync(){probe.unsupported('Programmable postprocessing pipeline');}}`:''}
`;}
let done=0;
try{
 for(const entry of cases){
  const file=`${root}/${entry.id}.result.json`;
  if(!wanted.length&&existsSync(file)){done++;continue;}
  const context=await browser.newContext({viewport:{width:512,height:512},deviceScaleFactor:1,acceptDownloads:false});
  const page=await context.newPage();const errors=[],failed=[];let pending=0,lastRequest=Date.now();
  page.on('pageerror',e=>{if(errors.length<5)errors.push(String(e).slice(0,500));});
  page.on('request',()=>{pending++;lastRequest=Date.now();});
  page.on('requestfinished',()=>{pending--;});
  page.on('requestfailed',r=>{pending--;if(failed.length<5)failed.push(r.url().replace(/^http:\/\/127.0.0.1:8175/,''));});
  page.on('dialog',d=>d.dismiss());
  await page.addInitScript({content:init});
  await page.route('**/build/three.module.js',r=>r.fulfill({contentType:'text/javascript',body:shim('/.cache/three-r186/src/Three.js',false)}));
  await page.route('**/build/three.webgpu.js',r=>r.fulfill({contentType:'text/javascript',body:shim('/.cache/three-r186/src/Three.WebGPU.js',true)}));
  await page.route('**/build/three.tsl.js',r=>r.fulfill({contentType:'text/javascript',body:"export * from '/.cache/three-r186/src/Three.TSL.js';"}));
  for(const name of ['CSS2DRenderer','CSS3DRenderer','SVGRenderer'])await page.route(`**/examples/jsm/renderers/${name}.js`,r=>r.fulfill({contentType:'text/javascript',body:`import {${name} as Base} from './${name}.js?original=1';export * from './${name}.js?original=1';export class ${name} extends Base {constructor(...args){super(...args);this.render=()=>globalThis.__galleryProbe.unsupported('${name} scene rendering');}}`}));
  await page.route('**/examples/jsm/postprocessing/EffectComposer.js',r=>r.fulfill({contentType:'text/javascript',body:"import {EffectComposer as Base} from './EffectComposer.js?original=1';export class EffectComposer extends Base {render(){globalThis.__galleryProbe.unsupported('EffectComposer postprocessing pipeline');}}"}));
  await page.route('**/examples/jsm/transpiler/Transpiler.js',r=>r.fulfill({contentType:'text/javascript',body:"import Base from './Transpiler.js?original=1';export default class Transpiler extends Base {parse(){globalThis.__galleryProbe.unsupported('GLSL to TSL/WGSL transpilation');}}"}));
  await page.route('**/examples/jsm/utils/UVsDebug.js',r=>r.fulfill({contentType:'text/javascript',body:"export function UVsDebug(){globalThis.__galleryProbe.unsupported('Canvas 2D UV debug visualization');}"}));
  const started=Date.now();let result;const watchdog=setTimeout(()=>context.close().catch(()=>{}),25000);
  try{
   await page.goto(`http://127.0.0.1:8175/.cache/three-r186/examples/${entry.id}.html`,{waitUntil:'domcontentloaded',timeout});
   if(['webgpu_compute_audio','webgl_postprocessing_glitch','webaudio_sandbox','webaudio_orientation','webaudio_timing','webaudio_visualizer'].includes(entry.id))await page.locator('#startButton').click({timeout:5000});
   while(Date.now()-started<timeout){
    const state=await page.evaluate(()=>{const p=window.__galleryProbe;let geometry=0;p.frame?.scene.traverseVisible(o=>{if(o.geometry)geometry++;});return {frame:!!p.frame,geometry,blocked:p.blocked.length};});
    if(state.blocked||((state.frame||errors.length)&&(!entry.id.includes('loader')||state.geometry>0||errors.length)&&pending===0&&Date.now()-lastRequest>800&&Date.now()-started>1200))break;
    await page.waitForTimeout(150);
   }
   const snapshot=await page.evaluate(()=>{const s=window.__galleryProbe.snapshot();window.__gallerySnapshotJSON=JSON.stringify(s);return {blockers:s.blockers,observed:s.observed};});
   const frame=await page.evaluate(()=>!!window.__galleryProbe.frame);
   if(frame||snapshot.blockers.some(b=>b!=='No scene reached the renderer')){
    // The Rust probe gets actual observed unsupported calls as well as full scenes.
    writeFileSync(`${root}/${entry.id}.scene.json`,await page.evaluate(()=>window.__gallerySnapshotJSON));
    result={stage:snapshot.blockers.length?'runtime-prerequisite':'captured-scene',blockers:snapshot.blockers,observed:snapshot.observed??null};
   }else result={stage:'reference-not-started',blockers:snapshot.blockers};
  }catch(error){result={stage:'reference-error',error:String(error).slice(0,600)};}
  result={id:entry.id,source_sha256:entry.source_sha256,...result,elapsed_ms:Date.now()-started,pending_requests:pending,errors,failed_requests:failed};
  writeFileSync(file,JSON.stringify(result,null,2)+'\n');
  clearTimeout(watchdog);await context.close();done++;
  process.stdout.write(`${done}/${cases.length} ${entry.id}: ${result.stage}\n`);
 }
}finally{await browser.close();}
