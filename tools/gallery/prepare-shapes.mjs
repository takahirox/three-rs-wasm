// Fixed example geometry only. Runtime camera, materials and animation remain Rust.
import {mkdir,readFile,writeFile,copyFile} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(s,c,n){if(s==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return n(s,c);}});
const THREE=await import('../../.cache/three-r186/src/Three.Core.js');
const {FontLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/FontLoader.js');
const {SVGLoader}=await import('../../.cache/three-r186/examples/jsm/loaders/SVGLoader.js');
const {ConvexGeometry}=await import('../../.cache/three-r186/examples/jsm/geometries/ConvexGeometry.js');
const BufferGeometryUtils=await import('../../.cache/three-r186/examples/jsm/utils/BufferGeometryUtils.js');
const {NURBSCurve}=await import('../../.cache/three-r186/examples/jsm/curves/NURBSCurve.js');
const {NURBSSurface}=await import('../../.cache/three-r186/examples/jsm/curves/NURBSSurface.js');
const {NURBSVolume}=await import('../../.cache/three-r186/examples/jsm/curves/NURBSVolume.js');
const {ParametricGeometry}=await import('../../.cache/three-r186/examples/jsm/geometries/ParametricGeometry.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/shapes/',import.meta.url);await mkdir(out,{recursive:true});
const sha=x=>createHash('sha256').update(x).digest('hex'),manifest=[];
for(const kind of ['convex','nurbs','text_shapes','text_stroke','shapes']){
 const source=`webgl_geometry_${kind}.html`,html=await readFile(new URL(source,root),'utf8');const group=new THREE.Group();
 if(kind==='shapes'){
  const body=html.slice(html.indexOf('function addShape('),html.indexOf('renderer = new THREE.WebGLRenderer'));
  Function('THREE','group','texture',body)(THREE,group,new THREE.Texture());
 }else if(kind==='convex'){
  const body=html.slice(html.indexOf('let dodecahedronGeometry ='),html.indexOf("window.addEventListener( 'resize'"));
  Function('THREE','BufferGeometryUtils','ConvexGeometry','group','texture',body)(THREE,BufferGeometryUtils,ConvexGeometry,group,new THREE.Texture());
 }else if(kind==='nurbs'){
  let body=html.slice(html.indexOf('// NURBS curve'),html.indexOf('renderer = new THREE.WebGLRenderer'));
  body=body.replace(/new THREE.TextureLoader\(\).load\([^;]*\)/g,'new THREE.Texture()').replaceAll('Math.random()','random()');let seed=186;const random=()=>((seed=(Math.imul(seed,1664525)+1013904223)>>>0)/4294967296);
  Function('THREE','NURBSCurve','NURBSSurface','NURBSVolume','ParametricGeometry','group','random',body)(THREE,NURBSCurve,NURBSSurface,NURBSVolume,ParametricGeometry,group,random);
 }else{
  const font=new FontLoader().parse(JSON.parse(await readFile(new URL(kind==='text_shapes'?'fonts/helvetiker_regular.typeface.json':'fonts/MPLUSRounded1c/MPLUSRounded1c-Regular.typeface.json',root),'utf8')));
  const callback=html.slice(html.indexOf('function ( font ) {')+'function ( font ) {'.length,html.indexOf('} ); //end load'));
  const fn=kind==='text_stroke'?html.slice(html.indexOf('function generateStrokeText('),html.indexOf('function onWindowResize()')):'';
  Function('THREE','SVGLoader','font','scene','render',fn+'\n'+callback)(THREE,SVGLoader,font,group,()=>{});
 }
 group.updateMatrixWorld(true);const objects=[];group.traverse(n=>{if(n.geometry)objects.push(n);});
 const words=[];const uint=n=>{const b=Buffer.alloc(4);b.writeUInt32LE(n);words.push(b);};const floats=a=>{const b=Buffer.alloc(a.length*4);a.forEach((v,i)=>b.writeFloatLE(v,i*4));words.push(b);};uint(objects.length);
 const metadata=[];
 for(const n of objects){const g=n.geometry,v=g.attributes.position.count;uint(v);uint(g.index?.count??0);for(const [name,size]of [['position',3],['normal',3],['uv',2]])floats(g.attributes[name]?Array.from(g.attributes[name].array):Array(v*size).fill(0));if(g.index)for(const i of g.index.array)uint(i);
  const p=new THREE.Vector3(),q=new THREE.Quaternion(),s=new THREE.Vector3();n.matrixWorld.decompose(p,q,s);const m=n.material;
  metadata.push({kind:n.isPoints?'points':n.isLine?'line':'mesh',position:p.toArray(),quaternion:q.toArray(),scale:s.toArray(),color:m.color.toArray(),opacity:m.opacity,transparent:m.transparent,double:m.side===THREE.DoubleSide,lambert:!!m.isMeshLambertMaterial,phong:!!m.isMeshPhongMaterial,size:m.size??1,map:!!m.map});
 }
 for(const [file,data]of [[kind+'.bin',Buffer.concat(words)],[kind+'.json',JSON.stringify(metadata)+'\n']]){await writeFile(new URL(file,out),data);manifest.push({file,source:'examples/'+source,source_sha256:sha(html),sha256:sha(data)});}
}
for(const [source,file]of [['textures/sprites/disc.png','disc.png'],['fonts/LICENSE','LICENSE-HELVETIKER'],['fonts/MPLUSRounded1c/OFL.txt','LICENSE-MPLUS']]){await copyFile(new URL(source,root),new URL(file,out));manifest.push({file,source:'examples/'+source,sha256:sha(await readFile(new URL(file,out)))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
