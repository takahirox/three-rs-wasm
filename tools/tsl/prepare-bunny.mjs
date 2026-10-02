// Bake webgl_materials_subsurface_scattering's model: the pinned FBXLoader's parse of
// models/fbx/stanford-bunny.fbx ( r186, MIT ), whose first child the page adds to the scene.
// Its non-indexed position, normal and uv attributes are stored as the loader leaves them,
// with the child's Euler angles ( the page replaces its position, scale and y rotation ).
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {registerHooks} from 'node:module';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
// FBXLoader creates a TextureLoader; the file's textures are not read.
globalThis.document={createElementNS:()=>({addEventListener(){},removeEventListener(){},style:{}})};
const {FBXLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FBXLoader.js');
const root=new URL('../../',import.meta.url);
const source=await readFile(new URL('.cache/three-r186/examples/models/fbx/stanford-bunny.fbx',root));
const model=new FBXLoader().parse(source.buffer.slice(source.byteOffset,source.byteOffset+source.length),'').children[0];
const g=model.geometry;
const arrays=['position','normal','uv'].map(name=>g.attributes[name].array);
const bytes=Buffer.concat(arrays.map(a=>Buffer.from(a.buffer,a.byteOffset,a.byteLength)));
const out=new URL('web/gallery/assets/subsurface-scattering/',root);
await mkdir(out,{recursive:true});
await writeFile(new URL('bunny.bin',out),bytes);
const json={source:'examples/models/fbx/stanford-bunny.fbx',source_sha256:createHash('sha256').update(source).digest('hex'),vertices:g.attributes.position.count,attributes:['position','normal','uv'],rotation:model.rotation.toArray().slice(0,3),file:'bunny.bin',sha256:createHash('sha256').update(bytes).digest('hex')};
await writeFile(new URL('bunny.json',out),JSON.stringify(json)+'\n');
console.log(json.vertices,bytes.length);
