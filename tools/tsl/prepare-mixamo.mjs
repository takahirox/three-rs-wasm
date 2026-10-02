// Bake webgpu_animation_retargeting_readyplayer's source character: the pinned FBXLoader's
// parse of models/fbx/mixamo.fbx ( r186, MIT ), stored as the loader leaves it. The node
// tree keeps each node's local TRS and the world matrix the loader left; the skinned
// meshes keep their attributes, Phong parameters, skeletons and bind matrices; the first
// clip keeps its keyframe tracks.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {registerHooks} from 'node:module';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
// FBXLoader creates a TextureLoader; the file has no textures.
globalThis.document={createElementNS:()=>({addEventListener(){},removeEventListener(){},style:{}})};
const {FBXLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FBXLoader.js');
const root=new URL('../../',import.meta.url);
const source=await readFile(new URL('.cache/three-r186/examples/models/fbx/mixamo.fbx',root));
const model=new FBXLoader().parse(source.buffer.slice(source.byteOffset,source.byteOffset+source.length),'');
const chunks=[];let offset=0;
const blob=array=>{const bytes=Buffer.from(array.buffer,array.byteOffset,array.byteLength);const at=offset;chunks.push(bytes);offset+=bytes.length;const pad=(4-offset%4)%4;if(pad){chunks.push(Buffer.alloc(pad));offset+=pad;}return {offset:at,length:array.length,type:array.constructor.name};};
const nodes=[];const index=new Map();
model.traverse(o=>{index.set(o,nodes.length);nodes.push(o);});
const json={source:'examples/models/fbx/mixamo.fbx',source_sha256:createHash('sha256').update(source).digest('hex'),nodes:[],meshes:[],clip:null};
for(const o of nodes)json.nodes.push({name:o.name,type:o.type,parent:o.parent?index.get(o.parent)??null:null,position:o.position.toArray(),quaternion:o.quaternion.toArray(),scale:o.scale.toArray(),matrixWorld:o.matrixWorld.toArray()});
for(const o of nodes){
 if(!o.isSkinnedMesh)continue;
 const g=o.geometry,m=o.material;
 json.meshes.push({node:index.get(o),
  attributes:Object.fromEntries(Object.entries(g.attributes).map(([k,a])=>[k,{...blob(a.array),itemSize:a.itemSize,normalized:a.normalized}])),
  material:{type:m.type,name:m.name,color:m.color.toArray(),specular:m.specular.toArray(),shininess:m.shininess,emissive:m.emissive.toArray(),emissiveIntensity:m.emissiveIntensity,opacity:m.opacity,transparent:m.transparent,side:m.side,flatShading:m.flatShading},
  bindMode:o.bindMode,bindMatrix:o.bindMatrix.toArray(),bindMatrixInverse:o.bindMatrixInverse.toArray(),
  bones:o.skeleton.bones.map(b=>index.get(b)),boneInverses:o.skeleton.boneInverses.map(m=>m.toArray())});
}
const clip=model.animations[0];
json.clip={name:clip.name,duration:clip.duration,tracks:clip.tracks.map(t=>({name:t.name,type:t.ValueTypeName,interpolation:t.getInterpolation(),times:blob(t.times),values:blob(t.values)}))};
const bytes=Buffer.concat(chunks);
const out=new URL('web/gallery/assets/retargeting-readyplayer/',root);
await mkdir(out,{recursive:true});
await writeFile(new URL('mixamo.bin',out),bytes);
json.file='mixamo.bin';json.sha256=createHash('sha256').update(bytes).digest('hex');
await writeFile(new URL('mixamo.json',out),JSON.stringify(json)+'\n');
console.log(nodes.length,json.meshes.length,json.clip.tracks.length,bytes.length);
