// The point-cloud examples' sprites; their positions are generated at load.
import {readFile,writeFile,mkdir,copyFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
const root=new URL('../../.cache/three-r186/examples/',import.meta.url),out=new URL('../../web/gallery/assets/point-clouds/',import.meta.url);await mkdir(out,{recursive:true});
const sha=x=>createHash('sha256').update(x).digest('hex'),manifest=[];
for(const name of ['disc.png','ball.png',...Array.from({length:5},(_,i)=>`snowflake${i+1}.png`)]){await copyFile(new URL('textures/sprites/'+name,root),new URL(name,out));manifest.push({file:name,source:'examples/textures/sprites/'+name,sha256:sha(await readFile(new URL(name,out)))});}
await writeFile(new URL('manifest.json',out),JSON.stringify(manifest,null,2)+'\n');
