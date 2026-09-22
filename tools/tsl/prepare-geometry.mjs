// Static initial geometry from the pinned MIT-licensed Three.js implementation.
import {readFileSync,writeFileSync,mkdirSync} from 'node:fs';
import {createHash} from 'node:crypto';
const url=new URL('../../.cache/three-r186/src/Three.Core.js',import.meta.url).href;
const source=readFileSync(new URL('../../.cache/three-r186/examples/jsm/geometries/TeapotGeometry.js',import.meta.url),'utf8').replace("from 'three'",`from '${url}'`);
const {TeapotGeometry}=await import('data:text/javascript;base64,'+Buffer.from(source).toString('base64'));
const geometry=new TeapotGeometry(.8,18);
const data={index:Array.from(geometry.index.array),attributes:Object.fromEntries(Object.entries(geometry.attributes).map(([k,a])=>[k,{size:a.itemSize,array:Array.from(a.array)}]))};
writeFileSync(new URL('../../web/gallery/assets/teapot-18.json',import.meta.url),JSON.stringify(data)+'\n');

const large=new TeapotGeometry(50,18);
writeFileSync(new URL('../../web/gallery/assets/teapot-50-18.json',import.meta.url),JSON.stringify({index:Array.from(large.index.array),attributes:Object.fromEntries(Object.entries(large.attributes).map(([k,a])=>[k,{size:a.itemSize,array:Array.from(a.array)}]))})+'\n');
const {DodecahedronGeometry}=await import(url);
const dodeca=new DodecahedronGeometry(.5);
writeFileSync(new URL('../../web/gallery/assets/dodecahedron.json',import.meta.url),JSON.stringify(Object.fromEntries(['position','normal','uv'].map(k=>[k,Array.from(dodeca.attributes[k].array)])))+'\n');
const utilsSource=readFileSync(new URL('../../.cache/three-r186/examples/jsm/utils/GeometryUtils.js',import.meta.url),'utf8').replace("from 'three'",`from '${url}'`);
const {hilbert3D}=await import('data:text/javascript;base64,'+Buffer.from(utilsSource).toString('base64'));
const {Vector3,CatmullRomCurve3,Color,SRGBColorSpace}=await import(url);
const curve=new CatmullRomCurve3(hilbert3D(new Vector3(),20,1,0,1,2,3,4,5,6,7));
const positions=[],colors=[];for(let i=0;i<256;i++){positions.push([...curve.getPoint(i/256).toArray(),0]);colors.push([...new Color().setHSL(i/256,1,.5,SRGBColorSpace).toArray(),1]);}
writeFileSync(new URL('../../web/gallery/assets/hilbert-points.json',import.meta.url),JSON.stringify({positions,colors})+'\n');

mkdirSync(new URL('../../web/gallery/assets/tsl-procedural/',import.meta.url),{recursive:true});
const snow=new TeapotGeometry(.5,18);
const snowData=JSON.stringify({index:Array.from(snow.index.array),attributes:Object.fromEntries(Object.entries(snow.attributes).map(([k,a])=>[k,{size:a.itemSize,array:Array.from(a.array)}]))})+'\n';
writeFileSync(new URL('../../web/gallery/assets/tsl-procedural/snow-teapot.json',import.meta.url),snowData);
const hash=x=>createHash('sha256').update(x).digest('hex');
writeFileSync(new URL('../../web/gallery/assets/tsl-procedural/snow-teapot-manifest.json',import.meta.url),JSON.stringify({file:'snow-teapot.json',sha256:hash(snowData),source:'examples/jsm/geometries/TeapotGeometry.js',source_sha256:hash(readFileSync(new URL('../../.cache/three-r186/examples/jsm/geometries/TeapotGeometry.js',import.meta.url)))},null,2)+'\n');
