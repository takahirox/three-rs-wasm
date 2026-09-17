// Generate independent oracle data from pinned Three.js; never loaded by the app.
import * as T from '../../.cache/three-r186/src/Three.Core.js';

import {writeFileSync,readFileSync} from 'node:fs';
const parametricSource=readFileSync('.cache/three-r186/examples/jsm/geometries/ParametricGeometry.js','utf8').replace(/from 'three'/g,`from '${new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href}'`);
const {ParametricGeometry}=await import('data:text/javascript;base64,'+Buffer.from(parametricSource).toString('base64'));
const points=Array.from({length:8},(_,i)=>new T.Vector2(Math.sin(i*.2)*Math.sin(i*.1)*15+50,(i-5)*2));
const geometries={
 box:new T.BoxGeometry(2,3,4,2,3,4),capsule:new T.CapsuleGeometry(2,3,4,7,2),
 icosahedron:new T.IcosahedronGeometry(3,0),icosphere:new T.IcosahedronGeometry(3,2),octahedron:new T.OctahedronGeometry(3,0),tetrahedron:new T.TetrahedronGeometry(3,0),
 circle:new T.CircleGeometry(3,7,.2,4),ring:new T.RingGeometry(1,3,7,3,.2,4),
 torus:new T.TorusGeometry(3,.7,7,9,4,.3,5),knot:new T.TorusKnotGeometry(3,.7,12,7,2,3),
 lathe:new T.LatheGeometry(points,7,.2,4),cylinder:new T.CylinderGeometry(1,3,4,7,3,false,.2,4),
 cone:new T.CylinderGeometry(0,3,4,7,3,false,.2,4),
 parametric:new ParametricGeometry((u,v,out)=>out.set(u,Math.sin(u*4)*Math.cos(v*3),v),7,5)
};
const result=Object.fromEntries(Object.entries(geometries).map(([name,g])=>[name,{attributes:Object.fromEntries(Object.entries(g.attributes).map(([key,a])=>[key,Array.from(a.array)])),index:g.index?Array.from(g.index.array):null,groups:g.groups}]));
writeFileSync('tests/fixtures/procedural-geometries.json',JSON.stringify(result));
