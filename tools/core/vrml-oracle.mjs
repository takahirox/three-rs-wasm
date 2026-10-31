// Oracle for the VRMLLoader port: the pinned r187 VRMLLoader's parse of every
// VRML asset webgl_loader_vrml loads, in the canonical form with full buffers
// ( tools/core/vrml-canonical.mjs ), written to .cache/vrml-oracle. Never
// loaded by the app; tests/vrml.rs compares the Rust parse against it.
import {readFile,writeFile,mkdir} from 'node:fs/promises';
import {registerHooks} from 'node:module';
const root=new URL('../../',import.meta.url);
const three=new URL('.cache/three-r187/src/Three.Core.js',root).href;
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:three,shortCircuit:true};return next(specifier,context);}});
globalThis.window={innerWidth:1,innerHeight:1};
const {VRMLLoader}=await import(new URL('.cache/three-r187/examples/jsm/loaders/VRMLLoader.js',root).href);
const {TextureLoader,Texture,Object3D,MeshBasicMaterial}=await import(three);
const {canonical}=await import(new URL('tools/core/vrml-canonical.mjs',root).href);
// TextureLoader builds an image element; the stub keeps the requested URL.
TextureLoader.prototype.load=function(url){const t=new Texture();t.userData.referenceUrl=(this.path??'')+url;return t;};
const assets=['creaseAngle','crystal','house','elevationGrid1','elevationGrid2','extrusion1','extrusion2','extrusion3','lines','linesTransparent','meshWithLines','meshWithTexture','pixelTexture','points','camera','multilineString'];
const out=new URL('.cache/vrml-oracle/',root);
await mkdir(out,{recursive:true});
for(const asset of assets){
 const text=await readFile(new URL(`.cache/three-r187/examples/models/vrml/${asset}.wrl`,root),'utf8');
 const bases={object:new Object3D().id+1,material:new MeshBasicMaterial().id+1};
 const scene=new VRMLLoader().parse(text,'models/vrml/');
 scene.updateMatrixWorld(true);
 await writeFile(new URL(`${asset}.json`,out),canonical(scene,bases,true));
 console.log(asset,scene.children.length);
}
