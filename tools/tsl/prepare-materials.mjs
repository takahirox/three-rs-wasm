// Exact static geometry conversion with the pinned official loaders, not rendered images.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {registerHooks} from 'node:module';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
const {FBXLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FBXLoader.js');
const {FontLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FontLoader.js');
const {TextGeometry}=await import('../../.cache/three-r186/examples/jsm/geometries/TextGeometry.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/tsl-materials/',import.meta.url);
await mkdir(out,{recursive:true});
const data=await readFile(new URL('models/fbx/stanford-bunny.fbx',root));
const bunny=new FBXLoader().parse(data.buffer.slice(data.byteOffset,data.byteOffset+data.byteLength),'');
function geometryFile(geometries){const chunks=[];const word=n=>{const b=Buffer.alloc(4);b.writeUInt32LE(n);chunks.push(b);};word(geometries.length);for(const g of geometries){word(g.attributes.position.count);word(g.index?.count??0);for(const k of ['position','normal','uv']){const b=Buffer.alloc(g.attributes[k].array.length*4);g.attributes[k].array.forEach((v,i)=>b.writeFloatLE(v,i*4));chunks.push(b);}if(g.index){const b=Buffer.alloc(g.index.count*4);g.index.array.forEach((v,i)=>b.writeUInt32LE(v,i*4));chunks.push(b);}}return Buffer.concat(chunks);}
await writeFile(new URL('bunny.bin',out),geometryFile([bunny.children[0].geometry]));
const font=new FontLoader().parse(JSON.parse(await readFile(new URL('fonts/gentilis_regular.typeface.json',root),'utf8')));
await writeFile(new URL('labels.bin',out),geometryFile(['-gradientMap','+gradientMap','-diffuse','+diffuse'].map(name=>new TextGeometry(name,{font,size:20,depth:1,curveSegments:1}))));
for(const p of ['models/fbx/bunny_thickness.jpg','models/gltf/ferrari.glb','models/gltf/ferrari_ao.png','textures/equirectangular/blouberg_sunrise_2_1k.hdr']){await mkdir(new URL(p.split('/').slice(0,-1).join('/')+'/',out),{recursive:true});await copyFile(new URL(p,root),new URL(p,out));}

const records=[];for(const [file,source] of [['bunny.bin','models/fbx/stanford-bunny.fbx'],['labels.bin','fonts/gentilis_regular.typeface.json']]){const hash=b=>createHash('sha256').update(b).digest('hex');records.push({file,source,sha256:hash(await readFile(new URL(file,out))),source_sha256:hash(await readFile(new URL(source,root)))});}await writeFile(new URL('geometry.json',out),JSON.stringify(records,null,2)+'\n');
