import {test,expect} from '@playwright/test';
import {readFileSync,readdirSync,writeFileSync} from 'node:fs';
// webgpu_tsl_transpiler: the Rust transpiler ( src/transpiler/ ) against the r186 one
// ( examples/jsm/transpiler/ ), both run in the same page. The output text and the
// error messages must match character for character, for both encoders.
const html=readFileSync('.cache/three-r186/examples/webgpu_tsl_transpiler.html','utf8');
const pageGlsl=html.match(/const glslCode = `([\s\S]*?)`;/)[1];
const shaders=readdirSync('.cache/three-r186/examples/jsm/shaders').filter(f=>f.endsWith('.js'));
// Statements, qualifiers, textures, preprocessor directives and malformed input.
const written=JSON.parse(readFileSync('tests/browser/transpiler-cases.json','utf8'));

test('Transpiler output matches the original for every GLSL input',async({page},info)=>{
 test.setTimeout(600000);
 await page.goto('/reference/three-js/compute-examples.html?id=none');
 const report=await page.evaluate(async({pageGlsl,shaders,written})=>{
  const Transpiler=(await import('/.cache/three-r186/examples/jsm/transpiler/Transpiler.js')).default;
  const GLSLDecoder=(await import('/.cache/three-r186/examples/jsm/transpiler/GLSLDecoder.js')).default;
  const TSLEncoder=(await import('/.cache/three-r186/examples/jsm/transpiler/TSLEncoder.js')).default;
  const WGSLEncoder=(await import('/.cache/three-r186/examples/jsm/transpiler/WGSLEncoder.js')).default;
  const rust=await import('/web/pkg/three_rs_wasm.js');await rust.default({module_or_path:'/web/pkg/three_rs_wasm_bg.wasm'});
  // The page's GLSL, every three.js shader chunk and every examples shader.
  const base=[['page',pageGlsl]];
  const {ShaderChunk}=await import('/.cache/three-r186/src/renderers/shaders/ShaderChunk.js');
  for(const [name,glsl] of Object.entries(ShaderChunk))if(typeof glsl==='string')base.push(['chunk/'+name,glsl]);
  for(const file of shaders){
   const module=await import('/.cache/three-r186/examples/jsm/shaders/'+file);
   for(const [name,shader] of Object.entries(module))if(shader&&typeof shader==='object')for(const key of ['vertexShader','fragmentShader'])if(typeof shader[key]==='string')base.push([`shaders/${file}/${name}.${key}`,shader[key]]);
  }
  written.forEach((glsl,i)=>base.push(['written/'+i,glsl]));
  // Seeded mutations of those inputs: dropped, inserted and repeated tokens and
  // characters, preprocessor fragments, line terminators and non-ASCII text.
  let seed=12345;const random=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;};
  const pick=items=>items[Math.floor(random()*items.length)];
  const pieces=['#','\r','"',"'",'é','\u2028','\\','/*','*/','//','\n','\t',' ','0x','1e','.',';','{','}','(',')','#define X 1\n','#if X\n','#else\n','#endif\n','#ifdef X\n','#define F(a,b) a+b\n','F(1,2)','defined','?',':','++','--','[',']','$&','struct S { float a; };','switch(i){case 1:break;default:break;}','do{}while(false);'];
  const cases=[...base];
  for(let n=0;n<1500;n++){
   const tokens=pick(base)[1].split(/(\s+|[;(){}\[\],.=+\-*\/<>!?:])/).filter(t=>t!=='');
   for(let k=0,K=1+Math.floor(random()*3);k<K;k++){
    const i=Math.floor(random()*tokens.length);const op=random();
    if(op<.4)tokens.splice(i,1);else if(op<.7)tokens.splice(i,0,pick(tokens));else{const j=Math.floor(random()*tokens.length);[tokens[i],tokens[j]]=[tokens[j],tokens[i]];}
   }
   cases.push(['tokens/'+n,tokens.join('')]);
  }
  for(let n=0;n<4000;n++){
   let glsl=pick(base)[1];
   for(let k=0,K=1+Math.floor(random()*4);k<K;k++){
    const i=Math.floor(random()*(glsl.length+1));const op=random();
    if(op<.35)glsl=glsl.slice(0,i)+glsl.slice(i+1+Math.floor(random()*4));else if(op<.8)glsl=glsl.slice(0,i)+pick(pieces)+glsl.slice(i);else{const j=Math.floor(random()*glsl.length);glsl=glsl.slice(0,i)+glsl.slice(j,j+8)+glsl.slice(i);}
   }
   cases.push(['characters/'+n,glsl]);
  }
  const original=(glsl,Encoder)=>{try{return new Transpiler(new GLSLDecoder(),new Encoder()).parse(glsl);}catch(e){return 'Error: '+e.message;}};
  const port=(glsl,encoder)=>{try{return rust.tsl_transpile(glsl,encoder);}catch(e){return 'Error: '+e;}};
  const mismatches=[];let errors=0;
  for(const [name,glsl] of cases)for(const [encoder,Encoder] of [['TSL',TSLEncoder],['WGSL',WGSLEncoder]]){
   const want=original(glsl,Encoder),got=port(glsl,encoder);
   if(want.startsWith('Error: '))errors++;
   if(want!==got)mismatches.push({name,encoder,glsl,want,got});
  }
  return {cases:cases.length,base:base.length,errors,count:mismatches.length,mismatches:mismatches.slice(0,20)};
 },{pageGlsl,shaders,written});
 writeFileSync(info.outputPath('transpiler.json'),JSON.stringify(report,null,1));
 expect(report.base).toBeGreaterThan(250);
 expect(report.errors).toBeGreaterThan(1000);
 expect(report.count,JSON.stringify(report.mismatches[0])).toBe(0);
});

