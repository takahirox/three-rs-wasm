// Bake only the official scene's static per-instance attributes (r186, MIT).
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import * as THREE from '../../.cache/three-r186/src/Three.Core.js';
const source=await readFile(new URL('../../.cache/three-r186/examples/webgpu_reflection.html',import.meta.url),'utf8');
const start=source.indexOf('const maxSteps = 5;'),end=source.indexOf('const geometry = new THREE.BoxGeometry();',start);
let seed=186;const random=()=>{seed=(Math.imul(seed,1664525)+1013904223)>>>0;return seed/4294967296;};
const arrays=new Function('THREE','fixtureRandom',`const random=()=> (fixtureRandom()-.5)*2;${source.slice(start,end).replaceAll('Math.random()','fixtureRandom()')}return [positions,normals,colors,data];`)(THREE,random);
const count=arrays[0].length/3,bytes=Buffer.alloc(count*64+4);bytes.writeUInt32LE(count);
for(let i=0;i<count;i++)for(let a=0;a<4;a++)for(let k=0;k<3;k++)bytes.writeFloatLE(arrays[a][i*3+k],4+i*64+a*16+k*4);
const out=new URL('../../web/gallery/assets/tsl-procedural/',import.meta.url);
await writeFile(new URL('reflection-tree.bin',out),bytes);
await writeFile(new URL('reflection-tree.json',out),JSON.stringify({file:'reflection-tree.bin',count,source:'examples/webgpu_reflection.html',source_sha256:createHash('sha256').update(source).digest('hex'),sha256:createHash('sha256').update(bytes).digest('hex')},null,2)+'\n');
