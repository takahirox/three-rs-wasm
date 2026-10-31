import {test,expect} from '@playwright/test';
import {readFileSync} from 'node:fs';
// The loader port is in webgl_loader_fbx's module ( tools/gallery/modules.py ).
const PACKAGE=JSON.parse(readFileSync(new URL('../../web/examples/modules.json',import.meta.url),'utf8')).examples.webgl_loader_fbx;
// The Rust FBXLoader against the pinned r186 FBXLoader in the same engine:
// every FBX model the examples load, in the canonical form of fbx::canonical
// ( numbers as their bits, buffers by hash ).
const ASSETS=['Samba Dancing','morph_test','monkey','monkey_embedded_texture','vCube','archer/ArcherRi01','warrior/Warrior','stanford-bunny','mixamo','RotationTest','exampleWindow','Head_69','morph-translation','ball_anims_asc_2018','nurbs'];
test('FBXLoader port matches the original bit for bit',async({page})=>{
 test.setTimeout(300000);
 await page.goto('/reference/three-js/fbx-loader.html');await expect(page).toHaveTitle('ready');
 const results=await page.evaluate(async ([assets,pkg])=>{
  const {loadRuntime}=await import('/web/examples/runtime.js');const rust=await loadRuntime(pkg);
  const f64=new Float64Array(1),u32=new Uint32Array(f64.buffer);
  const n=v=>{if(typeof v!=='number')v=Number(v);if(Number.isNaN(v))return '"NaN"';f64[0]=v;return `"${u32[1].toString(16).padStart(8,'0')}${u32[0].toString(16).padStart(8,'0')}"`;};
  const ns=a=>'['+Array.from(a,n).join(',')+']';
  const s=v=>JSON.stringify(v);
  const os=v=>v===undefined||v===null?'null':JSON.stringify(v);
  const hash=(bytes,count)=>{let h=0x811c9dc5;for(let i=0;i<bytes.length;i++){h^=bytes[i];h=Math.imul(h,0x01000193)>>>0;}return `"${count}:${h.toString(16).padStart(8,'0')}"`;};
  const th=a=>hash(new Uint8Array(a.buffer,a.byteOffset,a.byteLength),a.length);
  async function canonical(model){
   const nodes=[];const index=new Map();model.traverse(o=>{index.set(o,nodes.length);nodes.push(o);});
   const geometries=[],materials=[];
   for(const o of nodes){
    if(o.isMesh||o.isLine){for(const m of [].concat(o.material))if(!materials.includes(m))materials.push(m);}
    if((o.isMesh||o.isLine)&&o.geometry&&!geometries.includes(o.geometry))geometries.push(o.geometry);
   }
   const keys=['map','alphaMap','aoMap','bumpMap','displacementMap','emissiveMap','envMap','normalMap','specularMap'];
   const textures=[];for(const m of materials)for(const k of keys)if(m[k]&&!textures.includes(m[k]))textures.push(m[k]);
   const ti=t=>t?String(textures.indexOf(t)):'null';
   let out='{"nodes":[';
   out+=nodes.map(o=>{
    let extra='null';
    if(o.isMesh)extra=`[${geometries.indexOf(o.geometry)},[${[].concat(o.material).map(m=>materials.indexOf(m)).join(',')}],${Array.isArray(o.material)},[${Object.entries(o.morphTargetDictionary??{}).map(([k,v])=>`[${s(k)},${v}]`).join(',')}],${o.isSkinnedMesh&&o.skeleton?`[[${o.skeleton.bones.map(b=>b&&index.has(b)?index.get(b):'null').join(',')}],[${o.skeleton.boneInverses.map(m=>ns(m.elements)).join(',')}],${ns(o.bindMatrix.elements)}]`:'null'}]`;
    else if(o.isLine)extra=`[${o.geometry?geometries.indexOf(o.geometry):'null'},${materials.indexOf(o.material)}]`;
    else if(o.isPerspectiveCamera)extra=ns([o.fov,o.aspect,o.near,o.far]);
    else if(o.isPointLight)extra=`[${ns(o.color.toArray())},${ns([o.intensity,o.distance,o.decay])}]`;
    else if(o.isSpotLight)extra=`[${ns(o.color.toArray())},${ns([o.intensity,o.distance,o.angle,o.penumbra,o.decay])}]`;
    else if(o.isDirectionalLight||o.isAmbientLight)extra=`[${ns(o.color.toArray())},${n(o.intensity)}]`;
    const parent=o.parent&&index.has(o.parent)?index.get(o.parent):'null';
    return `[${s(o.name)},${s(o.type)},${parent},${ns(o.position.toArray())},${ns(o.quaternion.toArray())},${ns(o.scale.toArray())},${ns(o.matrixWorld.elements)},${o.castShadow},${extra}]`;
   }).join(',');
   out+='],"geometries":[';
   out+=geometries.map(g=>{
    const attrs=Object.keys(g.attributes).sort().map(k=>{const a=g.attributes[k];return `[${s(k)},${a.itemSize},${s(a.array.constructor.name)},${th(a.array)}]`;}).join(',');
    const groups=g.groups.map(x=>`[${x.start},${x.count},${x.materialIndex}]`).join(',');
    const morph=(g.morphAttributes.position??[]).map(a=>`[${s(a.name??'')},${th(a.array)}]`).join(',');
    return `[${s(g.name)},[${attrs}],[${groups}],[${morph}],${g.morphTargetsRelative}]`;
   }).join(',');
   out+='],"materials":[';
   out+=materials.map(m=>{
    const lit=m.isMeshPhongMaterial||m.isMeshLambertMaterial;
    return `[${s(m.type)},${os(m.name)},${ns(m.color.toArray())},${m.isMeshPhongMaterial?ns(m.specular.toArray()):'null'},${m.isMeshPhongMaterial?n(m.shininess):'null'},${lit?ns(m.emissive.toArray()):'null'},${lit?n(m.emissiveIntensity):'null'},${n(m.opacity)},${m.transparent},${m.vertexColors},${lit?n(m.bumpScale):'null'},${lit?n(m.displacementScale):'null'},${lit?n(m.reflectivity):'null'},[${keys.map(k=>ti(m[k])).join(',')}]]`;
   }).join(',');
   out+='],"textures":[';
   const tex=[];
   for(const t of textures){
    const url=t.userData.referenceUrl;let image='null,null';
    if(url!==undefined&&url.startsWith('blob:')){const r=await fetch(url);const b=new Uint8Array(await r.arrayBuffer());image=`${s(r.headers.get('content-type'))},${hash(b,b.length)}`;}
    else if(url!==undefined)image=`${s(url)},null`;
    tex.push(`[${os(t.name)},${image},${t.wrapS},${t.wrapT},${ns(t.repeat.toArray().map(Number))},${ns(t.offset.toArray().map(Number))},${t.colorSpace==='srgb'},${t.mapping===303}]`);
   }
   out+=tex.join(',');
   out+='],"clips":[';
   out+=(model.animations??[]).map(c=>`[${s(c.name)},${n(c.duration)},[${c.tracks.map(t=>`[${s(t.name)},${s(t.ValueTypeName)},${th(t.times)},${th(t.values)}]`).join(',')}]]`).join(',');
   out+=`],"unitScaleFactor":${model.userData.unitScaleFactor===undefined?'null':n(model.userData.unitScaleFactor)}}`;
   return out;
  }
  const results=[];
  for(const asset of assets){
   const bytes=new Uint8Array(await (await fetch(`/.cache/three-r187/examples/models/fbx/${encodeURIComponent(asset).replace('%2F','/')}.fbx`)).arrayBuffer());
   const dir=asset.includes('/')?asset.slice(0,asset.lastIndexOf('/')+1):'';
   const loader=new window.FBXLoader();loader.trimAnimationClips=asset==='ball_anims_asc_2018';
   const expected=await canonical(loader.parse(bytes.slice().buffer,dir));
   const actual=rust.fbx_canonical(bytes,dir,asset==='ball_anims_asc_2018',innerWidth,innerHeight);
   let at=-1;for(let i=0;i<Math.max(expected.length,actual.length);i++)if(expected[i]!==actual[i]){at=i;break;}
   results.push({asset,equal:expected===actual,length:expected.length,context:at<0?null:{expected:expected.slice(Math.max(0,at-120),at+80),actual:actual.slice(Math.max(0,at-120),at+80)}});
  }
  return results;
 },[ASSETS,PACKAGE]);
 for(const r of results)expect(r,JSON.stringify(r)).toMatchObject({equal:true});
});
