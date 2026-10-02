// Bake webgl_loader_3dm's Rhino_Logo.3dm: the pinned Rhino3dmLoader ( r186, MIT ) run
// as the page runs it, its worker decoding the file with rhino3dm 8.32.1 ( MIT, the
// version the page loads from jsDelivr; pass its directory ) and _createGeometry building
// the object tree, stored as the loader leaves it. Nodes keep their type, TRS, visibility
// and layer index; meshes, lines and points their attributes and material; text dots the
// canvas parameters the loader draws them with ( the port draws them on a canvas as the
// loader does ); lights their parameters.
// usage: node tools/tsl/prepare-3dm-loader.mjs <rhino3dm directory>
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {createRequire,registerHooks} from 'node:module';
import vm from 'node:vm';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
const library=process.argv[2];
const root=new URL('../../',import.meta.url);
const source=await readFile(new URL('.cache/three-r186/examples/models/3dm/Rhino_Logo.3dm',root));
// The worker, run in a context with the rhino3dm module.
const loaderSource=await readFile(new URL('.cache/three-r186/examples/jsm/loaders/3DMLoader.js',root),'utf8');
const worker=loaderSource.slice(loaderSource.indexOf('function Rhino3dmWorker() {'),loaderSource.indexOf('export { Rhino3dmLoader }'));
const rhino3dm=createRequire(import.meta.url)(`${library}/rhino3dm.js`);
let message;
const context=vm.createContext({rhino3dm,self:{postMessage:m=>{message=m;}},console,Promise});
vm.runInContext(`${worker}\nRhino3dmWorker();`,context);
const {Rhino3dmLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/3DMLoader.js');
const loader=new Rhino3dmLoader();
context.onmessage({data:{type:'init',libraryConfig:{wasmBinary:await readFile(`${library}/rhino3dm.wasm`)}}});
// The loader's default subdivision level ( 3 ) for the SubD meshes.
context.onmessage({data:{type:'decode',id:0,buffer:source.buffer.slice(source.byteOffset,source.byteOffset+source.length),subdivisionLevel:loader.subdivisionLevel}});
while(!message)await new Promise(r=>setTimeout(r,20));
if(message.type!=='decode')throw Error(message.error.message);
// The main thread's _createGeometry; text dots record their canvas parameters.
globalThis.window={devicePixelRatio:1};
const dots=[];
globalThis.document={createElement:()=>{const canvas={style:{},getContext:()=>({canvas,measureText:()=>({width:0}),setTransform(){},fillRect(){},fillText(text){dots.push(text);}})};return canvas;}};
const THREE=await import('three');
THREE.Object3D.DEFAULT_UP.set(0,0,1);
loader.url='models/3dm/Rhino_Logo.3dm';
const object=loader._createGeometry(message.data);
object.updateMatrixWorld(true);
const chunks=[];let offset=0;const blobs=new Map();
const blob=array=>{if(blobs.has(array))return blobs.get(array);const bytes=Buffer.from(array.buffer,array.byteOffset,array.byteLength);const at=offset;chunks.push(bytes);offset+=bytes.length;const pad=(4-offset%4)%4;if(pad){chunks.push(Buffer.alloc(pad));offset+=pad;}const b={offset:at,length:array.length,type:array.constructor.name};blobs.set(array,b);return b;};
const materials=[];const materialIndex=new Map();
const material=m=>{
 if(materialIndex.has(m))return materialIndex.get(m);
 if(m.map||m.bumpMap||m.alphaMap||m.envMap)throw Error('textured 3dm material');
 const keys=['type','name','opacity','transparent','side','vertexColors','flatShading','depthTest','depthWrite','metalness','roughness','ior','reflectivity','clearcoat','clearcoatRoughness','sheen','specularIntensity','thickness','anisotropy','size','sizeAttenuation'];
 const entry=Object.fromEntries(keys.filter(k=>m[k]!==undefined).map(k=>[k,m[k]]));
 for(const k of ['color','emissive','specularColor','sheenColor'])if(m[k])entry[k]=m[k].toArray();
 materials.push(entry);materialIndex.set(m,materials.length-1);return materials.length-1;
};
const nodes=[];const nodeIndex=new Map();
object.traverse(o=>{nodeIndex.set(o,nodes.length);nodes.push(o);});
let dot=0;
const json={source:'examples/models/3dm/Rhino_Logo.3dm',source_sha256:createHash('sha256').update(source).digest('hex'),rhino3dm:'8.32.1',layers:message.data.layers.map(l=>({name:l.name,visible:l.visible})),nodes:[]};
for(const o of nodes){
 const n={type:o.type,name:o.name,parent:o.parent?nodeIndex.get(o.parent)??null:null,position:o.position.toArray(),quaternion:o.quaternion.toArray(),scale:o.scale.toArray(),visible:o.visible,layerIndex:o.userData.attributes?.layerIndex??null,objectType:o.userData.objectType??null};
 if(o.geometry){const g=o.geometry;if(g.index)n.index=blob(g.index.array);n.attributes=Object.fromEntries(Object.entries(g.attributes).map(([k,a])=>[k,{...blob(a.array),itemSize:a.itemSize,normalized:a.normalized}]));}
 if(o.material&&!o.isSprite)n.material=material(o.material);
 if(o.isSprite){const g=message.data.objects.filter(x=>x.objectType==='TextDot')[dot++].geometry;const c=o.userData.attributes.drawColor;n.textDot={text:g.text,fontHeight:g.fontHeight,fontFace:g.fontFace,color:[c.r,c.g,c.b,c.a],point:g.point};}
 if(o.isLight){n.light={color:o.color.toArray(),intensity:o.intensity,distance:o.distance??null,decay:o.decay??null,angle:o.angle??null,penumbra:o.penumbra??null,width:o.width??null,height:o.height??null,target:o.target?o.target.position.toArray():null,castShadow:o.castShadow};}
 json.nodes.push(n);
}
const out=new URL('web/gallery/assets/3dm-loader/',root);
await mkdir(out,{recursive:true});
json.materials=materials;
const bytes=Buffer.concat(chunks);
await writeFile(new URL('Rhino_Logo.bin',out),bytes);
json.file='Rhino_Logo.bin';json.sha256=createHash('sha256').update(bytes).digest('hex');
await writeFile(new URL('Rhino_Logo.json',out),JSON.stringify(json)+'\n');
const types={};for(const o of nodes)types[o.type]=(types[o.type]||0)+1;
console.log(types,materials.length,bytes.length,dots);
