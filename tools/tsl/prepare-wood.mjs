// Static geometry/preset data only; all procedural shading executes in Rust/WGSL.
import {readFile,writeFile} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(s,c,next){if(s==='three'||s==='three/tsl'||s==='three/webgpu')return {url:new URL(s==='three'?'../../.cache/three-r186/src/Three.Core.js':s==='three/webgpu'?'../../.cache/three-r186/src/Three.WebGPU.js':'../../.cache/three-r186/src/Three.TSL.js',import.meta.url).href,shortCircuit:true};return next(s,c);}});
const {FontLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FontLoader.js');
const {TextGeometry}=await import('../../.cache/three-r186/examples/jsm/geometries/TextGeometry.js');
const {RoundedBoxGeometry}=await import('../../.cache/three-r186/examples/jsm/geometries/RoundedBoxGeometry.js');
const {GetWoodPreset,WoodGenuses,Finishes}=await import('../../.cache/three-r186/examples/jsm/materials/WoodNodeMaterial.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/tsl-procedural/',import.meta.url);
const fontBytes=await readFile(new URL('fonts/helvetiker_regular.typeface.json',root));const font=new FontLoader().parse(JSON.parse(fontBytes));
const labels=[...Finishes,...WoodGenuses,'custom'];
const geometries=[new RoundedBoxGeometry(.125,.9,.9,10,.02),...labels.map(text=>{const g=new TextGeometry(text,{font,size:.1,depth:.001,curveSegments:12,bevelEnabled:false});g.computeBoundingBox();const b=g.boundingBox;g.translate(-.5*(b.max.x-b.min.x),-.5*(b.max.y-b.min.y),-.5*(b.max.z-b.min.z));return g;})];
const chunks=[];const word=n=>{const b=Buffer.alloc(4);b.writeUInt32LE(n);chunks.push(b);};word(geometries.length);
for(const g of geometries){word(g.attributes.position.count);word(g.index?.count??0);for(const key of ['position','normal','uv']){const a=g.attributes[key].array;const b=Buffer.alloc(a.length*4);a.forEach((v,i)=>b.writeFloatLE(v,i*4));chunks.push(b);}if(g.index){const b=Buffer.alloc(g.index.count*4);g.index.array.forEach((v,i)=>b.writeUInt32LE(v,i*4));chunks.push(b);}}
const bytes=Buffer.concat(chunks);await writeFile(new URL('wood-geometry.bin',out),bytes);
const names=['centerSize','largeWarpScale','largeGrainStretch','smallWarpStrength','smallWarpScale','fineWarpStrength','fineWarpScale','ringThickness','ringBias','ringSizeVariance','ringVarianceScale','barkThickness','splotchScale','splotchIntensity','cellScale','cellSize'];
const presets=WoodGenuses.map(g=>{const p=GetWoodPreset(g,'raw');return {name:g,parameters:names.map(n=>p[n]),dark:parseInt(p.darkGrainColor.slice(1),16),light:parseInt(p.lightGrainColor.slice(1),16)};});
await writeFile(new URL('wood-presets.json',out),JSON.stringify(presets,null,2)+'\n');
const manifest=JSON.parse(await readFile(new URL('geometry.json',out))).filter(x=>!x.file.startsWith('wood-'));const hash=b=>createHash('sha256').update(b).digest('hex');
for(const file of ['wood-geometry.bin','wood-presets.json'])manifest.push({file,sha256:hash(await readFile(new URL(file,out))),source:'jsm/materials/WoodNodeMaterial.js',source_sha256:hash(await readFile(new URL('jsm/materials/WoodNodeMaterial.js',root)))});
await writeFile(new URL('geometry.json',out),JSON.stringify(manifest,null,2)+'\n');
