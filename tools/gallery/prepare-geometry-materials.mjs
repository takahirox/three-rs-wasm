import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(s,c,n){if(s==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return n(s,c);}});
const THREE=await import('../../.cache/three-r186/src/Three.Core.js');const {Lut}=await import('../../.cache/three-r186/examples/jsm/math/Lut.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/geometry-materials/',import.meta.url);await mkdir(out,{recursive:true});const manifest=[],sha=x=>createHash('sha256').update(x).digest('hex');
const save=async(file,data,source,sourceData)=>{await writeFile(new URL(file,out),data);manifest.push({file,source:'examples/'+source,source_sha256:sha(sourceData),sha256:sha(data)});};
for(const [name,src]of [['wireframe','WaltHeadLo_buffergeometry.json'],['pressure','pressure.json']]){
 const source='models/json/'+src,bytes=await readFile(new URL(source,root));const g=new THREE.BufferGeometryLoader().parse(JSON.parse(bytes));if(name==='pressure'){g.center();g.computeVertexNormals();}
 const n=g.attributes.position.count,words=[];const uint=x=>{const b=Buffer.alloc(4);b.writeUInt32LE(x);words.push(b);};const floats=a=>{const b=Buffer.alloc(a.length*4);a.forEach((v,i)=>b.writeFloatLE(v,i*4));words.push(b);};uint(1);uint(n);uint(g.index?.count??0);floats(Array.from(g.attributes.position.array));floats(g.attributes.normal?Array.from(g.attributes.normal.array):Array(n*3).fill(0));floats(Array.from({length:n*2},(_,j)=>j%2===0?(name==='pressure'?g.attributes.pressure.array[j/2]:j/2%3===0?1:0):(name==='pressure'?0:(j-1)/2%3===1?1:0)));if(g.index)for(const x of g.index.array)uint(x);await save(name+'.bin',Buffer.concat(words),source,bytes);
}
const colors=[],pixels=[];for(const name of ['rainbow','cooltowarm','blackbody','grayscale']){const lut=new Lut(name);for(const c of lut.lut)colors.push(...c.clone().convertSRGBToLinear().toArray(),1);const image={data:new Uint8ClampedArray(lut.n*4)};lut.updateCanvas({getContext:()=>({getImageData:()=>image,putImageData:()=>{}})});pixels.push(...image.data);}
const bytes=Buffer.alloc(colors.length*4);colors.forEach((x,i)=>bytes.writeFloatLE(x,i*4));const src='jsm/math/Lut.js',original=await readFile(new URL(src,root));await save('lut.bin',bytes,src,original);await save('lut.rgba',Buffer.from(pixels),src,original);await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
