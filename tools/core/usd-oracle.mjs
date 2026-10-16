// Oracle for the USDLoader port: the pinned r186 USDLoader's parse of
// webgl_loader_usdz's saeukkang.usdz, the fixtures of tools/core/usd-fixtures.mjs
// and, when present, macOS's own USDC files ( crate layers the repository
// cannot ship ), in the canonical form with full buffers
// ( tools/core/usd-canonical.mjs ), written to .cache/usd-oracle. Never loaded
// by the app; tests/usd.rs compares the Rust parse against it.
import {readFile,writeFile,mkdir,access} from 'node:fs/promises';
import {registerHooks} from 'node:module';
const root=new URL('../../',import.meta.url);
const three=new URL('.cache/three-r186/src/Three.Core.js',root).href;
registerHooks({resolve(specifier,context,next){if(specifier==='three')return {url:three,shortCircuit:true};return next(specifier,context);}});
// The composer decodes images through object URLs and Image elements: keep
// the bytes, and load every image.
const blobs=new Map();
URL.createObjectURL=blob=>{const url=`blob:oracle/${blobs.size}`;blobs.set(url,blob);return url;};
URL.revokeObjectURL=()=>{};
globalThis.Image=class{getAttribute(){return this._src;}set src(v){this._src=v;setTimeout(()=>this.onload?.(),0);}get src(){return this._src;}};
const {USDLoader}=await import(new URL('.cache/three-r186/examples/jsm/loaders/USDLoader.js',root).href);
const {canonical}=await import(new URL('tools/core/usd-canonical.mjs',root).href);
const {fixtures}=await import(new URL('tools/core/usd-fixtures.mjs',root).href);
export const SYSTEM_USDC=['/System/Applications/Chess.app/Contents/Resources/Meshes/bishop.usdc','/System/Applications/Chess.app/Contents/Resources/Meshes/king.usdc','/System/Library/ExtensionKit/Extensions/WallpaperSequoiaExtension.appex/Contents/Resources/mesh.usdc'];
const inputs=fixtures().map(f=>({name:f.name,path:f.path,bytes:f.bytes}));
const saeukkang=await readFile(new URL('.cache/three-r186/examples/models/usdz/saeukkang.usdz',root));
inputs.push({name:'saeukkang.usdz',path:'',bytes:new Uint8Array(saeukkang)});
for(const p of SYSTEM_USDC){try{await access(p);inputs.push({name:p.split('/').pop(),path:'',bytes:new Uint8Array(await readFile(p))});}catch{}}
const out=new URL('.cache/usd-oracle/',root);
const fx=new URL('.cache/usd-fixtures/',root);
await mkdir(out,{recursive:true});await mkdir(fx,{recursive:true});
for(const input of inputs){
 await writeFile(new URL(input.name,fx),input.bytes);
 blobs.clear();
 const group=await new Promise((resolve,reject)=>new USDLoader().parse(input.bytes.slice().buffer,input.path,resolve,reject));
 group.updateMatrixWorld(true);
 const images=new Map();for(const [url,blob] of blobs)images.set(url,new Uint8Array(await blob.arrayBuffer()));
 await writeFile(new URL(`${input.name}.json`,out),canonical(group,images,true));
 let count=0;group.traverse(()=>count++);
 console.log(input.name,count,group.animations.length);
}
