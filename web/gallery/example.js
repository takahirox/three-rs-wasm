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
  const {default:init, BrowserApp} = await import('../pkg/three_rs_wasm.js'); await init();
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
   if ([30,31,32].includes(entry.port.example)) {
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
   if ([16,18,20,25,28,29,30,31,32].includes(entry.port.example)) { installOrbit(canvas,app); } else {
   let drag;
   canvas.addEventListener('pointerdown', event => { drag = [event.clientX, event.clientY]; canvas.setPointerCapture(event.pointerId); app.gallery_input(0,0,0,true); });
   canvas.addEventListener('pointermove', event => { if(event.isPrimary===false)return;app.gallery_pointer(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2); if (drag) { app.gallery_input(event.clientX-drag[0],event.clientY-drag[1],0,true); app.orbit(event.clientX-drag[0], event.clientY-drag[1], 0); drag = [event.clientX,event.clientY]; } });
   for (const event of ['pointerup','pointercancel','lostpointercapture']) canvas.addEventListener(event, () => { drag = null; app?.gallery_input(0,0,0,false); });
   canvas.addEventListener('wheel', event => { event.preventDefault(); app.gallery_input(0,0,event.deltaY,false); app.orbit(0,0,event.deltaY); }, {passive:false});
   }
   const settings = document.querySelector('#settings');
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
