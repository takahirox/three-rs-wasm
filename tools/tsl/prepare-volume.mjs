// One-time static data preparation, matching the official CPU initialization.
import {readFileSync} from 'node:fs';
const source=readFileSync(new URL('../../.cache/three-r186/examples/jsm/math/ImprovedNoise.js',import.meta.url),'utf8').replace("import { MathUtils } from 'three';",'const MathUtils={lerp:(x,y,t)=>(1-t)*x+t*y};');
const {ImprovedNoise}=await import('data:text/javascript;base64,'+Buffer.from(source).toString('base64'));
import {writeFileSync} from 'node:fs';
const noise=new ImprovedNoise(),size=128;
for(const kind of ['perlin','cloud']){
 const data=new Uint8Array(size**3);let i=0;
 for(let z=0;z<size;z++)for(let y=0;y<size;y++)for(let x=0;x<size;x++){
  const d=1-Math.sqrt(((x-64)/128)**2+((y-64)/128)**2+((z-64)/128)**2);
  data[i++]=kind==='perlin'?noise.noise(x/128*6.5,y/128*6.5,z/128*6.5)*128+128:(128+128*noise.noise(x*.05/1.5,y*.05,z*.05/1.5))*d*d;
 }
 writeFileSync(new URL(`../../web/gallery/assets/volume-${kind}.raw`,import.meta.url),data);
}
