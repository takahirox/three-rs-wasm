// Bake webgl_loader_usdz's model: the pinned USDLoader's parse of models/usdz/saeukkang.usdz
// ( r186, MIT ). The mesh keeps its world matrix ( before the page offsets the model ), its
// non-indexed attributes and its MeshPhysicalMaterial parameters; the base color texture is
// the image the loader decodes from the archive, stored as its original bytes.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {registerHooks} from 'node:module';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
globalThis.document={createElementNS:()=>({addEventListener(){},removeEventListener(){},style:{}})};
// The loader decodes archive images through object URLs and Image elements: keep the bytes.
const blobs=new Map();const create=URL.createObjectURL;URL.createObjectURL=blob=>{const url=create(blob);blobs.set(url,blob);return url;};URL.revokeObjectURL=()=>{};
globalThis.Image=class{set src(value){this._src=value;setTimeout(()=>this.onload?.(),0);}get src(){return this._src;}};
const {USDLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/USDLoader.js');
const root=new URL('../../',import.meta.url);
const source=await readFile(new URL('.cache/three-r186/examples/models/usdz/saeukkang.usdz',root));
const scene=new USDLoader().parse(source.buffer.slice(source.byteOffset,source.byteOffset+source.length));
await new Promise(resolve=>setTimeout(resolve,50));
scene.updateMatrixWorld(true);
const meshes=[];scene.traverse(o=>{if(o.isMesh)meshes.push(o);});
if(meshes.length!==1)throw Error('expected one mesh');
const [mesh]=meshes,g=mesh.geometry,m=mesh.material;
const arrays=['position','normal','uv'].map(name=>g.attributes[name].array);
const bytes=Buffer.concat(arrays.map(a=>Buffer.from(a.buffer,a.byteOffset,a.byteLength)));
const image=Buffer.from(await blobs.get(m.map.image.src).arrayBuffer());
const extension=image[0]===0x89?'png':'jpg';
const out=new URL('web/gallery/assets/usdz/',root);
await mkdir(out,{recursive:true});
await writeFile(new URL('saeukkang.bin',out),bytes);
await writeFile(new URL(`saeukkang.${extension}`,out),image);
const json={source:'examples/models/usdz/saeukkang.usdz',source_sha256:createHash('sha256').update(source).digest('hex'),vertices:g.attributes.position.count,attributes:['position','normal','uv'],matrixWorld:mesh.matrixWorld.toArray(),
 material:{type:m.type,color:m.color.toArray(),roughness:m.roughness,metalness:m.metalness,ior:m.ior,specularIntensity:m.specularIntensity,specularColor:m.specularColor.toArray(),clearcoat:m.clearcoat,sheen:m.sheen,transmission:m.transmission,iridescence:m.iridescence,opacity:m.opacity,transparent:m.transparent,side:m.side,emissive:m.emissive.toArray(),
  map:{file:`saeukkang.${extension}`,wrap:[m.map.wrapS,m.map.wrapT],flipY:m.map.flipY,colorSpace:m.map.colorSpace,matrix:(m.map.updateMatrix(),m.map.matrix.toArray()),channel:m.map.channel}},
 file:'saeukkang.bin',sha256:createHash('sha256').update(bytes).digest('hex')};
await writeFile(new URL('saeukkang.json',out),JSON.stringify(json)+'\n');
console.log(JSON.stringify(json).slice(0,900));
