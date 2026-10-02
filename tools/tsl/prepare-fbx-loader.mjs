// Bake webgl_loader_fbx's 14 models: the pinned FBXLoader's parse ( r186, MIT ) of each
// models/fbx asset, stored as the loader leaves it. The node tree keeps each node's local
// TRS and type ( groups, bones, meshes, skinned meshes, the ambient light one model
// carries ); meshes keep their attributes, index, groups, morph attributes and target
// names, Phong / Lambert parameters with their texture images and transforms, and
// skinned meshes their bones, bone inverses and bind matrix; the clips keep their
// keyframe tracks resolved to node indices, as PropertyBinding resolves them by name.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {registerHooks} from 'node:module';
import {resolveObjectURL} from 'node:buffer';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
// TextureLoader builds an image element: a stub keeps the requested source.
globalThis.document={createElementNS:()=>({addEventListener(){},removeEventListener(){},style:{}})};
globalThis.self=globalThis;
const {FBXLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FBXLoader.js');
// The texture each load requests, by its resolved path ( the stub image never loads ).
const {TextureLoader,Texture}=await import('../../.cache/three-r186/src/Three.Core.js');
TextureLoader.prototype.load=function(url){const texture=new Texture();texture.userData.bakeSource=url.startsWith('blob:')?url:(this.path||'')+url;return texture;};
const root=new URL('../../',import.meta.url);
const assets=['Samba Dancing','morph_test','monkey','monkey_embedded_texture','vCube','archer/ArcherRi01','warrior/Warrior','stanford-bunny','mixamo','RotationTest','exampleWindow','Head_69','morph-translation','ball_anims_asc_2018'];
const out=new URL('web/gallery/assets/fbx-loader/',root);
await mkdir(out,{recursive:true});
const index=[];
for(const [assetIndex,asset] of assets.entries()){
 const dir=asset.includes('/')?asset.slice(0,asset.lastIndexOf('/')+1):'';
 const source=await readFile(new URL(`.cache/three-r186/examples/models/fbx/${asset}.fbx`,root));
 const loader=new FBXLoader();
 loader.trimAnimationClips=asset==='ball_anims_asc_2018';
 const model=loader.parse(source.buffer.slice(source.byteOffset,source.byteOffset+source.length),dir);
 model.updateMatrixWorld(true);
 const chunks=[];let offset=0;
 const blob=array=>{const bytes=Buffer.from(array.buffer,array.byteOffset,array.byteLength);const at=offset;chunks.push(bytes);offset+=bytes.length;const pad=(4-offset%4)%4;if(pad){chunks.push(Buffer.alloc(pad));offset+=pad;}return {offset:at,length:array.length,type:array.constructor.name};};
 const nodes=[];const nodeIndex=new Map();
 model.traverse(o=>{nodeIndex.set(o,nodes.length);nodes.push(o);});
 const images=new Map();
 const image=async texture=>{
  const src=texture.userData.bakeSource;if(!src)return null;
  if(images.has(src))return images.get(src);
  let bytes,name;
  if(src.startsWith('blob:')){const b=resolveObjectURL(src);bytes=Buffer.from(await b.arrayBuffer());name=`${assetIndex}-${images.size}.${(b.type.split('/')[1]||'png').replace('jpeg','jpg')}`;}
  else{
   // A file the asset names but the examples do not ship ( Head_69's ALPHA.png ) fails to load in the page too.
   try{bytes=await readFile(new URL(`.cache/three-r186/examples/models/fbx/${src}`,root));}catch{images.set(src,null);return null;}
   name=`${assetIndex}-${images.size}-${src.split('/').pop().replace(/[^A-Za-z0-9._-]/g,'_')}`;}
  await writeFile(new URL(name,out),bytes);images.set(src,name);return name;
 };
 const tex=async t=>t?{image:await image(t),wrapS:t.wrapS,wrapT:t.wrapT,repeat:t.repeat.toArray(),offset:t.offset.toArray(),rotation:t.rotation,center:t.center.toArray(),flipY:t.flipY,colorSpace:t.colorSpace}:null;
 const json={source:`examples/models/fbx/${asset}.fbx`,source_sha256:createHash('sha256').update(source).digest('hex'),nodes:[],meshes:[],clips:[]};
 for(const o of nodes){
  const n={name:o.name,type:o.type,parent:o.parent?nodeIndex.get(o.parent)??null:null,position:o.position.toArray(),quaternion:o.quaternion.toArray(),scale:o.scale.toArray()};
  if(o.isLight)n.light={color:o.color.toArray(),intensity:o.intensity};
  json.nodes.push(n);
 }
 for(const o of nodes){
  if(!o.isMesh)continue;
  const g=o.geometry;
  const materials=[];
  for(const m of [].concat(o.material))materials.push({type:m.type,name:m.name,color:m.color.toArray(),specular:m.specular?.toArray()??null,shininess:m.shininess??null,emissive:m.emissive.toArray(),emissiveIntensity:m.emissiveIntensity,opacity:m.opacity,transparent:m.transparent,side:m.side,flatShading:m.flatShading,vertexColors:m.vertexColors,map:await tex(m.map),alphaMap:await tex(m.alphaMap),normalMap:await tex(m.normalMap),bumpMap:await tex(m.bumpMap),specularMap:await tex(m.specularMap),emissiveMap:await tex(m.emissiveMap)});
  json.meshes.push({node:nodeIndex.get(o),
   multiMaterial:Array.isArray(o.material),
   attributes:Object.fromEntries(Object.entries(g.attributes).map(([k,a])=>[k,{...blob(a.array),itemSize:a.itemSize,normalized:a.normalized}])),
   index:g.index?blob(g.index.array):null,
   groups:g.groups,
   morphAttributes:Object.fromEntries(Object.entries(g.morphAttributes).map(([k,list])=>[k,list.map(a=>({...blob(a.array),itemSize:a.itemSize}))])),
   morphTargetsRelative:g.morphTargetsRelative,
   morphTargetDictionary:o.morphTargetDictionary??null,
   materials,
   skin:o.isSkinnedMesh?{bindMode:o.bindMode,bindMatrix:o.bindMatrix.toArray(),bones:o.skeleton.bones.map(b=>nodeIndex.get(b)),boneInverses:o.skeleton.boneInverses.map(m=>m.toArray())}:null});
 }
 for(const clip of model.animations??[]){
  const tracks=[];
  for(const t of clip.tracks){
   const dot=t.name.lastIndexOf('.');const nodeName=t.name.slice(0,dot);const property=t.name.slice(dot+1);
   const target=nodeName===model.name?model:model.getObjectByName(nodeName);
   if(!target)continue;
   tracks.push({node:nodeIndex.get(target),property,type:t.ValueTypeName,interpolation:t.getInterpolation(),times:blob(t.times),values:blob(t.values)});
  }
  json.clips.push({name:clip.name,duration:clip.duration,tracks});
 }
 const bytes=Buffer.concat(chunks);
 const file=`${assetIndex}.bin`;
 await writeFile(new URL(file,out),bytes);
 json.file=file;json.sha256=createHash('sha256').update(bytes).digest('hex');
 await writeFile(new URL(`${assetIndex}.json`,out),JSON.stringify(json)+'\n');
 index.push({asset,file:`${assetIndex}.json`});
 console.log(asset,nodes.length,json.meshes.length,json.clips.length,bytes.length,[...images.values()].join(' '));
}
await writeFile(new URL('index.json',out),JSON.stringify(index)+'\n');
