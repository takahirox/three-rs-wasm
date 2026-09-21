// Static initial attributes from the pinned official geometry generators.
import {mkdir,writeFile,copyFile,readFile} from 'node:fs/promises';
import {registerHooks} from 'node:module';
import {createHash} from 'node:crypto';
registerHooks({resolve(s,c,n){if(s==='three')return {url:new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href,shortCircuit:true};return n(s,c);}});
const THREE=await import('../../.cache/three-r186/src/Three.Core.js');
const {hilbert3D}=await import('../../.cache/three-r186/examples/jsm/utils/GeometryUtils.js');
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/tsl-primitives/',import.meta.url);await mkdir(out,{recursive:true});
const curve=new THREE.CatmullRomCurve3(hilbert3D(new THREE.Vector3(),20,1,0,1,2,3,4,5,6,7));
const positions=[],colors=[];for(let i=0;i<768;i++){positions.push(curve.getPoint(i/768).toArray());colors.push(new THREE.Color().setHSL(i/768,1,.5,THREE.SRGBColorSpace).toArray());}
const wire=new THREE.WireframeGeometry(new THREE.IcosahedronGeometry(20,1));
await writeFile(new URL('lines.json',out),JSON.stringify({positions,colors,wire:Array.from(wire.attributes.position.array)}));
const cam=new THREE.OrthographicCamera(-1.25,1.25,1.25,-1.25,0,.3);cam.rotation.x=Math.PI/2;cam.position.y=-.3;cam.updateMatrixWorld();const helper=new THREE.CameraHelper(cam);helper.geometry.applyMatrix4(cam.matrixWorld);await writeFile(new URL('helper.json',out),JSON.stringify({position:Array.from(helper.geometry.attributes.position.array),color:Array.from(helper.geometry.attributes.color.array)}));
const files=['textures/alphaMap.jpg','textures/ktx2/2d_uastc.ktx2'];
const manifest=[];for(const source of files){const file=source.split('/').at(-1);await copyFile(new URL(source,root),new URL(file,out));manifest.push({file,source,sha256:createHash('sha256').update(await readFile(new URL(file,out))).digest('hex')});}
for(const file of ['lines.json','helper.json'])manifest.push({file,sha256:createHash('sha256').update(await readFile(new URL(file,out))).digest('hex')});await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
