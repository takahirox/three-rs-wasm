// Oracle for the TextGeometry, ExtrudeGeometry and RoundedBoxGeometry ports:
// the pinned r186 geometries the examples build, written to
// .cache/shapes-oracle.json. tests/shapes.rs compares the Rust ports.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {registerHooks} from 'node:module';
const root=new URL('../../',import.meta.url);
const three=new URL('.cache/three-r186/src/Three.Core.js',root).href;
registerHooks({resolve(s,c,n){if(s==='three')return {url:three,shortCircuit:true};return n(s,c);}});
const {FontLoader}=await import(new URL('.cache/three-r186/examples/jsm/loaders/FontLoader.js',root).href);
const {TextGeometry}=await import(new URL('.cache/three-r186/examples/jsm/geometries/TextGeometry.js',root).href);
const {NURBSCurve}=await import(new URL('.cache/three-r186/examples/jsm/curves/NURBSCurve.js',root).href);
const {NURBSSurface}=await import(new URL('.cache/three-r186/examples/jsm/curves/NURBSSurface.js',root).href);
const {NURBSVolume}=await import(new URL('.cache/three-r186/examples/jsm/curves/NURBSVolume.js',root).href);
const {ParametricGeometry}=await import(new URL('.cache/three-r186/examples/jsm/geometries/ParametricGeometry.js',root).href);
const BufferGeometryUtils=await import(new URL('.cache/three-r186/examples/jsm/utils/BufferGeometryUtils.js',root).href);
const THREE=await import(three);
const {RoundedBoxGeometry}=await import(new URL('.cache/three-r186/examples/jsm/geometries/RoundedBoxGeometry.js',root).href);
const font=async name=>new FontLoader().parse(JSON.parse(await readFile(new URL(`.cache/three-r186/examples/fonts/${name}.typeface.json`,root),'utf8')));
const gentilis=await font('gentilis_regular'),helvetiker=await font('helvetiker_regular'),optimer=await font('optimer_bold');
const b64=a=>Buffer.from(a.buffer,a.byteOffset,a.byteLength).toString('base64');
const dump=g=>({attributes:Object.fromEntries(Object.entries(g.attributes).map(([k,a])=>[k,{itemSize:a.itemSize,data:b64(a.array)}])),groups:g.groups.map(x=>[x.start,x.count,x.materialIndex]),index:g.index?b64(new Uint32Array(g.index.array)):null});
const cases=[];
for(const text of ['-gradientMap','+gradientMap','-diffuse','+diffuse'])cases.push({kind:'text',font:'gentilis_regular',text,size:20,depth:1,curveSegments:1,geometry:dump(new TextGeometry(text,{font:gentilis,size:20,depth:1,curveSegments:1}))});
for(const text of ['Noise','2D','Worley','3D style 1','Unified Fractal','checkerboard'])cases.push({kind:'text',font:'helvetiker_regular',text,size:1.15,depth:.02,curveSegments:2,geometry:dump(new TextGeometry(text,{font:helvetiker,size:1.15,depth:.02,curveSegments:2}))});
for(const text of ['raw','semigloss','white_oak','mahogany','custom'])cases.push({kind:'text',font:'helvetiker_regular',text,size:.1,depth:.001,curveSegments:12,geometry:dump(new TextGeometry(text,{font:helvetiker,size:.1,depth:.001,curveSegments:12,bevelEnabled:false}))});
cases.push({kind:'text',font:'optimer_bold',text:'three.js',size:70,depth:20,curveSegments:4,bevel:[2,1.5,3],geometry:dump(new TextGeometry('three.js',{font:optimer,size:70,depth:20,curveSegments:4,bevelThickness:2,bevelSize:1.5,bevelEnabled:true}))});
// The wood labels, centered on their bounding box as the page centers them.
for(const text of ['gloss','cherry']){const g=new TextGeometry(text,{font:helvetiker,size:.1,depth:.001,curveSegments:12,bevelEnabled:false});g.computeBoundingBox();const b=g.boundingBox;g.translate(-.5*(b.max.x-b.min.x),-.5*(b.max.y-b.min.y),-.5*(b.max.z-b.min.z));cases.push({kind:'text',font:'helvetiker_regular',text,size:.1,depth:.001,curveSegments:12,center:true,geometry:dump(g)});}
cases.push({kind:'roundedBox',args:[.125,.9,.9,10,.02],geometry:dump(new RoundedBoxGeometry(.125,.9,.9,10,.02))});
cases.push({kind:'roundedBox',args:[1,2,3,2,.25],geometry:dump(new RoundedBoxGeometry(1,2,3,2,.25))});
// webgl_geometry_shapes: each shape's ShapeGeometry, ExtrudeGeometry, getPoints
// and getSpacedPoints( 50 ) with autoClose.
const pts=a=>b64(new Float64Array(a.flatMap(p=>[p.x,p.y])));
const shapePage=(()=>{
 const californiaPts=[[610,320],[450,300],[392,392],[266,438],[190,570],[190,600],[160,620],[160,650],[180,640],[165,680],[150,670],[90,737],[80,795],[50,835],[64,870],[60,945],[300,945],[300,743],[600,473],[626,425],[600,370],[610,320]].map(([x,y])=>new THREE.Vector2(x,y).multiplyScalar(.25));
 const S=()=>new THREE.Shape();const x=0,y=0;
 const shapes={california:new THREE.Shape(californiaPts),
  triangle:S().moveTo(80,20).lineTo(40,80).lineTo(120,80).lineTo(80,20),
  heart:S().moveTo(x+25,y+25).bezierCurveTo(x+25,y+25,x+20,y,x,y).bezierCurveTo(x-30,y,x-30,y+35,x-30,y+35).bezierCurveTo(x-30,y+55,x-10,y+77,x+25,y+95).bezierCurveTo(x+60,y+77,x+80,y+55,x+80,y+35).bezierCurveTo(x+80,y+35,x+80,y,x+50,y).bezierCurveTo(x+35,y,x+25,y+25,x+25,y+25),
  square:S().moveTo(0,0).lineTo(0,80).lineTo(80,80).lineTo(80,0).lineTo(0,0),
  track:S().moveTo(40,40).lineTo(40,160).absarc(60,160,20,Math.PI,0,true).lineTo(80,40).absarc(60,40,20,2*Math.PI,Math.PI,true),
  fish:S().moveTo(x,y).quadraticCurveTo(x+50,y-80,x+90,y-10).quadraticCurveTo(x+100,y-10,x+115,y-40).quadraticCurveTo(x+115,y,x+115,y+40).quadraticCurveTo(x+100,y+10,x+90,y+10).quadraticCurveTo(x+50,y+80,x,y),
  spline:S().moveTo(0,0).splineThru([new THREE.Vector2(70,20),new THREE.Vector2(80,90),new THREE.Vector2(-30,70),new THREE.Vector2(0,0)])};
 const smiley=S().moveTo(80,40).absarc(40,40,40,0,Math.PI*2,false);
 smiley.holes.push(new THREE.Path().moveTo(35,20).absellipse(25,20,10,10,0,Math.PI*2,true));
 smiley.holes.push(new THREE.Path().moveTo(65,20).absarc(55,20,10,0,Math.PI*2,true));
 smiley.holes.push(new THREE.Path().moveTo(20,40).quadraticCurveTo(40,60,60,40).bezierCurveTo(70,45,70,50,60,60).quadraticCurveTo(40,80,20,60).quadraticCurveTo(5,50,20,40));
 shapes.smiley=smiley;
 const out=[];
 for(const [name,shape] of Object.entries(shapes)){
  const flat=dump(new THREE.ShapeGeometry(shape));
  const extruded=dump(new THREE.ExtrudeGeometry(shape,{depth:8,bevelEnabled:true,bevelSegments:2,steps:2,bevelSize:1,bevelThickness:1}));
  shape.autoClose=true;
  out.push({kind:'shape',name,flat,extruded,points:pts(shape.getPoints()),spaced:pts(shape.getSpacedPoints(50)),holes:shape.holes.map(h=>{h.autoClose=true;return {points:pts(h.getPoints()),spaced:pts(h.getSpacedPoints(50))};})});
 }
 return out;
})();
cases.push(...shapePage);
// webgl_geometry_text_shapes / text_stroke: ShapeGeometry of the text, and the
// shapes' outlines and holes.
const mplus=await font('MPLUSRounded1c/MPLUSRounded1c-Regular');
for(const [f,name,text,size,dir] of [[helvetiker,'helvetiker_regular','   Three.js\nSimple text.',100,'ltr'],[mplus,'MPLUSRounded1c/MPLUSRounded1c-Regular','   Three.js\nStroke text.',80,'ltr'],[mplus,'MPLUSRounded1c/MPLUSRounded1c-Regular','טקסט קו',80,'rtl'],[mplus,'MPLUSRounded1c/MPLUSRounded1c-Regular','文字描邊',80,'tb']]){
 const shapes=f.generateShapes(text,size,dir);const g=new THREE.ShapeGeometry(shapes);
 const outlines=[];for(const s of shapes){outlines.push(pts(s.getPoints()));for(const h of s.holes)outlines.push(pts(h.getPoints()));}
 cases.push({kind:'textShapes',font:name,text,size,direction:dir,geometry:dump(g),outlines});
}
// webgl_geometry_nurbs: the curve with the gallery's seeded Math.random, and the
// surface and volume ParametricGeometry.
{let seed=186;const random=()=>((seed=(Math.imul(seed,1664525)+1013904223)>>>0)/4294967296);
 const cp=[],knots=[];const degree=3;for(let i=0;i<=degree;i++)knots.push(0);
 for(let i=0,j=20;i<j;i++){cp.push(new THREE.Vector4(random()*400-200,random()*400,random()*400-200,1));knots.push(THREE.MathUtils.clamp((i+1)/(j-degree),0,1));}
 const curve=new NURBSCurve(degree,knots,cp);
 cases.push({kind:'nurbsCurve',points:b64(new Float64Array(curve.getPoints(200).flatMap(p=>[p.x,p.y,p.z]))),control:b64(new Float64Array(cp.flatMap(p=>[p.x,p.y,p.z,p.w]))),knots});
 const V=(x,y,z,w)=>new THREE.Vector4(x,y,z,w);
 const surface=new NURBSSurface(2,3,[0,0,0,1,1,1],[0,0,0,0,1,1,1,1],[[V(-200,-200,100,1),V(-200,-100,-200,1),V(-200,100,250,1),V(-200,200,-100,1)],[V(0,-200,0,1),V(0,-100,-100,5),V(0,100,150,5),V(0,200,0,1)],[V(200,-200,-100,1),V(200,-100,200,1),V(200,100,-250,1),V(200,200,100,1)]]);
 cases.push({kind:'nurbsSurface',geometry:dump(new ParametricGeometry((u,v,t)=>surface.getPoint(u,v,t),20,20))});
 const vol=new NURBSVolume(2,3,1,[0,0,0,1,1,1],[0,0,0,0,1,1,1,1],[0,0,1,1],[[[V(-200,-200,-200,1),V(-200,-200,200,1)],[V(-200,-100,-200,1),V(-200,-100,200,1)],[V(-200,100,-200,1),V(-200,100,200,1)],[V(-200,200,-200,1),V(-200,200,200,1)]],[[V(0,-200,-200,1),V(0,-200,200,1)],[V(0,-100,-200,1),V(0,-100,200,1)],[V(0,100,-200,1),V(0,100,200,1)],[V(0,200,-200,1),V(0,200,200,1)]],[[V(200,-200,-200,1),V(200,-200,200,1)],[V(200,-100,0,1),V(200,-100,100,1)],[V(200,100,0,1),V(200,100,100,1)],[V(200,200,0,1),V(200,200,100,1)]]]);
 const faces={front:(u,v,t)=>vol.getPoint(u,v,0,t),middle:(u,v,t)=>vol.getPoint(u,v,.5,t),back:(u,v,t)=>vol.getPoint(u,v,1,t),top:(u,w,t)=>vol.getPoint(u,1,w,t),side:(v,w,t)=>vol.getPoint(0,v,w,t)};
 for(const [face,f] of Object.entries(faces))cases.push({kind:'nurbsVolume',face,geometry:dump(new ParametricGeometry(f,20,20))});
}
// webgl_geometry_convex: mergeVertices of the bare DodecahedronGeometry( 10 ).
{const d=new THREE.DodecahedronGeometry(10);d.deleteAttribute('normal');d.deleteAttribute('uv');cases.push({kind:'mergeVertices',geometry:dump(BufferGeometryUtils.mergeVertices(d))});}
// webgpu_instance_path's path, sampled by getPointAt; the wireframe of an icosphere.
{const x=0,y=0;const path=new THREE.Path().moveTo(x-2.5,y-2.5).bezierCurveTo(x-2.5,y-2.5,x-2,y,x,y).bezierCurveTo(x+3,y,x+3,y-3.5,x+3,y-3.5).bezierCurveTo(x+3,y-5.5,x+1,y-7.7,x-2.5,y-9.5).bezierCurveTo(x-6,y-7.7,x-8,y-5.5,x-8,y-3.5).bezierCurveTo(x-8,y-3.5,x-8,y,x-5,y).bezierCurveTo(x-3.5,y,x-2.5,y-2.5,x-2.5,y-2.5);
 cases.push({kind:'pathAt',points:pts(Array.from({length:1000},(_,i)=>path.getPointAt(i/1000)))});
 cases.push({kind:'wireframe',geometry:dump(new THREE.WireframeGeometry(new THREE.IcosahedronGeometry(20,1)))});
 cases.push({kind:'wireframeIndexed',geometry:dump(new THREE.WireframeGeometry(new THREE.BoxGeometry(1,2,3,2,2,2)))});}
await mkdir(new URL('.cache/',root),{recursive:true});
await writeFile(new URL('.cache/shapes-oracle.json',root),JSON.stringify(cases));
console.log(cases.length,'cases');
