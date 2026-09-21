// Evaluate static r186 path samples once; GPU shaders animate the instances.
import {readFileSync,writeFileSync} from 'node:fs';
import {Path} from '../../.cache/three-r186/src/Three.Core.js';
const source=readFileSync(new URL('../../.cache/three-r186/examples/webgpu_instance_path.html',import.meta.url),'utf8');
const expr=source.match(/const path = ([\s\S]*?);/)[1].replace('THREE.Path','Path');
const path=new Function('Path',`const x=0,y=0;return ${expr}`)(Path);
writeFileSync(new URL('../../web/gallery/assets/tsl-next/path.json',import.meta.url),JSON.stringify(Array.from({length:1000},(_,i)=>path.getPointAt(i/1000).toArray()))+'\n');
