// The canonical text of a USDLoader parse, as usd::canonical writes it for the
// port: numbers as the hexadecimal bits of their double, buffers as their FNV-1a
// hash or, with `full`, their values, and geometries, materials, textures and
// skeletons in first-reference order. `images` maps a texture's image source
// ( a blob URL ) to its bytes; a texture's source is its image's src attribute.
const f64=new Float64Array(1),u32=new Uint32Array(f64.buffer);
const n=v=>{if(typeof v!=='number')v=Number(v);if(Number.isNaN(v))return '"NaN"';f64[0]=v;return `"${u32[1].toString(16).padStart(8,'0')}${u32[0].toString(16).padStart(8,'0')}"`;};
const ns=a=>'['+Array.from(a,n).join(',')+']';
const s=v=>JSON.stringify(v);
const hash=(bytes,count)=>{let h=0x811c9dc5;for(let i=0;i<bytes.length;i++){h^=bytes[i];h=Math.imul(h,0x01000193)>>>0;}return `"${count}:${h.toString(16).padStart(8,'0')}"`;};
export function canonical(group,images,full=false){
 const buffer=a=>full?ns(a):hash(new Uint8Array(a.buffer,a.byteOffset,a.byteLength),a.length);
 const nodes=[];const index=new Map();group.traverse(o=>{index.set(o,nodes.length);nodes.push(o);});
 const geometries=[],materials=[],skeletons=[];
 for(const o of nodes)if(o.isMesh){if(!geometries.includes(o.geometry))geometries.push(o.geometry);for(const m of [].concat(o.material))if(!materials.includes(m))materials.push(m);if(o.isSkinnedMesh&&o.skeleton&&!skeletons.includes(o.skeleton))skeletons.push(o.skeleton);}
 if(group.skeleton&&!skeletons.includes(group.skeleton))skeletons.push(group.skeleton);
 const keys=['map','emissiveMap','normalMap','roughnessMap','metalnessMap','aoMap','specularColorMap'];
 const textures=[];for(const m of materials)for(const k of keys)if(m[k]&&!textures.includes(m[k]))textures.push(m[k]);
 let out='{"nodes":[';
 out+=nodes.map(o=>{
  let extra='null';
  if(o.isMesh)extra=`[${geometries.indexOf(o.geometry)},[${[].concat(o.material).map(m=>materials.indexOf(m)).join(',')}],${Array.isArray(o.material)},${o.isSkinnedMesh?`[${o.skeleton?skeletons.indexOf(o.skeleton):'null'},${ns(o.bindMatrix.elements)}]`:'null'}]`;
  else if(o.isPerspectiveCamera)extra=ns([o.fov,o.aspect,o.near,o.far,o.filmGauge,o.filmOffset,o.focus]);
  else if(o.isOrthographicCamera)extra=ns([o.left,o.right,o.top,o.bottom,o.near,o.far]);
  else if(o.isSpotLight)extra=`[${ns(o.color.toArray())},${ns([o.intensity,o.angle,o.penumbra])}]`;
  else if(o.isRectAreaLight)extra=`[${ns(o.color.toArray())},${ns([o.intensity,o.width,o.height])}]`;
  else if(o.isLight)extra=`[${ns(o.color.toArray())},${n(o.intensity)}]`;
  const parent=o.parent&&index.has(o.parent)?index.get(o.parent):'null';
  return `[${s(o.name)},${s(o.type)},${parent},${ns(o.position.toArray())},${ns(o.quaternion.toArray())},${ns(o.scale.toArray())},${o.pivot?ns(o.pivot.toArray()):'null'},${ns(o.matrixWorld.elements)},${extra}]`;
 }).join(',');
 out+='],"geometries":[';
 out+=geometries.map(g=>{
  const attrs=Object.keys(g.attributes).sort().map(k=>{const a=g.attributes[k];return `[${s(k)},${a.itemSize},${s(a.array.constructor.name)},${buffer(a.array)}]`;}).join(',');
  const ix=g.index?Uint32Array.from(g.index.array):null;
  return `[${s(g.name)},[${attrs}],${ix?buffer(ix):'null'},[${g.groups.map(x=>`[${x.start},${x.count},${x.materialIndex}]`).join(',')}]]`;
 }).join(',');
 out+='],"materials":[';
 out+=materials.map(m=>{
  const ti=t=>t?String(textures.indexOf(t)):'null';
  return `[${ns(m.color.toArray())},${ns(m.emissive.toArray())},${ns([m.emissiveIntensity,m.roughness,m.metalness,m.ior,m.specularIntensity,m.clearcoat,m.clearcoatRoughness,m.transmission,m.thickness,m.attenuationDistance,m.iridescence,m.iridescenceIOR,m.iridescenceThicknessMinimum,m.iridescenceThicknessMaximum,m.anisotropy,m.opacity,m.alphaTest,m.normalScale.x,m.normalScale.y])},[${ns(m.specularColor.toArray())},${ns(m.attenuationColor.toArray())},${m.transparent}],[${keys.map(k=>ti(m[k])).join(',')}]]`;
 }).join(',');
 out+='],"textures":[';
 out+=textures.map(t=>{
  const im=t.image;const src=im?(im.getAttribute?im.getAttribute('src'):im.src):null;const bytes=images.get(src);
  const image=bytes?`null,${hash(bytes,bytes.length)}`:`${s(src??null)},null`;
  const list=v=>v?ns(v):'null';
  return `[${image},${t.colorSpace==='srgb'},${t.wrapS},${t.wrapT},${ns(t.repeat.toArray())},${ns(t.offset.toArray())},${n(t.rotation)},${t.channel},${list(t.userData.scale)},${list(t.userData.bias)}]`;
 }).join(',');
 out+='],"skeletons":[';
 out+=skeletons.map(k=>`[[${k.bones.map(b=>b&&index.has(b)?index.get(b):'null').join(',')}],[${k.boneInverses.map(m=>ns(m.elements)).join(',')}]]`).join(',');
 out+='],"clips":[';
 out+=(group.animations??[]).map(c=>`[${s(c.name)},${n(c.duration)},[${c.tracks.map(t=>`[${s(t.name)},${s(t.ValueTypeName)},${buffer(t.times)},${buffer(t.values)}]`).join(',')}]]`).join(',');
 out+=`],"skeleton":${group.skeleton?skeletons.indexOf(group.skeleton):'null'}}`;
 return out;
}
