// Bake webgl_loader_ifc's rac_advanced_sample_project.ifc as web-ifc 0.0.77 ( MPL-2.0, the
// version the page loads from jsDelivr; pass its directory ) streams it to the page:
// the model opened with COORDINATE_TO_ORIGIN, each placed geometry's expressID, colour
// and flat transformation in stream order, and each distinct geometry's interleaved
// vertex data ( position, normal ) and index as GetVertexArray / GetIndexArray return
// them. The port runs the page's loadAllGeometry on them at load: the sRGB colour
// conversion, applyMatrix4 and mergeGeometries into the opaque and transparent meshes.
// usage: node tools/tsl/prepare-ifc-loader.mjs <web-ifc directory>
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {createRequire} from 'node:module';
const library=process.argv[2];
const root=new URL('../../',import.meta.url);
const {IfcAPI}=createRequire(import.meta.url)(`${library}/web-ifc-api-node.js`);
const source=await readFile(new URL('.cache/three-r186/examples/models/ifc/rac_advanced_sample_project.ifc',root));
const ifcAPI=new IfcAPI();
ifcAPI.SetWasmPath(`${library}/`,true);
await ifcAPI.Init();
const modelID=ifcAPI.OpenModel(new Uint8Array(source),{COORDINATE_TO_ORIGIN:true});
const chunks=[];let offset=0;
const blob=array=>{const bytes=Buffer.from(array.buffer,array.byteOffset,array.byteLength);const at=offset;chunks.push(Buffer.from(bytes));offset+=bytes.length;const pad=(4-offset%4)%4;if(pad){chunks.push(Buffer.alloc(pad));offset+=pad;}return {offset:at,length:array.length,type:array.constructor.name};};
const geometries=[];const geometryIndex=new Map();
const placements=[];
ifcAPI.StreamAllMeshes(modelID,flatMesh=>{
 const placed=flatMesh.geometries;
 for(let i=0;i<placed.size();i++){
  const p=placed.get(i);
  if(!geometryIndex.has(p.geometryExpressID)){
   const g=ifcAPI.GetGeometry(modelID,p.geometryExpressID);
   const vertex=ifcAPI.GetVertexArray(g.GetVertexData(),g.GetVertexDataSize());
   const index=ifcAPI.GetIndexArray(g.GetIndexData(),g.GetIndexDataSize());
   geometryIndex.set(p.geometryExpressID,geometries.length);
   geometries.push({vertex:blob(vertex),index:blob(index)});
   g.delete();
  }
  placements.push(geometryIndex.get(p.geometryExpressID),p.color.x,p.color.y,p.color.z,p.color.w,...p.flatTransformation);
 }
});
ifcAPI.CloseModel(modelID);
// Per placement: geometry, colour ( x, y, z, w ) and the column-major transformation.
const placement=blob(new Float64Array(placements));
const out=new URL('web/gallery/assets/ifc-loader/',root);
await mkdir(out,{recursive:true});
const bytes=Buffer.concat(chunks);
await writeFile(new URL('rac_advanced_sample_project.bin',out),bytes);
const json={source:'examples/models/ifc/rac_advanced_sample_project.ifc',source_sha256:createHash('sha256').update(source).digest('hex'),webIfc:'0.0.77',file:'rac_advanced_sample_project.bin',sha256:createHash('sha256').update(bytes).digest('hex'),placementStride:21,placements:placement,geometries};
await writeFile(new URL('rac_advanced_sample_project.json',out),JSON.stringify(json)+'\n');
console.log(placements.length/21,geometries.length,bytes.length);
