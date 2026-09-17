import {test,expect} from '@playwright/test';
import {readFileSync,writeFileSync} from 'node:fs';
import {PNG} from 'pngjs';
for(const name of ['lambert','phong','flat','normal','physical','clearcoat','sheen','anisotropy','ior','area','toon','matcap','depth','transmission','iridescence','physical_maps'])test(`Core material matches original Three.js: ${name}`,async({page},info)=>{
 const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto(`/reference/three-js/core-materials.html?material=${name}`);await page.waitForFunction(()=>window.ready);expect(errors).toEqual([]);
 const reference=await page.locator('canvas').screenshot(),actual=readFileSync(`.cache/core-materials/${name}.png`);
 const a=PNG.sync.read(actual),b=PNG.sync.read(reference);let differing=0,square=0,maximum=0;
 for(let i=0;i<a.data.length;i+=4){let different=false;for(let c=0;c<3;c++){const d=Math.abs(a.data[i+c]-b.data[i+c]);different||=d>6;square+=d*d;maximum=Math.max(maximum,d);}if(different)differing++;}
 const metrics={fraction:differing/(256*256),rms:Math.sqrt(square/(256*256*3)),maximum};
 writeFileSync(info.outputPath('comparison.json'),JSON.stringify(metrics));writeFileSync(info.outputPath('actual.png'),actual);writeFileSync(info.outputPath('reference.png'),reference);
 expect(metrics.fraction).toBeLessThan(0.025);expect(metrics.rms).toBeLessThan(4);
});
