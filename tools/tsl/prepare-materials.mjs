// The TSL materials examples' original assets; their geometry is built at load.
import {mkdir,copyFile} from 'node:fs/promises';
const root=new URL('../../.cache/three-r187/examples/',import.meta.url),out=new URL('../../web/examples/assets/tsl-materials/',import.meta.url);
await mkdir(out,{recursive:true});
for(const p of ['models/fbx/bunny_thickness.jpg','models/gltf/ferrari.glb','models/gltf/ferrari_ao.png','textures/equirectangular/blouberg_sunrise_2_1k.hdr']){await mkdir(new URL(p.split('/').slice(0,-1).join('/')+'/',out),{recursive:true});await copyFile(new URL(p,root),new URL(p,out));}

