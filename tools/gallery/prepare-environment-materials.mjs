import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/examples/assets/environment-materials/',import.meta.url);await mkdir(out,{recursive:true});const manifest=[],sha=x=>createHash('sha256').update(x).digest('hex');
const paths=JSON.parse(await readFile(new URL('./environment-material-paths.json',import.meta.url),'utf8'));
for(const path of paths){const dest=new URL(path,out);await mkdir(new URL('.',dest),{recursive:true});await copyFile(new URL(path,root),dest);manifest.push({file:path,source:'examples/'+path,sha256:sha(await readFile(dest))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
