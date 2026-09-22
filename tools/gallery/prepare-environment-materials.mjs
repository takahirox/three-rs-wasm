import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(s,c,n){if(s==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return n(s,c);}});
const {OBJLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/OBJLoader.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/environment-materials/',import.meta.url);await mkdir(out,{recursive:true});const manifest=[],sha=x=>createHash('sha256').update(x).digest('hex');
const paths=JSON.parse(await readFile(new URL('./environment-material-paths.json',import.meta.url),'utf8'));
for(const path of paths){if(path.endsWith('.obj'))continue;const dest=new URL(path,out);await mkdir(new URL('.',dest),{recursive:true});await copyFile(new URL(path,root),dest);manifest.push({file:path,source:'examples/'+path,sha256:sha(await readFile(dest))});}
const source='models/obj/ninja/ninjaHead_Low.obj',text=await readFile(new URL(source,root),'utf8');const g=new OBJLoader().parse(text).children[0].geometry;g.center();const words=[];const uint=x=>{const b=Buffer.alloc(4);b.writeUInt32LE(x);words.push(b);};uint(1);uint(g.attributes.position.count);uint(0);for(const name of ['position','normal','uv']){const a=g.attributes[name].array,b=Buffer.alloc(a.length*4);a.forEach((x,i)=>b.writeFloatLE(x,i*4));words.push(b);}const data=Buffer.concat(words);await writeFile(new URL('ninja.bin',out),data);manifest.push({file:'ninja.bin',source:'examples/'+source,source_sha256:sha(text),sha256:sha(data)});await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
