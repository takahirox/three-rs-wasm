import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/shader-geometry/',import.meta.url);await mkdir(out,{recursive:true});
const html=await readFile(new URL('webgl_buffergeometry_instancing_interleaved.html',root),'utf8');
const array=pattern=>Function(`return [${html.match(pattern)[1]}]`)();
const vertices=array(/new Float32Array\( \[([\s\S]*?)\] \), 8/),indices=array(/new Uint16Array\( \[([\s\S]*?)\] \)/);
const cube=JSON.stringify({vertices,indices})+'\n';await writeFile(new URL('cube.json',out),cube);
const sha=x=>createHash('sha256').update(x).digest('hex');const manifest=[{file:'cube.json',source:'examples/webgl_buffergeometry_instancing_interleaved.html',source_sha256:sha(html),sha256:sha(cube)}];
for(const [file,path] of [['crate.gif','textures/crate.gif'],['checker.jpg','textures/floors/FloorsCheckerboard_S_Diffuse.jpg'],['grass.jpg','textures/terrain/grasslight-big.jpg']]){await copyFile(new URL(path,root),new URL(file,out));manifest.push({file,source:'examples/'+path,sha256:sha(await readFile(new URL(file,out)))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
