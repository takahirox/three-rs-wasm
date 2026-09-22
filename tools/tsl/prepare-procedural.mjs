// Bake only static font geometry with pinned Three.js; all shading stays in Rust.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
const {FontLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FontLoader.js');
const {TextGeometry}=await import('../../.cache/three-r186/examples/jsm/geometries/TextGeometry.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/tsl-procedural/',import.meta.url);
await mkdir(out,{recursive:true});
const fontBytes=await readFile(new URL('fonts/helvetiker_regular.typeface.json',root));
const font=new FontLoader().parse(JSON.parse(fontBytes));
const names=[['Noise','2D'],['Noise','3D'],['Cellnoise','2D'],['Cellnoise','3D'],['Worley','2D style 0'],['Worley','3D style 0'],['Worley','2D style 1'],['Worley','3D style 1'],['Fractal','2D'],['Fractal','3D'],['Unified Perlin','2D'],['Unified Perlin','3D'],['Unified Cell','2D'],['Unified Cell','3D'],['Unified Worley','2D style 0'],['Unified Worley','3D style 0'],['Unified Worley','2D style 1'],['Unified Worley','3D style 1'],['Unified Fractal','2D'],['Unified Fractal','3D'],['Hex Tiled','checkerboard']];
const chunks=[];const word=n=>{const b=Buffer.alloc(4);b.writeUInt32LE(n);chunks.push(b);};word(names.length*2);
for(const [i,lines] of names.entries())for(const [line,name] of lines.entries()){
 const g=new TextGeometry(name,{font,size:1.15,depth:.02,curveSegments:2});g.computeBoundingBox();const box=g.boundingBox;
 g.translate(-.5*(box.max.x-box.min.x)+(i%7)*15.5-46.5,-.5*(box.max.y-box.min.y)+.775-line*1.55+18-Math.floor(i/7)*18-7.5,0);
 word(g.attributes.position.count);word(g.index?.count??0);
 for(const key of ['position','normal','uv']){const a=g.attributes[key].array;const b=Buffer.alloc(a.length*4);a.forEach((v,i)=>b.writeFloatLE(v,i*4));chunks.push(b);}
 if(g.index){const a=g.index.array;const b=Buffer.alloc(a.length*4);a.forEach((v,i)=>b.writeUInt32LE(v,i*4));chunks.push(b);}
}
const bytes=Buffer.concat(chunks);await writeFile(new URL('noise-labels.bin',out),bytes);
const hash=b=>createHash('sha256').update(b).digest('hex');
await writeFile(new URL('geometry.json',out),JSON.stringify([{file:'noise-labels.bin',source:'fonts/helvetiker_regular.typeface.json',sha256:hash(bytes),source_sha256:hash(fontBytes)}],null,2)+'\n');
