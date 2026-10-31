// Oracle for the FBXLoader port: the pinned r186 FBXLoader's parse of every
// FBX model the examples load, written to .cache/fbx-oracle. Never loaded by
// the app; tests/fbx.rs compares the Rust parse against it.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {resolveObjectURL} from 'node:buffer';
const root=new URL('../../',import.meta.url);
const three=new URL('.cache/three-r187/src/Three.Core.js',root).href;
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:three,shortCircuit:true};return next(specifier,context);}});
// TextureLoader builds an image element; the stub keeps the requested URL.
globalThis.document={createElementNS:()=>({addEventListener(){},removeEventListener(){},style:{}})};
globalThis.self=globalThis;
globalThis.window={innerWidth:1,innerHeight:1};
const {FBXLoader}=await import(new URL('.cache/three-r187/examples/jsm/loaders/FBXLoader.js',root).href);
const {TextureLoader,Texture}=await import(three);
TextureLoader.prototype.load=function(url){const t=new Texture();t.userData.oracleUrl=url.startsWith('blob:')||url.startsWith('data:')?url:(this.path??'')+url;return t;};
const assets=['Samba Dancing','morph_test','monkey','monkey_embedded_texture','vCube','archer/ArcherRi01','warrior/Warrior','stanford-bunny','mixamo','RotationTest','exampleWindow','Head_69','morph-translation','ball_anims_asc_2018','nurbs'];
const out=new URL('.cache/fbx-oracle/',root);
await mkdir(out,{recursive:true});
const num=v=>typeof v==='number'?v:(v===undefined||v===null?null:Number(v));
for(const asset of assets){
 const dir=asset.includes('/')?asset.slice(0,asset.lastIndexOf('/')+1):'';
 const source=await readFile(new URL(`.cache/three-r187/examples/models/fbx/${asset}.fbx`,root));
 const loader=new FBXLoader();
 loader.trimAnimationClips=asset==='ball_anims_asc_2018';
 const model=loader.parse(source.buffer.slice(source.byteOffset,source.byteOffset+source.length),dir);
 const chunks=[];let offset=0;
 const blob=array=>{const bytes=Buffer.from(array.buffer,array.byteOffset,array.byteLength);const at=offset;chunks.push(bytes);offset+=bytes.length;const pad=(8-offset%8)%8;if(pad){chunks.push(Buffer.alloc(pad));offset+=pad;}return {offset:at,length:array.length,type:array.constructor.name};};
 const nodes=[];const index=new Map();
 model.traverse(o=>{index.set(o,nodes.length);nodes.push(o);});
 const geometries=[],geometryIndex=new Map(),materials=[],materialIndex=new Map(),textures=[],textureIndex=new Map();
 const texture=async t=>{
  if(!t)return null;if(textureIndex.has(t))return textureIndex.get(t);
  const url=t.userData.oracleUrl;const r={name:t.name??null,wrapS:t.wrapS,wrapT:t.wrapT,repeat:t.repeat.toArray().map(num),offset:t.offset.toArray().map(num),colorSpace:t.colorSpace,mapping:t.mapping};
  if(url===undefined)r.url=null;
  else if(url.startsWith('blob:')){const b=resolveObjectURL(url);r.blob=blob(new Uint8Array(await b.arrayBuffer()));r.mime=b.type;}
  else r.url=url;
  textures.push(r);textureIndex.set(t,textures.length-1);return textures.length-1;
 };
 const material=async m=>{
  if(materialIndex.has(m))return materialIndex.get(m);
  const r={type:m.type,name:m.name??null,color:m.color.toArray(),specular:m.specular?.toArray()??null,shininess:m.shininess??null,emissive:m.emissive?.toArray()??null,emissiveIntensity:m.emissiveIntensity??null,opacity:m.opacity,transparent:m.transparent,vertexColors:m.vertexColors,bumpScale:m.bumpScale??null,displacementScale:m.displacementScale??null,reflectivity:m.reflectivity??null,maps:{}};
  for(const k of ['map','alphaMap','aoMap','bumpMap','displacementMap','emissiveMap','envMap','normalMap','specularMap'])r.maps[k]=await texture(m[k]);
  materials.push(r);materialIndex.set(m,materials.length-1);return materials.length-1;
 };
 const geometry=g=>{
  if(!g)return null;if(geometryIndex.has(g))return geometryIndex.get(g);
  geometries.push({name:g.name,attributes:Object.fromEntries(Object.entries(g.attributes).map(([k,a])=>[k,{...blob(a.array),itemSize:a.itemSize}])),groups:g.groups.map(x=>[x.start,x.count,x.materialIndex]),morph:(g.morphAttributes.position??[]).map(a=>({name:a.name,...blob(a.array)})),morphTargetsRelative:g.morphTargetsRelative,index:g.index?blob(g.index.array):null});
  geometryIndex.set(g,geometries.length-1);return geometries.length-1;
 };
 const json={source:`examples/models/fbx/${asset}.fbx`,nodes:[],geometries,materials,textures,clips:[],unitScaleFactor:model.userData.unitScaleFactor??null};
 for(const o of nodes){
  const n={name:o.name,type:o.type,id:o.ID??null,parent:o.parent&&index.has(o.parent)?index.get(o.parent):null,position:o.position.toArray(),quaternion:o.quaternion.toArray(),scale:o.scale.toArray(),matrixWorld:o.matrixWorld.toArray(),castShadow:o.castShadow};
  if(o.isLight){n.light={color:o.color.toArray(),intensity:o.intensity,distance:o.distance??null,decay:o.decay??null,angle:o.angle??null,penumbra:o.penumbra??null,target:o.target&&index.has(o.target)?index.get(o.target):null};}
  if(o.isCamera)n.camera={fov:o.fov,aspect:o.aspect,near:o.near,far:o.far};
  if(o.isMesh){const ms=[];for(const m of [].concat(o.material))ms.push(await material(m));n.mesh={geometry:geometry(o.geometry),materials:ms,multi:Array.isArray(o.material),morphTargetDictionary:Object.entries(o.morphTargetDictionary??{}),skin:o.isSkinnedMesh&&o.skeleton?{bones:o.skeleton.bones.map(b=>b&&index.has(b)?index.get(b):null),boneInverses:o.skeleton.boneInverses.map(m=>m.toArray()),bindMatrix:o.bindMatrix.toArray()}:null};}
  if(o.isLine){n.line={geometry:geometry(o.geometry),material:await material(o.material)};}
  json.nodes.push(n);
 }
 for(const clip of model.animations??[])json.clips.push({name:clip.name,duration:clip.duration,tracks:clip.tracks.map(t=>({name:t.name,type:t.ValueTypeName,times:blob(t.times),values:blob(t.values)}))});
 const name=asset.replace(/[^A-Za-z0-9_-]/g,'_');
 await writeFile(new URL(`${name}.bin`,out),Buffer.concat(chunks));
 await writeFile(new URL(`${name}.json`,out),JSON.stringify(json));
 console.log(asset,nodes.length,geometries.length,materials.length,textures.length,json.clips.length);
}
