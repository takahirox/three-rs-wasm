// Bake webgpu_compute_rasterizer_ibl's load-time Meshopt LODs and meshlets (r186, MIT).
// The page runs MeshoptSimplifier and MeshoptClusterizer (meshoptimizer 1.1) on the
// DamagedHelmet geometry at load; this runs the same code on the same input and stores
// the result, which the port packs into the page's storage buffers.
import {readFile,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import * as THREE from '../../.cache/three-r186/src/Three.Core.js';
import {MeshoptSimplifier} from '../../.cache/three-r186/examples/jsm/libs/meshopt_simplifier.module.js';
import {MeshoptClusterizer} from '../../.cache/three-r186/examples/jsm/libs/meshopt_clusterizer.module.js';
const root=new URL('../../',import.meta.url);
const page=await readFile(new URL('.cache/three-r186/examples/webgpu_compute_rasterizer_ibl.html',root),'utf8');
const gltfUrl=new URL('web/models/DamagedHelmet/glTF/DamagedHelmet.gltf',root);
const gltf=JSON.parse(await readFile(gltfUrl,'utf8'));
const bin=await readFile(new URL('web/models/DamagedHelmet/glTF/DamagedHelmet.bin',root));
const accessor=(index,Type,size)=>{const a=gltf.accessors[index],view=gltf.bufferViews[a.bufferView];const offset=bin.byteOffset+view.byteOffset+(a.byteOffset||0);return new Type(bin.buffer.slice(offset,offset+a.count*size*Type.BYTES_PER_ELEMENT));};
const primitive=gltf.meshes[0].primitives[0];
const geom=new THREE.BufferGeometry();
geom.setAttribute('position',new THREE.BufferAttribute(accessor(primitive.attributes.POSITION,Float32Array,3),3));
geom.setAttribute('normal',new THREE.BufferAttribute(accessor(primitive.attributes.NORMAL,Float32Array,3),3));
geom.setAttribute('uv',new THREE.BufferAttribute(accessor(primitive.attributes.TEXCOORD_0,Float32Array,2),2));
geom.setIndex(new THREE.BufferAttribute(accessor(primitive.indices,Uint16Array,1),1));
// The glTF node's rotation, baked as the page bakes sourceMesh.matrixWorld.
const node=gltf.nodes[0],object=new THREE.Object3D();
if(node.rotation)object.quaternion.fromArray(node.rotation);
object.updateMatrixWorld(true);geom.applyMatrix4(object.matrixWorld);
await Promise.all([MeshoptClusterizer.ready,MeshoptSimplifier.ready]);
// The page's LOD loop, verbatim between these markers.
const start=page.indexOf('const lodTargets = ['),end=page.indexOf("console.info( 'LOD Meshlets count: '",start);
const lods=new Function('THREE','MeshoptSimplifier','MeshoptClusterizer','sourceMesh',`${page.slice(start,end)}return lods;`)(THREE,MeshoptSimplifier,MeshoptClusterizer,{geometry:geom});
// Layout: u32 vertexCount, u32 lodCount; positions, normals (f32×3) and uvs (f32×2);
// per LOD: u32 meshletCount, then per meshlet 64 triangles of u16 vertex indices
// (padded as the page pads) and the f32 bounding sphere.
const vertexCount=geom.attributes.position.count,parts=[];
const u32=v=>{const b=Buffer.alloc(4);b.writeUInt32LE(v);return b;};
parts.push(u32(vertexCount),u32(lods.length));
for(const name of ['position','normal','uv'])parts.push(Buffer.from(geom.attributes[name].array.buffer.slice(0)));
for(const lod of lods){
 parts.push(u32(lod.numChunks));
 for(let m=0;m<lod.numChunks;m++){
  const meshlet=MeshoptClusterizer.extractMeshlet(lod.meshletBuffers,m),count=meshlet.triangles.length/3;
  const tris=new Uint16Array(64*3);
  for(let t=0;t<64;t++)for(let k=0;k<3;k++)tris[t*3+k]=t<count?meshlet.vertices[meshlet.triangles[t*3+k]]:meshlet.vertices[0];
  parts.push(Buffer.from(tris.buffer));
  const b=lod.bounds[m];parts.push(Buffer.from(new Float32Array([b.centerX,b.centerY,b.centerZ,b.radius]).buffer));
 }
}
const bytes=Buffer.concat(parts);
const out=new URL('web/gallery/assets/rasterizer-ibl/',root);
await writeFile(new URL('helmet-lods.bin',out),bytes);
await writeFile(new URL('helmet-lods.json',out),JSON.stringify({file:'helmet-lods.bin',vertexCount,meshlets:lods.map(l=>l.numChunks),errors:lods.map(l=>l.error),source:'examples/webgpu_compute_rasterizer_ibl.html',source_sha256:createHash('sha256').update(page).digest('hex'),meshoptimizer:'1.1',sha256:createHash('sha256').update(bytes).digest('hex')},null,2)+'\n');
console.log(lods.map(l=>l.numChunks),bytes.length);
