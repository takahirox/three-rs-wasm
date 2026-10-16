// The shader-geometry examples' textures; the cube's arrays are in the port as the page writes them.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/shader-geometry/',import.meta.url);await mkdir(out,{recursive:true});
const sha=x=>createHash('sha256').update(x).digest('hex'),manifest=[];
for(const [file,path] of [['crate.gif','textures/crate.gif'],['checker.jpg','textures/floors/FloorsCheckerboard_S_Diffuse.jpg'],['grass.jpg','textures/terrain/grasslight-big.jpg']]){await copyFile(new URL(path,root),new URL(file,out));manifest.push({file,source:'examples/'+path,sha256:sha(await readFile(new URL(file,out)))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
