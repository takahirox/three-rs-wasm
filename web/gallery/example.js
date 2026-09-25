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
  const runtimeRevision = 'lights-probes-1';
  const {default:init, BrowserApp} = await import(`../pkg/three_rs_wasm.js?v=${runtimeRevision}`);
  await init({module_or_path:new URL(`../pkg/three_rs_wasm_bg.wasm?v=${runtimeRevision}`,import.meta.url)});
  const resize = () => { canvas.width = Math.max(1, Math.round(innerWidth * ([132,141].includes(entry.port.example)?1:devicePixelRatio))); canvas.height = Math.max(1, Math.round(innerHeight * ([132,141].includes(entry.port.example)?1:devicePixelRatio))); app?.request_render(); };
  if(entry.port.example===139){const {prepareAudio}=await import('./audio.js');await prepareAudio(loading);}
  resize(); canvas.hidden = false;
  if (entry.port.example === 191) {
   // The original's DOM drawing canvas; Rust draws into it and uploads its pixels.
   const drawing=document.createElement('canvas');drawing.id='drawing-canvas';drawing.width=drawing.height=128;
   // The gallery page stretches canvases; keep the original's 128 CSS pixels.
   drawing.style.cssText='position:absolute;background-color:#000000;top:0px;right:0px;z-index:3000;cursor:crosshair;touch-action:none;width:128px;height:128px';
   document.body.append(drawing);
  }
  if (entry.port.example === 266) {
   // The original's hidden looping video; the Rust scene uploads its frames.
   const video=document.createElement('video');video.id='video';Object.assign(video,{loop:true,muted:true,playsInline:true,crossOrigin:'anonymous'});video.style.display='none';
   for(const name of ['pano.webm','pano.mp4']){const source=document.createElement('source');source.src=`assets/${name}`;video.append(source);}
   document.body.append(video);video.play().catch(()=>{});
  }
  app = await BrowserApp.create(canvas, entry.port.example, !new URLSearchParams(location.search).has('still'));
  if (closing) { app.free(); app = null; } else {
   window.app = app;
   if(entry.port.example===139){const {installAudio}=await import('./audio.js');installAudio(app,e=>{canvas.dataset.error=String(e);});}
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
   if(entry.port.example===135)addEventListener('gallery-readback-ready',()=>app?.request_render());
   if([92,93].includes(entry.port.example))installElements(canvas,app,entry.port.example===93);
   if(entry.port.example===97){for(const [i,label] of ['DataArrayTexture','Data3DTexture','RenderTargetArray','RenderTarget3D'].entries()){const item=text('div',label,document.body);item.className='viewport-label';item.style.cssText=`position:absolute;bottom:${i<2?2:52}%;left:${i%2?52:2}%;color:white;background:rgba(0,0,0,.7);padding:5px 10px;border-radius:4px;font-family:monospace;pointer-events:none;user-select:none`;}}
   if ([16,18,20,25,28,29,30,31,32,33,34,41,48,51,53,56,58,59,60,62,63,64,66,67,68,69,70,71,72,73,75,76,77,81,82,83,87,88,89,90,91,94,95,96,97,98,99,101,102,103,104,105,106,107,108,109,110,111,112,113,114,115,116,117,118,119,121,122,125,126,127,128,129,130,132,133,134,135,136,137,138,140,141,142,143,144,146,147,148,149,154,156,157,172,173,175,176,179,180,181,188,190,192,195,196,199,202,206,208,209,212,219,220,221,222,224,226,228,229,232,235,236,240,243,246,248,249,258,259,260,261,262,263,267,268,270,271,272].includes(entry.port.example)) { installOrbit(canvas,app,{touchRotate:entry.port.example!==56}); } else if (entry.port.example < 35) {
   let drag;
   canvas.addEventListener('pointerdown', event => { drag = [event.clientX, event.clientY]; canvas.setPointerCapture(event.pointerId); app.gallery_input(0,0,0,true); });
   canvas.addEventListener('pointermove', event => { if(event.isPrimary===false)return;app.gallery_pointer(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2); if (drag) { app.gallery_input(event.clientX-drag[0],event.clientY-drag[1],0,true); app.orbit(event.clientX-drag[0], event.clientY-drag[1], 0); drag = [event.clientX,event.clientY]; } });
   for (const event of ['pointerup','pointercancel','lostpointercapture']) canvas.addEventListener(event, () => { drag = null; app?.gallery_input(0,0,0,false); });
   canvas.addEventListener('wheel', event => { event.preventDefault(); app.gallery_input(0,0,event.deltaY,false); app.orbit(0,0,event.deltaY); }, {passive:false});
   }
   if (entry.port.example === 199) canvas.addEventListener('click',event=>app.gallery_select(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2));
   if ([73,111,193].includes(entry.port.example)) canvas.addEventListener('pointerdown',event=>app.gallery_select(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2));
   if (entry.port.example===155) {
    let pointer=null;canvas.style.touchAction='none';
    canvas.addEventListener('pointerdown',e=>{if(e.isPrimary===false)return;pointer={id:e.pointerId,x:e.clientX};canvas.setPointerCapture(e.pointerId);});
    canvas.addEventListener('pointermove',e=>{if(pointer?.id!==e.pointerId)return;app.gallery_input(e.clientX-pointer.x,0,0,false);pointer.x=e.clientX;});
    for(const type of ['pointerup','pointercancel','lostpointercapture'])canvas.addEventListener(type,()=>pointer=null);
   }
   if ([75,134,230,238].includes(entry.port.example)) window.addEventListener('pointermove',event=>app.gallery_pointer(event.clientX/canvas.clientWidth*2-1,1-event.clientY/canvas.clientHeight*2));
   if ([39,50,55,56,150,151,163,164,165,174,178,185,186,187,188,192,193,194,198,203,204,205,209,219].includes(entry.port.example)) canvas.addEventListener('pointermove',event=>app.gallery_pointer(event.offsetX/canvas.clientWidth*2-1,1-event.offsetY/canvas.clientHeight*2));
   if ([46,50].includes(entry.port.example)) document.body.style.background = '#000';
   if ([150,151,174].includes(entry.port.example)) {
    for (let side=0;side<2;side++) {
     const label=text('div','',document.body);label.className='texture-label';
     label.style.cssText=`color:white;font:bold 16px Arial;position:absolute;bottom:0;${side?'right':'left'}:0;padding:1em;background:#000d;text-shadow:1px 1px #000;pointer-events:none`;
     label.innerHTML=entry.port.example===151?`anisotropy: ${side?1:16}`:`Floor${side?'':' (128x128)'}<br>mag: ${side?'Nearest':'Linear'}<br>min: ${side?(entry.port.example===174?'Nearest':'NearestMipmapNearestFilter'):'LinearMipmapLinear'}<br><br>Painting${side?'':' (748x600)'}<br>mag: ${side?'Nearest':'Linear'}<br>min: ${side?'Nearest':'Linear'}`;
    }
   }
   const settings = document.querySelector('#settings');
   if (entry.port.example >= 48) {
    const controls={268:[],269:[],270:[],271:[["light probe",0,1,1,0.02],["directional light",0,1,0.6,0.02],["envMap",0,1,1,0.02]],272:[["type",["None","Linear","Reinhard","Cineon","ACESFilmic","AgX","Neutral"],6],["exposure",0,2,1,0.01],["blurriness",0,1,0.3,0.01],["intensity",0,1,1,0.01]],263:[["turbidity",0,20,10,0.1],["rayleigh",0,4,3,0.001],["mieCoefficient",0,0.1,0.005,0.001],["mieDirectionalG",0,1,0.7,0.001],["elevation",0,90,65,0.1],["azimuth",-180,180,0,0.1],["exposure",0,1,0.05,0.0001],["showSunDisc",true],["coverage",0,1,0.4,0.01],["density",0,1,0.4,0.01],["cloud elevation",0,1,0.5,0.01]],264:[["show cascades",false],["shadow far",100,3000,1000,1],["shadow resolution",["256","512","1024","2048","4096"],3],["azimuth",0,360,135,0.1],["elevation",5,80,10,0.1]],265:[],266:[],267:[["elevation",0,90,2,0.1],["azimuth",-180,180,180,0.1],["exposure",0,1,0.1,0.0001],["distortionScale",0,8,3.7,0.1],["size",0.1,10,1,0.1],["strength",0,3,0.1,0.01],["radius",0,1,0,0.01],["coverage",0,1,0.4,0.01],["density",0,1,0.5,0.01],["elevation",0,1,0.5,0.01]],258:[["Export STL (ASCII)",null],["Export STL (Binary)",null]],259:[["Export PLY (ASCII)",null],["Export PLY (Binary BE)",null],["Export PLY (Binary LE)",null]],260:[["Triangle",null],["Cube",null],["Cylinder",null],["Multiple objects",null],["Transformed objects",null],["Point Cloud",null],["Export OBJ",null]],261:[["color","#ffffff"],["exposure",0,2,1,0.01]],262:[["hemiIrradiance",["0.0001 lx (Moonless Night)","0.002 lx (Night Airglow)","0.5 lx (Full Moon)","3.4 lx (City Twilight)","50 lx (Living Room)","100 lx (Very Overcast)","350 lx (Office Room)","400 lx (Sunrise/Sunset)","1000 lx (Overcast)","18000 lx (Daylight)","50000 lx (Direct Sun)"],0],["bulbPower",["110000 lm (1000W)","3500 lm (300W)","1700 lm (100W)","800 lm (60W)","400 lm (40W)","180 lm (25W)","20 lm (4W)","Off"],4],["exposure",0,1,0.68,0.01],["shadows",true]],253:[],254:[],255:[],256:[],257:[],248:[["Local Enabled",true],["Local Shadows",true],["Local Visualize",false],["Global Enabled",false]],249:[["spline",["GrannyKnot","HeartCurve","VivianiCurve","KnotCurve","HelixCurve","TrefoilKnot","TorusKnot","CinquefoilKnot","TrefoilPolynomialKnot","FigureEightPolynomialKnot","DecoratedTorusKnot4a","DecoratedTorusKnot4b","DecoratedTorusKnot5a","DecoratedTorusKnot5c","PipeSpline","SampleClosedSpline"],0],["scale",2,10,4,2],["extrusionSegments",50,500,100,50],["radiusSegments",2,12,3,1],["closed",true],["animationView",false],["lookAhead",false],["cameraHelper",false]],250:[["change color",null],["change font",null],["change weight",null],["change bevel",null]],251:[],252:[],243:[],244:[],245:[],246:[],247:[["toggle hemisphere light",true],["toggle directional light",true]],238:[],239:[],240:[],241:[],242:[["exposure",0,4,2,0.01]],233:[["molecule",["Ethanol","Aspirin","Caffeine","Nicotine","LSD","Cocaine","Cholesterol","Lycopene","Glucose","Aluminium oxide","Cubane","Copper","Fluorite","Salt","YBCO superconductor","Buckyball","Graphite"],2]],234:[],235:[["ratio",0.01,1,0.125,0.01]],236:[],237:[],228:[],229:[],230:[],231:[],232:[["method",["INSTANCED","MERGED","NAIVE"],0],["count",1,10000,1000,1]],223:[],224:[["showMap",false],["smoothShading",true],["edgeSplit",true],["cutOffAngle",0,180,20,1],["tryKeepNormals",true]],225:[],226:[["Tessellation Level",["2","3","4","5","6","8","10","15","20","30","40","50"],7],["display lid",true],["display body",true],["display bottom",true],["snug lid",false],["original scale",false],["Shading",["wireframe","flat","smooth","glossy","textured","reflective"],3]],227:[["count",0,2000,2000,1],["distribution",["random","weighted"],0]],218:[],219:[],220:[["asset",["benchy","test_m82","test_m83"],0]],221:[],222:[],213:[["exposure",0,4,2,.01]],214:[],215:[["use orthographic",false],["multi touch roll",false]],216:[],217:[],208:[],209:[["zoomToCursor",false],["screenSpacePanning",false]],210:[],211:[],212:[["showDots",true],["showLines",true],["minDistance",10,300,150,.01],["limitConnections",false],["maxConnections",0,30,20,1],["particleCount",0,1000,500,1]],203:[],204:[],205:[],206:[["size",0.001,0.01,0.005,0.0001],["color","#ffffff"],["type",["ascii/simple.pcd","binary/Zaghetto.pcd","binary/Zaghetto_8bit.pcd","binary_compressed/pcl_logo.pcd"],1]],207:[],198:[],199:[],200:[],201:[],202:[],195:[["alphaToCoverage",true],["clip intersection",true],["plane constant",-1,1,0,.01],["show helpers",false]],197:[["blendEquation",["Add","Subtract","ReverseSubtract","Min","Max"],0]],193:[],194:[],196:[],188:[["count",0,1000,1000,1]],189:[["useLookAt",false]],190:[],191:[],192:[],183:[],184:[["procedure",["noiseRandom1D","noiseRandom2D","noiseRandom3D"],2]],185:[],186:[],187:[],178:[["color","#ffffff"],["mapping",["ReflectionMapping","RefractionMapping"],0],["refractionRatio",0,1,.98,.01],["transparent",false],["opacity",0,1,1,.01]],179:[["Type",["Cube","Equirectangular"],0],["Refraction",false],["backgroundRotationX",false],["backgroundRotationY",false],["backgroundRotationZ",false],["syncMaterial",false]],180:[["metalness",0,1,1,.01],["roughness",0,1,.4,.01],["aoMapIntensity",0,1,1,.01],["ambientIntensity",0,1,.2,.01],["envMapIntensity",0,3,1,.01],["displacementScale",0,3,2.436143,.001],["normalScale",-1,1,1,.01]],181:[["enable bump map",true],["bump scale",0,40,10,.1]],182:[],173:[["thickness",0,4,1,.1]],174:[],175:[],176:[["color map",["rainbow","cooltowarm","blackbody","grayscale"],0]],177:[],168:[],169:[],170:[["instance count",0,50000,50000,1]],171:[],172:[],163:[["size attenuation",true]],164:[["texture",true]],165:[],166:[],167:[],158:[],159:[],160:[],161:[],162:[],153:[["Tint for Visibility",false]],154:[],155:[],156:[],157:[],148:[],149:[["alphaToCoverage",true],["local enabled",true],["clip shadows",false],["intersection",true],["plane",.3,1.25,.8,.01],["global enabled",true],["global plane",-.4,3,.1,.01]],150:[],151:[],152:[],146:[["upscaleMethod",["Bilinear","TAAU"],1],["resolutionScale",.25,1,.5,.25],["sharpening",true],["sharpness",0,2,.2,.05]],147:[["autoRotate",true],["blur amount",0,3,1,.01],["speed",0,2,1,.01]],145:[],144:[["enabled",true],["amount",0,1,1,.01],["flash light",0,30,5,.1]],143:[["position",-50,50,0,.001],["scale",.1,3.5,3.5,.01],["drop count",200,50000,25000,1]],142:[],141:[["Pixel Size",1,16,6,1],["Normal Edge Strength",0,2,.3,.05],["Depth Edge Strength",0,1,.4,.05],["pixelAlignedPanning",true]],140:[["roughness",0,1,.9,.01],["radius",0,1,.2,.01],["resolution scale",.25,1,.5,.01]],139:[["pitch",.5,2,1.5,.01],["delayVolume",0,1,.2,.01],["delayOffset",.1,1,.55,.01]],138:[],137:[],136:[],135:[["selection",["mrt","diffuse","normal"],0]],134:[["noiseIterations",0,10,3,1],["positionFrequency",0,1,.175,.001],["strength",0,20,10,.001],["warpFrequency",0,20,6,.001],["warpStrength",0,2,1,.001],["colorSand","#ffe894"],["colorGrass","#85d534"],["colorSnow","#ffffff"],["colorRock","#bfbd8d"],["roughness",0,1,.5,.01],["ior",1,2,1.333,.001],["color","#4db2ff"]],133:[["sliceStart",-Math.PI,Math.PI,1.75,.001],["sliceArc",0,Math.PI*2,1.25,.001],["sliceColor","#b62f58"]],132:[["centerSize", 0, 2, 1.11, 0.01], ["largeWarpScale", 0, 1, 0.32, 0.001], ["largeGrainStretch", 0, 1, 0.24, 0.001], ["smallWarpStrength", 0, 0.2, 0.059, 0.001], ["smallWarpScale", 0, 5, 2, 0.01], ["fineWarpStrength", 0, 0.05, 0.006, 0.001], ["fineWarpScale", 0, 50, 32.8, 0.1], ["ringThickness", 0, 0.1, 0.029411764705882353, 0.001], ["ringBias", -0.2, 0.2, 0.03, 0.001], ["ringSizeVariance", 0, 0.2, 0.03, 0.001], ["ringVarianceScale", 0, 10, 4.4, 0.1], ["barkThickness", 0, 1, 0.3, 0.01], ["splotchScale", 0, 1, 0.2, 0.01], ["splotchIntensity", 0, 1, 0.541, 0.01], ["cellScale", 100, 2000, 910, 1], ["cellSize", 0.01, 0.5, 0.1, 0.001], ["darkGrainColor", "#0c0504"], ["lightGrainColor", "#926c50"], ["clearcoat", 0, 1, 1, 0.01], ["clearcoatRoughness", 0, 1, 0.2, 0.01]],131:[],130:[],129:[],128:[],123:[],124:[],125:[["shadowBlur",0,15,3.5,.1],["shadowDarkness",0,10,1,.1],["shadowOpacity",0,1,1,.01],["planeColor","#ffffff"],["planeOpacity",0,1,1,.01],["showWireframe",false]],126:[["line type",["LineGeometry","line-strip"],0],["world units",false],["width (pixels)",1,10,5,.1],["alphaToCoverage",false],["dashed",false],["dash scale",.5,2,1,.1],["dash offset",0,5,0,.1],["dash / gap",["2 : 1","1 : 1","1 : 2"],1]],127:[["line type",["LineGeometry","line-list"],0],["width (px)",1,10,5,.1],["dashed",false],["dash scale",.5,1,1,.1],["dash / gap",["2 : 1","1 : 1","1 : 2"],1]],120:[],121:[["oit",true],["opacity",0,1,.5,.01]],122:[["Grounded",true]],118:[["distortion",.01,1,.1,.01],["ambient",.01,5,.4,.01],["attenuation",.01,5,.8,.01],["power",.01,16,2,.01],["scale",.01,50,16,.01]],119:[],113:[],114:[["box scale x",.1,2,1,.01],["box scale y",.1,2,1,.01],["material",["blurred","checker","depth","pixel"],1]],115:[],116:[["mode",["hard","soft"],1],["soft distance",.1,2,1,.01],["soft contrast",1,6,2,.01]],117:[["upscaleMethod",["Bilinear","FSR1"],1],["resolutionScale",.25,1,.5,.25]],108:[],109:[["roughness",0,1,.5,.001]],110:[["Light Map Intensity",0,1,1,.01]],111:[["min distance",0,3,1,.01],["max distance",0,5,3,.01],["blur size",1,3,2,1],["blur spread",1,7,4,1]],112:[["strength",0,2,1,.01],["radius",0,1,1,.01],["threshold",0,1,.5,.01],["attenuation",10,50,25,.1],["spacing",0,.3,.25,.01],["exposure",.1,2,1,.01]],103:[],104:[["mix",-1,2,0,.01],["blurBackground",0,1,0,.01],["offsetHDR1",0,Math.PI*2,0,.01],["offsetHDR2",0,Math.PI*2,0,.01],["procedural",0,1,0,.01],["intensity",0,5,1,.01],["hue",0,Math.PI*2,0,.01],["saturation",0,2,1,.01]],105:[["box projected",true],["roughness",0,1,.25,.01]],106:[["alpha",0,1,.5,.01],["alphaHash",true],["sampleLevel",0,4,3,1]],107:[["enabled",true],["Strength",0,3,1.5,.01],["Center X",-1,1,.5,.01],["Center Y",-1,1,.5,.01],["Scale",.5,2,1.2,.01],["animated",true],["autoRotate",true]],98:[],99:[["enabled",true]],100:[["enabled",true],["autoRotate",true]],101:[["lut",["Bourbon 64.CUBE","Chemical 168.CUBE","Clayton 33.CUBE","Cubicle 99.CUBE","Remy 24.CUBE","Presetpro-Cinematic.3dl","NeutralLUT","B&WLUT","NightLUT"],0],["intensity",0,1,1,.01]],102:[["Background Blurriness",0,1,.4,.01],["Parallax Scale",.2,.5,.5,.01],["UV Scale",1,5,3,.01]],96:[],97:[],95:[['alphaToCoverage',true],['minWidth',1,30,6,1],['maxWidth',2,30,20,1],['pulseSpeed',1,20,6,.1]],94:[],93:[],92:[],91:[['focus distance',10,3000,500,1],['focal length',50,750,200,1],['bokeh scale',1,20,10,.1]],90:[],89:[],88:[['atmosphereDayColor','#4db2ff'],['atmosphereTwilightColor','#bc490b'],['roughnessLow',0,1,.25,.001],['roughnessHigh',0,1,.35,.001]],87:[['intensity',0,10,5,.01],['threshold',0,.9,.3,.01],['samples',2,128,80,1],['tint color','#7a8aff'],['bloom radius',0,1,0,.01],['time scale',0,1,.5,.01]],86:[],85:[['sampling',['normal','centroid','sample','flat first','flat either'],0]],84:[],83:[['threshold',0,1,.08,.01],['opacity',0,1,.08,.01],['range',0,1,.1,.01],['steps',0,200,100,1]],81:[['threshold',0,1,.6,.01],['steps',0,300,200,1],['refine',true]],82:[['threshold',0,1,.25,.01],['opacity',0,1,.25,.01],['range',0,1,.1,.01],['steps',0,200,100,1]],80:[],78:[['multisampling',true],['animated',true]],79:[['Red',true],['Yellow',true],['Green',true]],48:[['Density',.001,.1,.04,.0001],['Height',-5,5,2,.01]],49:[],50:[['size attenuation',true]],51:[['size',0,1,.08,.001],['color inside','#ffa575'],['color outside','#311599']],52:[['damp',.25,1,.8,.01],['enabled',true]],53:[['speed',0,1,.2,.01]],54:[['instance count',1,1000,1000,1]],55:[['limit x',0,1,1,.01],['limit y',0,1,1,.01]],56:[['gravity',-.0098,0,-.00098,.0001],['bounce',.1,1,.8,.01],['friction',.96,.99,.99,.01],['size',.12,.5,.12,.01]],57:[],58:[],61:[],62:[],63:[],64:[],65:[],66:[],67:[],68:[['roughness',0,1,.5,.01],['metalness',0,1,.5,.01]],69:[],73:[['threshold',0,1,0,.01],['strength',0,3,1,.01],['radius',0,1,0,.01],['exposure',.1,3,1,.01]],75:[['elasticity',0,.5,.4,.01],['damping',.9,.98,.94,.001],['brush size',.1,.5,.25,.01],['brush strength',.1,.3,.22,.01]],76:[['emissiveColor','#ff8b4d'],['timeScale',-1,1,.2,.01],['parabolStrength',0,2,1,.01],['parabolOffset',0,1,.3,.01],['parabolAmplitude',0,2,.2,.01],['strength',0,10,1,.01],['radius',0,1,.1,.01]],77:[],74:[],70:[],71:[['threshold',0,1,0,.01],['strength',0,3,1,.01],['radius',0,1,0,.01],['exposure',.1,2,1,.01]],72:[['strength',0,5,2.5,.01],['radius',0,1,.5,.01],['exposure',.1,2,1,.01]],60:[["ambient", 0, 10, 3, 0.001], ["directional", 0, 20, 8, 0.001], ["color", "#fb00ff"], ["count", 1, 200, 140, 1], ["x", -1, 1, -0.4, 0.01], ["y", -1, 1, -1, 0.01], ["z", -1, 1, 0.5, 0.01], ["start", -1, 1, 1, 0.01], ["end", -1, 1, 0, 0.01], ["mix low", 0, 1, 0, 0.01], ["mix high", 0, 1, 0.5, 0.01], ["radius", 0, 1, 0.8, 0.01], ["color", "#94ffd1"], ["count", 1, 200, 180, 1], ["x", -1, 1, 0.5, 0.01], ["y", -1, 1, 0.5, 0.01], ["z", -1, 1, -0.2, 0.01], ["start", -1, 1, 0.55, 0.01], ["end", -1, 1, 0.2, 0.01], ["mix low", 0, 1, 0.5, 0.01], ["mix high", 0, 1, 1, 0.01], ["radius", 0, 1, 0.8, 0.01], ["default color", "#ff622e"]],59:[['color','#271442'],['roughness',0,1,.15,.001],['emissive color','#ff0a81'],['low',-1,0,-.25,.001],['high',0,1,.2,.001],['power',1,10,7,1],['large speed',0,5,1.25,.01],['large multiplier',0,1,.15,.01],['frequency x',0,10,3,.01],['frequency y',0,10,1,.01],['small iterations',0,5,3,1],['small frequency',0,10,2,.01],['small speed',0,1,.3,.01],['small multiplier',0,1,.18,.01],['normal shift',0,.1,.01,.0001]]}[entry.port.example];
    settings.hidden=!controls.length;
    controls.forEach(([name,min,max,value,step],i)=>{if(min===null){const button=text('button',name,settings);button.id='particle-'+i;button.addEventListener('click',()=>{app.tsl_parameter(i,1);const file=app.gallery_take_export?.();if(file){const link=document.createElement('a');link.href=URL.createObjectURL(new Blob([file[1]]));link.download=file[0];link.click();}});return;}const label=text('label',name+' ',settings),input=document.createElement(Array.isArray(min)?'select':'input');input.id='particle-'+i;
     if(Array.isArray(min)){min.forEach((name,index)=>input.add(new Option(name,index)));input.value=max;}else{
     input.type=typeof min==='boolean'?'checkbox':typeof min==='string'?'color':'range';if(input.type==='checkbox')input.checked=min;else if(input.type==='color')input.value=min;else Object.assign(input,{min,max,value,step});
     }label.append(input);input.addEventListener('input',()=>app.tsl_parameter(i,input.type==='checkbox'?Number(input.checked):input.type==='color'?parseInt(input.value.slice(1),16):Number(input.value)));
    });
   }
   if ([214,215,217,218,225,231,233,245,250,251,255,256,264,266,269].includes(entry.port.example)) {
    // Absolute CSS-pixel pointer events for controls that read positions, not deltas.
    canvas.style.touchAction='none';canvas.addEventListener('contextmenu',event=>event.preventDefault());
    canvas.addEventListener('pointerdown',event=>{if(event.isPrimary===false)return;canvas.setPointerCapture(event.pointerId);app.gallery_draw(10+event.button,event.clientX,event.clientY);});
    canvas.addEventListener('pointermove',event=>{if(event.isPrimary===false)return;app.gallery_draw(0,event.clientX,event.clientY);});
    canvas.addEventListener('pointerup',event=>{if(event.isPrimary===false)return;app.gallery_draw(20+event.button,event.clientX,event.clientY);});
    if(![217,218].includes(entry.port.example))canvas.addEventListener('wheel',event=>{event.preventDefault();app.gallery_input(0,0,event.deltaY,false);},{passive:false});
   }
   if (entry.port.example === 250) {
    // keydown (backspace, first-letter reset) and keypress characters, as the original's document listeners.
    document.addEventListener('keydown',event=>{if(event.keyCode===8)event.preventDefault();app.gallery_key(event.keyCode,true);});
    document.addEventListener('keypress',event=>{if(event.which!==8)app.gallery_key(100000+event.which,true);});
   }
   if (entry.port.example === 256) {
    // SelectionHelper: the CSS rectangle that follows the pointer while it is down.
    const box=document.createElement('div');box.id='selectBox';box.style.cssText='border:1px solid #55aaff;background-color:rgba(75,160,255,0.3);position:fixed;pointer-events:none';let start=null;
    canvas.addEventListener('pointerdown',event=>{start=[event.clientX,event.clientY];box.style.display='none';Object.assign(box.style,{left:start[0]+'px',top:start[1]+'px',width:'0px',height:'0px'});document.body.append(box);});
    canvas.addEventListener('pointermove',event=>{if(!start)return;box.style.display='block';const [l,t]=[Math.min(start[0],event.clientX),Math.min(start[1],event.clientY)];Object.assign(box.style,{left:l+'px',top:t+'px',width:Math.max(start[0],event.clientX)-l+'px',height:Math.max(start[1],event.clientY)-t+'px'});});
    canvas.addEventListener('pointerup',()=>{start=null;box.remove();});
   }
   if (entry.port.example === 265) {
    // PointerLockControls: the blocker locks the pointer; movement deltas turn the view while locked.
    const blocker=document.createElement('div');blocker.id='blocker';blocker.style.cssText='position:absolute;inset:0;background:rgba(0,0,0,0.5);display:flex;flex-direction:column;justify-content:center;align-items:center;text-align:center;color:#fff;font-size:14px;cursor:pointer';
    blocker.innerHTML='<p style="font-size:36px">Click to play</p><p>Move: WASD<br/>Jump: SPACE<br/>Look: MOUSE</p>';document.body.append(blocker);
    blocker.addEventListener('click',()=>canvas.requestPointerLock?.());
    document.addEventListener('pointerlockchange',()=>{const locked=document.pointerLockElement===canvas;blocker.style.display=locked?'none':'flex';app.tsl_parameter(0,locked?1:0);});
    document.addEventListener('mousemove',event=>app.gallery_input(event.movementX,event.movementY,0,false));
   }
   if ([193,210,214,215,217,218,225,231,233,245,251,255,264,265,269].includes(entry.port.example)) for(const [type,down] of [['keydown',true],['keyup',false]])document.addEventListener(type,event=>app.gallery_key(event.keyCode,down));
   if (entry.port.example === 229) {
    // The original's centred 128px selection frame; the copied region lies inside it.
    const selection=document.createElement('div');selection.id='selection';
    selection.style.cssText='position:fixed;display:flex;flex-direction:column;justify-content:center;align-items:center;height:100%;width:100%;top:0;z-index:999;pointer-events:none';
    const frame=document.createElement('div');frame.style.cssText='height:128px;width:128px;border:1px solid white';selection.append(frame);document.body.append(selection);
   }
   if (entry.port.example === 230) {
    const values=document.createElement('span');values.id='values';
    values.style.cssText='position:absolute;top:40px;width:100%;text-align:center;color:white;font:13px monospace;pointer-events:none';document.body.append(values);
   }
   if (entry.port.example === 196) {
    // The original's comparison slider; dragging it disables the controls, as there.
    const slider=document.createElement('div');slider.className='slider';
    slider.style.cssText='position:absolute;cursor:ew-resize;width:40px;height:40px;background-color:#F32196;opacity:0.7;border-radius:50%;top:calc(50% - 20px);left:calc(50% - 20px);touch-action:none';
    document.body.append(slider);
    const move=event=>{if(event.isPrimary===false)return;const x=Math.max(0,Math.min(innerWidth,event.pageX));slider.style.left=x-slider.offsetWidth/2+'px';app.gallery_slider(x);};
    const up=()=>{removeEventListener('pointermove',move);removeEventListener('pointerup',up);};
    slider.addEventListener('pointerdown',event=>{if(event.isPrimary===false)return;addEventListener('pointermove',move);addEventListener('pointerup',up);});
   }
   if (entry.port.example === 191) {
    const drawing=document.getElementById('drawing-canvas');
    for(const [kind,type] of [[0,'pointerdown'],[1,'pointermove'],[2,'pointerup'],[2,'pointerleave']])drawing.addEventListener(type,event=>app.gallery_draw(kind,event.offsetX,event.offsetY));
   }
   if (entry.port.example === 162) {
    settings.hidden=false;
    const status=text('div',app.gallery_status(),settings);status.id='line-count';
    for(const [i,name] of ['CULL SOME LINES','SHOW ALL LINES'].entries()){
     const button=text('button',name,settings);button.id=i?'showAllLines':'hideLines';button.addEventListener('click',()=>{app.tsl_parameter(i,1);status.textContent=app.gallery_status();});
    }
   }
   if (entry.port.example === 126) {
    const world=document.getElementById('particle-1'),width=document.getElementById('particle-2');
    world.addEventListener('input',()=>{Object.assign(width,world.checked?{min:.1,max:.5,value:.5}:{min:1,max:10,value:10});width.parentElement.firstChild.textContent=world.checked?'width (world units) ':'width (pixels) ';});
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
