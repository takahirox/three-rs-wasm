// Static geometry from the pinned example; all animated shading stays on the GPU.
import {readFileSync,writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import * as THREE from '../../.cache/three-r186/src/Three.Core.js';
const source='examples/webgpu_materials_retroreflection.html';
const html=readFileSync(new URL('../../.cache/three-r186/'+source,import.meta.url),'utf8');
const body=html.slice(html.indexOf('function createBaseGeometry('),html.indexOf('function updateMaterial()'));
const geometry=Function('THREE',body+';return createBaseGeometry(.42,.04,.08);')(THREE);
const file='retro-base.json';const data=JSON.stringify({index:geometry.index?Array.from(geometry.index.array):null,attributes:Object.fromEntries(Object.entries(geometry.attributes).map(([k,a])=>[k,{size:a.itemSize,array:Array.from(a.array)}]))})+'\n';
const output=new URL('../../web/gallery/assets/tsl-procedural/',import.meta.url);writeFileSync(new URL(file,output),data);
const sha=x=>createHash('sha256').update(x).digest('hex');writeFileSync(new URL('retro-base-manifest.json',output),JSON.stringify({file,sha256:sha(data),source,source_sha256:sha(html)},null,2)+'\n');
