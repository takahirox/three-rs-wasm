// Bake webgl_batch_lod_bvh's four LODs per torus knot as @three.ez/simplify-geometry 0.0.1
// builds them with meshoptimizer 1.1.1 ( MIT, the versions the page loads from jsDelivr;
// pass the meshoptimizer ESM bundle ): simplifyGeometriesByErrorLOD( geometries, 4,
// performanceRangeLOD ) calls MeshoptSimplifier.simplify( index, position, 3, 0, error, [] )
// for the errors 0.01, 0.02, 0.05 and 0.1. Only the index changes: the port builds the
// knots at load and checks each one's position hash against the baked one.
// usage: node tools/tsl/prepare-batch-lod.mjs <meshoptimizer ESM bundle>
import {writeFile,mkdir} from 'node:fs/promises';
import {pathToFileURL} from 'node:url';
const {MeshoptSimplifier}=await import(pathToFileURL(process.argv[2]).href);
const THREE=await import(new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href);
await MeshoptSimplifier.ready;
const knots=[[1,1],[1,2],[1,3],[1,4],[1,5],[2,1],[2,3],[3,1],[4,1],[5,3]];
const errors=[0.01,0.02,0.05,0.1];
// FNV-1a over the position bytes.
const hash=bytes=>{let h=0x811c9dc5;for(const b of bytes)h=Math.imul(h^b,16777619)>>>0;return h;};
const out=[];const u32=v=>{const b=Buffer.alloc(4);b.writeUInt32LE(v);out.push(b);};
u32(knots.length);u32(errors.length);
for(const [p,q] of knots){
 const g=new THREE.TorusKnotGeometry(1,0.4,256,32,p,q);
 const position=g.attributes.position.array,index=g.index.array;
 u32(hash(new Uint8Array(position.buffer,position.byteOffset,position.byteLength)));u32(position.length/3);
 for(const error of errors){
  const [lod]=MeshoptSimplifier.simplify(index,position,3,0,error,[]);
  if(lod.length===index.length)throw Error('simplification failed');
  u32(lod.length);const b=Buffer.alloc(lod.length*2+(lod.length%2)*2);lod.forEach((v,i)=>b.writeUInt16LE(v,i*2));out.push(b);
  console.log(p,q,error,lod.length);
 }
}
await mkdir(new URL('../../web/gallery/assets/batch_lod/',import.meta.url),{recursive:true});
await writeFile(new URL('../../web/gallery/assets/batch_lod/lods.bin',import.meta.url),Buffer.concat(out));
