// The original files the geometry and material examples load at run time.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r187/examples/',import.meta.url),out=new URL('../../web/examples/assets/geometry-materials/',import.meta.url);
const sha=x=>createHash('sha256').update(x).digest('hex'),manifest=[];
for(const path of ['models/json/WaltHeadLo_buffergeometry.json','models/json/pressure.json']){const dest=new URL(path,out);await mkdir(new URL('.',dest),{recursive:true});await copyFile(new URL(path,root),dest);manifest.push({file:path,source:'examples/'+path,sha256:sha(await readFile(dest))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
