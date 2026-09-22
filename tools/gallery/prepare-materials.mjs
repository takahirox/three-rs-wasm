// Static geometry from the pinned r186 paper-model example; no runtime Three.js.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import * as THREE from '../../.cache/three-r186/src/Three.Core.js';
const root=new URL('../../',import.meta.url);
const source=await readFile(new URL('.cache/three-r186/examples/webgpu_materials_arrays.html',root),'utf8');
const fn=source.slice(source.indexOf('function makeHoleyGeometry('),source.indexOf('function onWindowResize()'));
const make=new Function('THREE','materials','_a','_b',`${fn};return makeHoleyGeometry;`)(THREE,Array(6),new THREE.Vector3(),new THREE.Vector3());
const inputs=[[new THREE.TetrahedronGeometry(1.2),3],[new THREE.BoxGeometry(1.5,1.5,1.5),6],[new THREE.OctahedronGeometry(1.2),3],[new THREE.DodecahedronGeometry(1.1),9],[new THREE.IcosahedronGeometry(1.1),3]];
const geometries=inputs.map(([g,n])=>{const h=make(g,n);h.computeBoundingBox();return {position:Array.from(h.attributes.position.array),normal:Array.from(h.attributes.normal.array),groups:h.groups,minY:h.boundingBox.min.y};});
const out=new URL('web/gallery/assets/material-textures/',root);await mkdir(out,{recursive:true});await writeFile(new URL('paper-models.json',out),JSON.stringify(geometries)+'\n');
// Preserve Canvas 2D's fractional coverage of the final 1x1 mip.
const {chromium}=await import('@playwright/test');
const browser=await chromium.launch({channel:'chrome',headless:true});
try {const page=await browser.newPage();const pixels=await page.evaluate(()=>{
 const bytes=[];const colors=['#f00','#0f0','#00f','#400','#040','#004','#044','#404'];
 for(let i=0;i<8;i++){const size=128>>i,c=document.createElement('canvas');c.width=c.height=size;const x=c.getContext('2d');x.fillStyle='#444';x.fillRect(0,0,size,size);x.fillStyle=colors[i];x.fillRect(0,0,size/2,size/2);x.fillRect(size/2,size/2,size/2,size/2);bytes.push(...x.getImageData(0,0,size,size).data);}return bytes;
});await writeFile(new URL('manual-mips.rgba',out),Buffer.from(pixels));}finally{await browser.close();}
