// The shapes examples' sprite and font notices; their geometry is built at load.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r187/examples/',import.meta.url),out=new URL('../../web/examples/assets/shapes/',import.meta.url);await mkdir(out,{recursive:true});
const sha=x=>createHash('sha256').update(x).digest('hex'),manifest=[];
for(const [source,file]of [['textures/sprites/disc.png','disc.png'],['fonts/LICENSE','LICENSE-HELVETIKER'],['fonts/MPLUSRounded1c/OFL.txt','LICENSE-MPLUS'],['fonts/MPLUSRounded1c/MPLUSRounded1c-Regular.typeface.json','MPLUSRounded1c-Regular.typeface.json']]){await copyFile(new URL(source,root),new URL(file,out));manifest.push({file,source:'examples/'+source,sha256:sha(await readFile(new URL(file,out)))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
