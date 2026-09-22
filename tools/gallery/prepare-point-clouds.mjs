// Fixed indexed-box sample positions; all animation, color and sizing stay on the GPU.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(s,c,n){if(s==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return n(s,c);}});
const THREE=await import('../../.cache/three-r186/src/Three.Core.js');
const BufferGeometryUtils=await import('../../.cache/three-r186/examples/jsm/utils/BufferGeometryUtils.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/point-clouds/',import.meta.url);await mkdir(out,{recursive:true});
const source='webgl_custom_attributes_points3.html',html=await readFile(new URL(source,root),'utf8');
const body=html.slice(html.indexOf('let radius = 100;'),html.indexOf("const texture = new THREE.TextureLoader()"));
let seed=186;const random=()=>((seed=(Math.imul(seed,1664525)+1013904223)>>>0)/4294967296);
const {geometry,vertices1}=Function('THREE','BufferGeometryUtils','random','let vertices1;'+body.replaceAll('Math.random()','random()')+'return {geometry,vertices1};')(THREE,BufferGeometryUtils,random);
const positions=geometry.attributes.position.array;const binary=Buffer.alloc(8+positions.length*4);binary.writeUInt32LE(positions.length/3);binary.writeUInt32LE(vertices1,4);positions.forEach((v,i)=>binary.writeFloatLE(v,8+i*4));
const sha=x=>createHash('sha256').update(x).digest('hex'),manifest=[];
await writeFile(new URL('frame.bin',out),binary);manifest.push({file:'frame.bin',source:'examples/'+source,source_sha256:sha(html),sha256:sha(binary)});
for(const name of ['disc.png','ball.png',...Array.from({length:5},(_,i)=>`snowflake${i+1}.png`)]){await copyFile(new URL('textures/sprites/'+name,root),new URL(name,out));manifest.push({file:name,source:'examples/textures/sprites/'+name,sha256:sha(await readFile(new URL(name,out)))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
