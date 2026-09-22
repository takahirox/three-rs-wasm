import {registerHooks} from 'node:module';
import {writeFile} from 'node:fs/promises';
// Camera-only oracle from pinned Three.js r186 (148ef33e). No rasterization.
const root=new URL('../../.cache/three-r186/',import.meta.url).href;
registerHooks({resolve(s,c,next){if(s==='three')return {url:root+'src/Three.Core.js',shortCircuit:true};return next(s,c);}});
const T=await import(root+'src/Three.Core.js');const {SunLight}=await import(root+'examples/jsm/lights/SunLight.js');
const rows=[];
for(const ortho of [false,true])for(const p of [[6.25,3,4],[0,1,0]]) {
 const camera=ortho?new T.OrthographicCamera(-4,4,3,-3,.1,100):new T.PerspectiveCamera(35,1.5,.1,100);
 camera.coordinateSystem=T.WebGPUCoordinateSystem;camera.position.set(-10,8,-2.2);camera.lookAt(0,0,0);camera.updateProjectionMatrix();camera.updateMatrixWorld();
 const light=new SunLight();light.position.fromArray(p);light.updateMatrixWorld();light.shadow.camera.coordinateSystem=T.WebGPUCoordinateSystem;light.shadow.camera.far=30;light.shadow.updateMatrices(light,camera);
 rows.push({ortho,position:p,camera:camera.matrixWorld.elements,cascades:[0,1].map(i=>({matrix:light.shadow.getCamera(i).projectionMatrix.clone().multiply(light.shadow.getCamera(i).matrixWorldInverse).elements,range:light.shadow._cascadeData[i].toArray()}))});
}
await writeFile(new URL('../../tests/fixtures/sun-cascades-r186.json',import.meta.url),JSON.stringify(rows,null,2)+'\n');
