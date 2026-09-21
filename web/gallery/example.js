import {installElements} from './elements.js';
import {installOrbit} from './orbit.js';
const catalog = await (await fetch('catalog.json')).json();
const id = new URLSearchParams(location.search).get('id');
const entry = catalog.examples.find(example => example.id === id);
const canvas = document.querySelector('canvas');
const loading = document.querySelector('#loading');
const diagnostic = document.querySelector('#diagnostic');
let app;
let closing = false;
let pending = 0;
const release = () => { if (closing && pending === 0) {app?.free(); app = null;} };
window.addEventListener('pagehide', () => { closing = true; release(); });
function text(tag, value, parent) { const element = document.createElement(tag); element.textContent = value; parent.append(element); return element; }
function explain(error) {
 loading.hidden = true; canvas.hidden = true; diagnostic.hidden = false;
 text('h1', entry?.id.replaceAll('_', ' / ') || 'Unknown example', diagnostic);
 text('p', error || 'このexampleはまだRustへ移植されていません。', diagnostic);
 if (!entry) return;
 if (entry.preferred_example) {
  const preferred = catalog.examples.find(e => e.id === entry.preferred_example);
  const link = text('a', `WebGPU版: ${entry.preferred_example}${preferred?.port ? '' : '（移植待ち）'}`, diagnostic);
  link.href = `example.html?id=${encodeURIComponent(entry.preferred_example)}`;
 }
 const preview = document.createElement('img'); preview.src = `screenshots/${entry.id}.jpg`; preview.alt = 'Three.js公式のプレビュー（Rustの実行結果ではありません）'; diagnostic.append(preview);
 text('p', preview.alt, diagnostic);
 const source = text('a', '固定したThree.jsのソースを見る ↗', diagnostic); source.href = entry.source; source.target = '_blank'; source.rel = 'noopener';
 text('h2', '必要機能の調査', diagnostic);
 text('p', '以下はソースから検出した機能です。実行検証済みの不足一覧ではなく、移植時に確認する対象を示します。', diagnostic);
 const list = document.createElement('ul'); diagnostic.append(list);
 for (const [feature, evidence] of Object.entries(entry.requirements)) {
  const li = text('li', catalog.features[feature], list);
  const details = document.createElement('details'); li.append(details); text('summary', '参照ソース', details);
  for (const item of evidence) { const link = text('a', `L${item.line}: ${item.text}`, text('p', '', details)); link.href = `${entry.source}#L${item.line}`; link.target = '_blank'; link.rel = 'noopener'; }
 }
 if (!list.children.length) text('p', '専用のシーン移植と比較テストが必要です。未対応機能なし、という判定ではありません。', diagnostic);
 document.body.dataset.status = entry.status;
}
if (!entry || entry.status === 'excluded' || !entry.port) {
 explain(entry?.excluded_reason);
} else {
 try {
  if (!navigator.gpu) throw new Error('WebGPU対応ブラウザが必要です。');
  // Keep the glue and Wasm on the same cache revision when the runtime API changes.
  const runtimeRevision = 'tsl-9';
  const {default:init, BrowserApp} = await import(`../pkg/three_rs_wasm.js?v=${runtimeRevision}`);
  await init({module_or_path:new URL(`../pkg/three_rs_wasm_bg.wasm?v=${runtimeRevision}`,import.meta.url)});
  const resize = () => { canvas.width = Math.max(1, Math.round(innerWidth * devicePixelRatio)); canvas.height = Math.max(1, Math.round(innerHeight * devicePixelRatio)); app?.request_render(); };
  resize(); canvas.hidden = false;
  app = await BrowserApp.create(canvas, entry.port.example, !new URLSearchParams(location.search).has('still'));
  if (closing) { app.free(); app = null; } else {
   window.app = app;
   loading.hidden = true;
   document.body.dataset.status = 'running';
   document.body.dataset.backend = 'rust-wasm-webgpu';
   const notice = document.querySelector('#notice'); notice.hidden = false;
   for (const limitation of entry.port.limitations) text('p', limitation, notice.querySelector('div'));
   if ([30,31,32,33,34,103,104].includes(entry.port.example)) {
    const credit=text('a','Model and environment credits',notice.querySelector('div'));
    credit.href='../THIRD_PARTY.md';credit.target='_blank';credit.rel='noopener';
   }
   if (entry.port.example === 28) {
    const credit = text('a', 'Forest House — peachyroyalty · CC BY-NC 4.0', notice.querySelector('div'));
    credit.href = '../THIRD_PARTY.md'; credit.target = '_blank'; credit.rel = 'noopener';
   }
   if (entry.port.example === 29) {
    const credit=text('a','Iridescence Lamp — Wayfair / Eric Chadwick · CC BY 4.0',notice.querySelector('div'));
    credit.href='../THIRD_PARTY.md';credit.target='_blank';credit.rel='noopener';
   }
   addEventListener('resize', resize);
   if([92,93].includes(entry.port.example))installElements(canvas,app,entry.port.example===93);
   if(entry.port.example===97){for(const [i,label] of ['DataArrayTexture','Data3DTexture','RenderTargetArray','RenderTarget3D'].entries()){const item=text('div',label,document.body);item.className='viewport-label';item.style.cssText=`position:absolute;bottom:${i<2?2:52}%;left:${i%2?52:2}%;color:white;background:rgba(0,0,0,.7);padding:5px 10px;border-radius:4px;font-family:monospace;pointer-events:none;user-select:none`;}}
   if ([16,18,20,25,28,29,30,31,32,33,34,41,48,51,53,56,58,59,60,62,63,64,66,67,68,69,70,71,72,73,75,76,77,81,82,83,87,88,89,90,91,94,95,96,97,98,99,101,102,103,104,105,106,107].includes(entry.port.example)) { installOrbit(canvas,app,{touchRotate:entry.port.example!==56}); } else if (entry.port.example < 35) {
   let drag;
   canvas.addEventListener('pointerdown', event => { drag = [event.clientX, event.clientY]; canvas.setPointerCapture(event.pointerId); app.gallery_input(0,0,0,true); });
   canvas.addEventListener('pointermove', event => { if(event.isPrimary===false)return;app.gallery_pointer(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2); if (drag) { app.gallery_input(event.clientX-drag[0],event.clientY-drag[1],0,true); app.orbit(event.clientX-drag[0], event.clientY-drag[1], 0); drag = [event.clientX,event.clientY]; } });
   for (const event of ['pointerup','pointercancel','lostpointercapture']) canvas.addEventListener(event, () => { drag = null; app?.gallery_input(0,0,0,false); });
   canvas.addEventListener('wheel', event => { event.preventDefault(); app.gallery_input(0,0,event.deltaY,false); app.orbit(0,0,event.deltaY); }, {passive:false});
   }
   if (entry.port.example===73) canvas.addEventListener('pointerdown',event=>app.gallery_select(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2));
   if (entry.port.example===75) window.addEventListener('pointermove',event=>app.gallery_pointer(event.clientX/canvas.clientWidth*2-1,1-event.clientY/canvas.clientHeight*2));
   if ([39,50,55,56].includes(entry.port.example)) canvas.addEventListener('pointermove',event=>app.gallery_pointer(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2));
   if ([46,50].includes(entry.port.example)) document.body.style.background = '#000';
   const settings = document.querySelector('#settings');
   if (entry.port.example >= 48) {
    const controls={103:[],104:[["mix",-1,2,0,.01],["blurBackground",0,1,0,.01],["offsetHDR1",0,Math.PI*2,0,.01],["offsetHDR2",0,Math.PI*2,0,.01],["procedural",0,1,0,.01],["intensity",0,5,1,.01],["hue",0,Math.PI*2,0,.01],["saturation",0,2,1,.01]],105:[["box projected",true],["roughness",0,1,.25,.01]],106:[["alpha",0,1,.5,.01],["alphaHash",true],["sampleLevel",0,4,3,1]],107:[["enabled",true],["Strength",0,3,1.5,.01],["Center X",-1,1,.5,.01],["Center Y",-1,1,.5,.01],["Scale",.5,2,1.2,.01],["animated",true],["autoRotate",true]],98:[],99:[["enabled",true]],100:[["enabled",true],["autoRotate",true]],101:[["lut",["Bourbon 64.CUBE","Chemical 168.CUBE","Clayton 33.CUBE","Cubicle 99.CUBE","Remy 24.CUBE","Presetpro-Cinematic.3dl","NeutralLUT","B&WLUT","NightLUT"],0],["intensity",0,1,1,.01]],102:[["Background Blurriness",0,1,.4,.01],["Parallax Scale",.2,.5,.5,.01],["UV Scale",1,5,3,.01]],96:[],97:[],95:[['alphaToCoverage',true],['minWidth',1,30,6,1],['maxWidth',2,30,20,1],['pulseSpeed',1,20,6,.1]],94:[],93:[],92:[],91:[['focus distance',10,3000,500,1],['focal length',50,750,200,1],['bokeh scale',1,20,10,.1]],90:[],89:[],88:[['atmosphereDayColor','#4db2ff'],['atmosphereTwilightColor','#bc490b'],['roughnessLow',0,1,.25,.001],['roughnessHigh',0,1,.35,.001]],87:[['intensity',0,10,5,.01],['threshold',0,.9,.3,.01],['samples',2,128,80,1],['tint color','#7a8aff'],['bloom radius',0,1,0,.01],['time scale',0,1,.5,.01]],86:[],85:[['sampling',['normal','centroid','sample','flat first','flat either'],0]],84:[],83:[['threshold',0,1,.08,.01],['opacity',0,1,.08,.01],['range',0,1,.1,.01],['steps',0,200,100,1]],81:[['threshold',0,1,.6,.01],['steps',0,300,200,1],['refine',true]],82:[['threshold',0,1,.25,.01],['opacity',0,1,.25,.01],['range',0,1,.1,.01],['steps',0,200,100,1]],80:[],78:[['multisampling',true],['animated',true]],79:[['Red',true],['Yellow',true],['Green',true]],48:[['Density',.001,.1,.04,.0001],['Height',-5,5,2,.01]],49:[],50:[['size attenuation',true]],51:[['size',0,1,.08,.001],['color inside','#ffa575'],['color outside','#311599']],52:[['damp',.25,1,.8,.01],['enabled',true]],53:[['speed',0,1,.2,.01]],54:[['instance count',1,1000,1000,1]],55:[['limit x',0,1,1,.01],['limit y',0,1,1,.01]],56:[['gravity',-.0098,0,-.00098,.0001],['bounce',.1,1,.8,.01],['friction',.96,.99,.99,.01],['size',.12,.5,.12,.01]],57:[],58:[],61:[],62:[],63:[],64:[],65:[],66:[],67:[],68:[['roughness',0,1,.5,.01],['metalness',0,1,.5,.01]],69:[],73:[['threshold',0,1,0,.01],['strength',0,3,1,.01],['radius',0,1,0,.01],['exposure',.1,3,1,.01]],75:[['elasticity',0,.5,.4,.01],['damping',.9,.98,.94,.001],['brush size',.1,.5,.25,.01],['brush strength',.1,.3,.22,.01]],76:[['emissiveColor','#ff8b4d'],['timeScale',-1,1,.2,.01],['parabolStrength',0,2,1,.01],['parabolOffset',0,1,.3,.01],['parabolAmplitude',0,2,.2,.01],['strength',0,10,1,.01],['radius',0,1,.1,.01]],77:[],74:[],70:[],71:[['threshold',0,1,0,.01],['strength',0,3,1,.01],['radius',0,1,0,.01],['exposure',.1,2,1,.01]],72:[['strength',0,5,2.5,.01],['radius',0,1,.5,.01],['exposure',.1,2,1,.01]],60:[["ambient", 0, 10, 3, 0.001], ["directional", 0, 20, 8, 0.001], ["color", "#fb00ff"], ["count", 1, 200, 140, 1], ["x", -1, 1, -0.4, 0.01], ["y", -1, 1, -1, 0.01], ["z", -1, 1, 0.5, 0.01], ["start", -1, 1, 1, 0.01], ["end", -1, 1, 0, 0.01], ["mix low", 0, 1, 0, 0.01], ["mix high", 0, 1, 0.5, 0.01], ["radius", 0, 1, 0.8, 0.01], ["color", "#94ffd1"], ["count", 1, 200, 180, 1], ["x", -1, 1, 0.5, 0.01], ["y", -1, 1, 0.5, 0.01], ["z", -1, 1, -0.2, 0.01], ["start", -1, 1, 0.55, 0.01], ["end", -1, 1, 0.2, 0.01], ["mix low", 0, 1, 0.5, 0.01], ["mix high", 0, 1, 1, 0.01], ["radius", 0, 1, 0.8, 0.01], ["default color", "#ff622e"]],59:[['color','#271442'],['roughness',0,1,.15,.001],['emissive color','#ff0a81'],['low',-1,0,-.25,.001],['high',0,1,.2,.001],['power',1,10,7,1],['large speed',0,5,1.25,.01],['large multiplier',0,1,.15,.01],['frequency x',0,10,3,.01],['frequency y',0,10,1,.01],['small iterations',0,5,3,1],['small frequency',0,10,2,.01],['small speed',0,1,.3,.01],['small multiplier',0,1,.18,.01],['normal shift',0,.1,.01,.0001]]}[entry.port.example];
    settings.hidden=!controls.length;
    controls.forEach(([name,min,max,value,step],i)=>{const label=text('label',name+' ',settings),input=document.createElement(Array.isArray(min)?'select':'input');input.id='particle-'+i;
     if(Array.isArray(min)){min.forEach((name,index)=>input.add(new Option(name,index)));input.value=max;}else{
     input.type=typeof min==='boolean'?'checkbox':typeof min==='string'?'color':'range';if(input.type==='checkbox')input.checked=min;else if(input.type==='color')input.value=min;else Object.assign(input,{min,max,value,step});
     }label.append(input);input.addEventListener('input',()=>app.tsl_parameter(i,input.type==='checkbox'?Number(input.checked):input.type==='color'?parseInt(input.value.slice(1),16):Number(input.value)));
    });
   }
   if (entry.port.example >= 43 && entry.port.example <= 47) {
    const controls = {
     43:[['Saturation',0,1,0,.01]],
     44:[['weight',0,1,.9,.01],['decay',0,1,.95,.01],['sample count',16,64,32,1],['exposure',1,10,5,1],['enabled',true],['animated',true]],
     45:[['enabled',true],['animated',false]],
     46:[['sample level',0,5,3,1],['clear color',['black','white','blue','green','red'],0],['clear alpha',0,1,1,.01],['view offset X',-100,100,0,1],['auto rotate',true]],
     47:[['animate scene',true],['animate transition',true],['transition',0,1,0,.01],['use texture',true],['texture',['Perlin','Squares','Cells','Distort','Gradient','Radial'],5],['cycle',true],['threshold',0,1,.1,.01]]
    }[entry.port.example];
    settings.hidden=false;
    controls.forEach(([name,min,max,value,step],i)=>{
     const label=text('label',name+' ',settings);let input;
     if(Array.isArray(min)){input=document.createElement('select');min.forEach((name,index)=>{const option=new Option(name,index);input.add(option);});input.value=max;}
     else{input=document.createElement('input');input.type=typeof min==='boolean'?'checkbox':'range';if(input.type==='checkbox')input.checked=min;else{Object.assign(input,{min,max,value,step});}}
     input.id='filter-'+i;label.append(input);input.addEventListener('input',()=>app.tsl_parameter(i,input.type==='checkbox'?Number(input.checked):Number(input.value)));
    });
   }
   if (entry.port.example === 41) {
    settings.hidden=false;settings.innerHTML='<label>speed <input id="tsl-speed" type="range" min="0" max="2" value="0" step="0.01"></label>';
    settings.addEventListener('input',()=>app.tsl_parameter(0,Number(document.querySelector('#tsl-speed').value)));
   }
   if ([35,37].includes(entry.port.example)) {
    settings.hidden=false;
    const fields=entry.port.example===35 ? [[1,'Cell Size',6,6,50,1],[2,'Cell Offset',.5,0,1,.1],[3,'Border Mask',1,0,5,.1],[4,'Pulse Intensity',.06,0,.5,.01],[5,'Pulse Width',60,10,100,5],[6,'WGSL Shader Speed',1,1,10,.1],[7,'TSL Shader Speed',1,1,10,.1]] : [[1,'uv scale ( before rtt )',4,1,10,.1],[2,'blur amount ( after rtt )',.5,0,2,.01]];
    for (const [index,name,value,min,max,step] of fields) {
     const label=text('label',name,settings),input=document.createElement('input');
     Object.assign(input,{type:'range',id:`tsl-${index}`,min,max,step,value});label.append(input);
     input.addEventListener('input',()=>app.tsl_parameter(index,Number(input.value)));
    }
    if (entry.port.example===37) {
     const label=text('label','auto update',settings),input=document.createElement('input');
     Object.assign(input,{type:'checkbox',id:'tsl-auto',checked:true});label.append(input);
     input.addEventListener('change',()=>app.tsl_parameter(3,Number(input.checked)));
    }
   }
   if (entry.port.example === 31) {
    settings.hidden=false;settings.innerHTML='<strong>SheenChair_fabric</strong><label>Sheen <input id="sheen" type="range" min="0" max="1" value="1" step="0.01"></label>';
    settings.addEventListener('input',()=>app.gallery_sheen(Number(document.querySelector('#sheen').value)));
   }
   if ([4,5,6].includes(entry.port.example)) {
    settings.hidden = false;
    settings.innerHTML = `<label>Model <select id="model"><option value="4">DamagedHelmet</option><option value="5">BoomBox</option></select></label><label>Exposure <input id="exposure" type="range" min="0.1" max="3" step="0.05" value="1"></label><label>Environment <input id="intensity" type="range" min="0" max="3" step="0.05" value="1"></label><label>Rotation <input id="rotation" type="range" min="-3.14159" max="3.14159" step="0.01" value="0"></label><label>Background blur <input id="blur" type="range" min="0" max="1" step="0.01" value="${entry.port.example===6?0.5:0}"></label><label>Background <input id="background" type="checkbox" checked></label><p id="asset-status" role="status"></p>`;
    const apply = () => app.gltf_controls(...['exposure','intensity','rotation','blur'].map(id => Number(document.getElementById(id).value)), document.querySelector('#background').checked);
    settings.addEventListener('input', apply);
    if (entry.port.example === 6) document.querySelector('#model').parentElement.remove();
    else document.querySelector('#model').addEventListener('change', async event => {
     const status = document.querySelector('#asset-status'); status.textContent = 'Loading…';
     pending++;
     try { const loaded = await app.load_model(Number(event.target.value)); if (!closing && loaded) {apply(); status.textContent = '';} } catch (error) {if (!closing) status.textContent = String(error);} finally {pending--; release();}
    });
   }
   if (entry.port.example === 15) {
    settings.hidden=false;const names=JSON.parse(app.animation_names());
    const label=text('label','Animation ',settings),select=document.createElement('select');select.id='animation';label.append(select);
    names.forEach((name,index)=>{const option=text('option',name,select);option.value=index;option.selected=name==='Walking';});
    select.addEventListener('change',()=>app.select_animation(Number(select.value)));
   }
   if (entry.port.example === 22) {
    settings.hidden=false;settings.innerHTML='<label>Wireframe <input id="wireframe" type="checkbox"></label>';
    settings.addEventListener('input',()=>app.gallery_wireframe(document.querySelector('#wireframe').checked));
   }
   if (entry.port.example === 18) {
    settings.hidden=false;settings.innerHTML='<strong>Morph Targets</strong><label>Spherify <input id="spherify" type="range" min="0" max="1" value="0" step="0.01"></label><label>Twist <input id="twist" type="range" min="0" max="1" value="0" step="0.01"></label>';
    settings.addEventListener('input',()=>app.gallery_morph(Number(document.querySelector('#spherify').value),Number(document.querySelector('#twist').value)));
   }
   if (entry.port.example === 14) {
    settings.hidden=false;settings.innerHTML='<label>PMREM <input id="pmrem" type="checkbox" checked></label>';
    settings.addEventListener('input',()=>app.gallery_pmrem(document.querySelector('#pmrem').checked));
   }
   if (entry.port.example === 13) {
    settings.hidden=false;settings.innerHTML='<label>Background intensity <input id="background-intensity" type="range" min="0" max="1" value="1" step="0.01"></label>';
    settings.addEventListener('input',()=>app.background_intensity(Number(document.querySelector('#background-intensity').value)));
   }
   if (entry.port.example === 11) {
    settings.hidden = false;
    const controls=[['offset.x',0,0,1],['offset.y',0,0,1],['repeat.x',0.25,0.25,2],['repeat.y',0.25,0.25,2],['rotation',Math.PI/4,-2,2],['center.x',0.5,0,1],['center.y',0.5,0,1]];
    settings.innerHTML=controls.map(([label,value,min,max],i)=>`<label>${label}<input id="uv-${i}" type="range" value="${value}" min="${min}" max="${max}" step="0.01"></label>`).join('');
    settings.addEventListener('input',()=>app.texture_transform([...settings.querySelectorAll('input')].map(e=>Number(e.value))));
   }
   if (entry.port.example === 3) {
    settings.hidden = false;
    settings.innerHTML = '<label>Animation <input id="animate" type="checkbox" checked></label><label>Deformation <input id="amount" type="range" min="0" max="3" step="0.1" value="1"></label><label>Speed <input id="speed" type="range" min="0" max="2" step="0.1" value="1"></label>';
    settings.addEventListener('input', () => app.point_lights_controls(!document.querySelector('#animate').checked, Number(document.querySelector('#amount').value), Number(document.querySelector('#speed').value)));
   }
   new MutationObserver(() => { if (canvas.dataset.error && !closing) {closing = true; release(); settings.hidden = true; explain(canvas.dataset.error); document.body.dataset.status = 'error';} }).observe(canvas, {attributes:true, attributeFilter:['data-error']});
  }
 } catch (error) { app?.free(); app = null; explain(String(error)); document.body.dataset.status = 'error'; }
}
