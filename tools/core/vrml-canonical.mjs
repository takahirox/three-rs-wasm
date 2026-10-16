// The canonical text of a VRMLLoader parse, as vrml::canonical writes it for
// the port: numbers as the hexadecimal bits of their double, buffers as their
// FNV-1a hash or, with `full`, their values, objects and materials by id
// relative to the ids before the parse ( `bases` ), and geometries,
// materials and textures in first-reference order. Textures record the URL
// TextureLoader was asked for in `userData.referenceUrl`.
const f64=new Float64Array(1),u32=new Uint32Array(f64.buffer);
const n=v=>{if(typeof v!=='number')v=Number(v);if(Number.isNaN(v))return '"NaN"';f64[0]=v;return `"${u32[1].toString(16).padStart(8,'0')}${u32[0].toString(16).padStart(8,'0')}"`;};
const ns=a=>'['+Array.from(a,n).join(',')+']';
const s=v=>JSON.stringify(v);
const hash=(bytes,count)=>{let h=0x811c9dc5;for(let i=0;i<bytes.length;i++){h^=bytes[i];h=Math.imul(h,0x01000193)>>>0;}return `"${count}:${h.toString(16).padStart(8,'0')}"`;};
export function canonical(scene,bases,full=false){
 const buffer=a=>full?ns(a):hash(new Uint8Array(a.buffer,a.byteOffset,a.byteLength),a.length);
 const nodes=[];const index=new Map();scene.traverse(o=>{index.set(o,nodes.length);nodes.push(o);});
 const geometries=[],materials=[];
 for(const o of nodes)if(o.isMesh||o.isLineSegments||o.isPoints){if(!geometries.includes(o.geometry))geometries.push(o.geometry);if(!materials.includes(o.material))materials.push(o.material);}
 const textures=[];for(const m of materials)if(m.map&&!textures.includes(m.map))textures.push(m.map);
 let out='{"nodes":[';
 out+=nodes.map(o=>{
  let extra='null';
  if(o.isMesh||o.isLineSegments||o.isPoints)extra=`[${geometries.indexOf(o.geometry)},${materials.indexOf(o.material)}]`;
  else if(o.isPerspectiveCamera)extra=ns([o.fov,o.aspect,o.near,o.far]);
  else if(o.isOrthographicCamera)extra=ns([o.left,o.right,o.top,o.bottom,o.near,o.far]);
  const parent=o.parent&&index.has(o.parent)?index.get(o.parent):'null';
  return `[${o.id-bases.object},${s(o.name)},${s(o.type)},${parent},${ns(o.position.toArray())},${ns(o.quaternion.toArray())},${ns(o.scale.toArray())},${n(o.renderOrder)},${o.visible},${ns(o.matrixWorld.elements)},${extra}]`;
 }).join(',');
 out+='],"geometries":[';
 out+=geometries.map(g=>{
  const attrs=Object.keys(g.attributes).sort().map(k=>{const a=g.attributes[k];return `[${s(k)},${a.itemSize},${buffer(a.array)}]`;}).join(',');
  const ix=g.index?Uint32Array.from(g.index.array):null;
  const index=ix===null?'null':full?`[${Array.from(ix).join(',')}]`:hash(new Uint8Array(ix.buffer),ix.length);
  return `[${s(g.name)},[${attrs}],${index},[${g.groups.map(x=>`[${x.start},${x.count},${x.materialIndex}]`).join(',')}]]`;
 }).join(',');
 out+='],"materials":[';
 out+=materials.map(m=>{
  const phong=m.isMeshPhongMaterial,points=m.isPointsMaterial;
  return `[${m.id-bases.material},${s(m.type)},${s(m.name)},${ns(m.color.toArray())},${phong?ns(m.emissive.toArray()):'null'},${phong?ns(m.specular.toArray()):'null'},${phong?n(m.shininess):'null'},${n(m.opacity)},${m.transparent},${m.side},${m.vertexColors},${m.depthWrite},${m.depthTest},${m.fog},${points?n(m.size):'null'},${points?m.sizeAttenuation:'null'},${m.map?textures.indexOf(m.map):'null'}]`;
 }).join(',');
 out+='],"textures":[';
 out+=textures.map(t=>{
  t.updateMatrix();
  const image=t.isDataTexture?`null,${hash(t.image.data,t.image.data.length)},${n(t.image.width)},${n(t.image.height)}`:`${s(t.userData.referenceUrl??null)},null,null,null`;
  return `[${s(t.name)},${image},${t.wrapS},${t.wrapT},${t.magFilter},${t.minFilter},${t.generateMipmaps},${t.flipY},${t.colorSpace==='srgb'},${ns(t.offset.toArray())},${ns(t.repeat.toArray())},${ns(t.center.toArray())},${n(t.rotation)},${ns(t.matrix.elements)}]`;
 }).join(',');
 return out+']}';
}
