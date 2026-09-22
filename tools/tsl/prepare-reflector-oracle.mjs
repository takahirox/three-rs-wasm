// Execute pinned ReflectorNode camera updates without rasterizing a scene.
import {writeFile} from 'node:fs/promises';
import * as THREE from '../../.cache/three-r186/src/Three.WebGPU.js';
import {reflector} from '../../.cache/three-r186/src/nodes/utils/ReflectorNode.js';
const camera=new THREE.PerspectiveCamera(45,1.5,1,500);camera.coordinateSystem=THREE.WebGPUCoordinateSystem;camera.position.set(0,75,160);camera.lookAt(0,40,0);camera.updateMatrixWorld();camera.updateProjectionMatrix();
const floor=new THREE.Object3D();floor.rotation.x=-Math.PI/2;floor.updateMatrixWorld();const wall=new THREE.Object3D();wall.position.set(0,50,-50);wall.updateMatrixWorld();
const scene=new THREE.Scene();let output;
const renderer={coordinateSystem:THREE.WebGPUCoordinateSystem,autoClear:true,getDrawingBufferSize:v=>v.set(512,512),getRenderTarget:()=>null,getMRT:()=>null,setMRT(){},setRenderTarget(){},clear(){},render(s,c){output=c.clone();}};
const results=[];
for(const targets of [[floor,wall],[wall,floor]]){
 let c=camera;const row=[];
 for(const target of targets){const node=reflector({target}).reflector;node.updateBefore({scene,camera:c,renderer,material:{visible:true}});c=output;row.push({matrix:c.matrixWorld.toArray(),projection:c.projectionMatrix.toArray()});}
 results.push(row);
}
await writeFile(new URL('../../tests/fixtures/reflector-cameras-r186.json',import.meta.url),JSON.stringify(results,null,2)+'\n');