test('Gallery page transpiles with the page editors and encoder selection',async({page})=>{
 test.setTimeout(180000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto('/web/gallery/example.html?id=webgpu_tsl_transpiler');
 const builds=()=>page.evaluate(()=>Number(document.body.dataset.builds||0));
 await expect.poll(builds,{timeout:90000}).toBe(1);
 // The original runs in the fixture page, whose import map resolves three/webgpu.
 const fixture=await page.context().newPage();await fixture.goto('/reference/three-js/compute-examples.html?id=none');
 const original=(glsl,encoder)=>fixture.evaluate(async({glsl,encoder})=>{
  const Transpiler=(await import('/.cache/three-r186/examples/jsm/transpiler/Transpiler.js')).default;
  const GLSLDecoder=(await import('/.cache/three-r186/examples/jsm/transpiler/GLSLDecoder.js')).default;
  const Encoder=(await import(`/.cache/three-r186/examples/jsm/transpiler/${encoder}Encoder.js`)).default;
  try{return new Transpiler(new GLSLDecoder(),new Encoder()).parse(glsl);}catch(e){return 'Error: '+e.message;}
 },{glsl,encoder});
 const shown=()=>page.evaluate(()=>window.transpilerEditors.result.getValue());
 const language=()=>page.evaluate(()=>window.transpilerEditors.result.getModel().getLanguageId());
 expect(await shown()).toBe(await original(pageGlsl,'TSL'));
 expect(await language()).toBe('javascript');
 await page.selectOption('#encoder-select','WGSL');await expect.poll(builds).toBe(2);
 expect(await shown()).toBe(await original(pageGlsl,'WGSL'));
 expect(await language()).toBe('wgsl');
 // Editing the source rebuilds a second after the last change, as the page's timeout does.
 const edited='uniform float time;\nvarying vec2 vUv;\nvoid main() { gl_FragColor = vec4( vUv, sin( time ), 1.0 ); }';
 await page.evaluate(glsl=>window.transpilerEditors.editor.setValue(glsl),edited);
 await page.waitForTimeout(500);expect(await builds()).toBe(2);
 await expect.poll(builds).toBe(3);
 expect(await shown()).toBe(await original(edited,'WGSL'));
 await page.selectOption('#encoder-select','TSL');await expect.poll(builds).toBe(4);
 expect(await shown()).toBe(await original(edited,'TSL'));
 // Errors show the original's message.
 const broken='void main() { float x = ; }';
 await page.evaluate(glsl=>window.transpilerEditors.editor.setValue(glsl),broken);await expect.poll(builds).toBe(5);
 expect(await shown()).toBe(await original(broken,'TSL'));
 expect(await shown()).toMatch(/^Error: /);
 expect(errors).toEqual([]);
});
