// The primitives examples' textures; their lines and helper are built at load.
import {mkdir,writeFile,copyFile,readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r187/examples/',import.meta.url),out=new URL('../../web/examples/assets/tsl-primitives/',import.meta.url);await mkdir(out,{recursive:true});
const files=['textures/alphaMap.jpg','textures/ktx2/2d_uastc.ktx2'];
const manifest=[];for(const source of files){const file=source.split('/').at(-1);await copyFile(new URL(source,root),new URL(file,out));manifest.push({file,source,sha256:createHash('sha256').update(await readFile(new URL(file,out))).digest('hex')});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
