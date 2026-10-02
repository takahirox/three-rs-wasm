// Bake webgl_loader_vrml's sixteen assets: the pinned VRMLLoader's parse of each
// models/vrml/*.wrl ( r186, MIT ), stored as the loader leaves the scene. Each renderable
// ( mesh, line segments, points ) keeps its id, render and group order, world matrix,
// geometry ( attributes, index, groups, bounding sphere ) and material; textures keep their
// image URL or pixel data with their wrapping, filtering and uv transform.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import {registerHooks} from 'node:module';
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return next(specifier,context);}});
// TextureLoader creates image elements; the bake records the URL only.
globalThis.document={createElementNS:()=>({addEventListener(){},removeEventListener(){},style:{}})};
const {VRMLLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/VRMLLoader.js');
// The image URL each ImageTexture node requests.
const {TextureLoader}=await import('three');
const load=TextureLoader.prototype.load;TextureLoader.prototype.load=function(url,...rest){const texture=load.call(this,url,...rest);texture.userData.url=url;return texture;};
const root=new URL('../../',import.meta.url);
const assets=['creaseAngle','crystal','house','elevationGrid1','elevationGrid2','extrusion1','extrusion2','extrusion3','lines','linesTransparent','meshWithLines','meshWithTexture','pixelTexture','points','camera','multilineString'];
const chunks=[];let offset=0;
const blob=array=>{const bytes=Buffer.from(array.buffer,array.byteOffset,array.byteLength);const at=offset;chunks.push(bytes);offset+=bytes.length;const pad=(4-offset%4)%4;if(pad){chunks.push(Buffer.alloc(pad));offset+=pad;}return {offset:at,length:array.length,type:array.constructor.name};};
const json={source:'examples/models/vrml/',assets:[]};
const finite=v=>Number.isFinite(v)?v:(v>0?1e30:-1e30);
for(const name of assets){
 const text=await readFile(new URL(`.cache/three-r186/examples/models/vrml/${name}.wrl`,root),'utf8');
 const scene=new VRMLLoader().parse(text,'');
 scene.updateMatrixWorld(true);
 const geometries=new Map(),textures=new Map(),out={name,source_sha256:createHash('sha256').update(text).digest('hex'),geometries:[],textures:[],objects:[]};
 const geometry=g=>{if(!geometries.has(g)){if(g.boundingSphere===null)g.computeBoundingSphere();geometries.set(g,out.geometries.length);out.geometries.push({attributes:Object.fromEntries(Object.entries(g.attributes).map(([k,a])=>[k,{...blob(a.array),itemSize:a.itemSize,normalized:a.normalized}])),index:g.index?blob(g.index.array):null,groups:g.groups,drawRange:[g.drawRange.start,finite(g.drawRange.count)],sphere:[...g.boundingSphere.center.toArray(),g.boundingSphere.radius]});}return geometries.get(g);};
 const texture=t=>{if(!t)return null;if(!textures.has(t)){t.updateMatrix();textures.set(t,out.textures.length);out.textures.push({url:t.userData.url??null,data:t.isDataTexture?{...blob(t.image.data),width:t.image.width,height:t.image.height}:null,wrap:[t.wrapS,t.wrapT],filter:[t.magFilter,t.minFilter],mipmaps:t.generateMipmaps,flipY:t.flipY,colorSpace:t.colorSpace,matrix:t.matrix.toArray()});}return textures.get(t);};
 const visit=(o,groupOrder)=>{if(o.isGroup)groupOrder=o.renderOrder;
  if((o.isMesh||o.isLineSegments||o.isPoints)&&o.visible){const m=o.material;
   out.objects.push({id:o.id,kind:o.isMesh?'mesh':o.isLineSegments?'lines':'points',renderOrder:finite(o.renderOrder),groupOrder:finite(groupOrder),frustumCulled:o.frustumCulled,matrixWorld:o.matrixWorld.toArray(),geometry:geometry(o.geometry),
    material:{id:m.id,type:m.type,color:m.color.toArray(),emissive:m.emissive?.toArray()??null,specular:m.specular?.toArray()??null,shininess:m.shininess??null,opacity:m.opacity,transparent:m.transparent,side:m.side,vertexColors:m.vertexColors,depthWrite:m.depthWrite,depthTest:m.depthTest,flatShading:m.flatShading??false,size:m.size??null,sizeAttenuation:m.sizeAttenuation??null,map:texture(m.map)}});}
  for(const c of o.children)visit(c,groupOrder);};
 visit(scene,0);
 json.assets.push(out);
}
const bytes=Buffer.concat(chunks);
const dir=new URL('web/gallery/assets/vrml/',root);
await mkdir(dir,{recursive:true});
await writeFile(new URL('scenes.bin',dir),bytes);
json.file='scenes.bin';json.sha256=createHash('sha256').update(bytes).digest('hex');
await writeFile(new URL('scenes.json',dir),JSON.stringify(json)+'\n');
await copyFile(new URL('.cache/three-r186/examples/models/vrml/map.gif',root),new URL('map.gif',dir));
console.log(json.assets.map(a=>`${a.name}:${a.objects.length}/${a.textures.length}`).join(' '),bytes.length);
