// Reproducible, offline conversion of the pinned UltraHDR using its official decoder.
// Start tools/serve.py first. Requires the pinned reference prepared by prepare_reference.py.
import {chromium} from '@playwright/test';
import {writeFileSync} from 'node:fs';
import {createHash} from 'node:crypto';
import config from '../playwright.config.js';
const browser=await chromium.launch(config.use.launchOptions);
try {
 const page=await browser.newPage();await page.goto('http://127.0.0.1:8173/reference/three-js/gltf-pbr.html');await page.waitForFunction(()=>window.reference);
 const hdr=await page.evaluate(async()=>{
  const {DataUtils}=await import('/.cache/three-r186/src/Three.WebGPU.js');const {data,width,height}=window.reference.environment.image;
  const bytes=[];let maximumRelativeError=0;
  for(let i=0;i<data.length;i+=4){const rgb=[0,1,2].map(c=>DataUtils.fromHalfFloat(data[i+c]));const max=Math.max(...rgb);if(max<1e-32){bytes.push(0,0,0,0);continue;}
   const exponent=Math.floor(Math.log2(max))+1;const scale=256/2**exponent;const rgbe=rgb.map(c=>Math.floor(c*scale));
   for(let c=0;c<3;c++)maximumRelativeError=Math.max(maximumRelativeError,Math.abs(rgb[c]-rgbe[c]/scale)/Math.max(max,1e-32));bytes.push(...rgbe,exponent+128);
  }
  // Radiance scanline RLE, with independent byte channels.
  const result=Array.from(new TextEncoder().encode(`#?RADIANCE\n# Converted from Three.js r186 UltraHDRLoader; see environments.json\nFORMAT=32-bit_rle_rgbe\n\n-Y ${height} +X ${width}\n`));
  for(let y=0;y<height;y++){result.push(2,2,width>>8,width&255);for(let c=0;c<4;c++){
   const row=Array.from({length:width},(_,x)=>bytes[(y*width+x)*4+c]);let i=0;
   while(i<width){let run=1;while(i+run<width&&run<127&&row[i+run]===row[i])run++;if(run>=4){result.push(128+run,row[i]);i+=run;}else{
    const start=i;i+=run;while(i<width&&i-start<128){let next=1;while(i+next<width&&next<4&&row[i+next]===row[i])next++;if(next>=4)break;i+=Math.min(next,128-(i-start));}
    result.push(i-start,...row.slice(start,i));
   }}
  }}
  let binary='';for(let i=0;i<result.length;i+=32768)binary+=String.fromCharCode(...result.slice(i,i+32768));return {data:btoa(binary),width,height,maximumRelativeError};
 });
 const bytes=Buffer.from(hdr.data,'base64');const output='web/environments/royal_esplanade_2k.hdr';writeFileSync(output,bytes);
 console.log(JSON.stringify({output,width:hdr.width,height:hdr.height,maximumRelativeError:hdr.maximumRelativeError,sha256:createHash('sha256').update(bytes).digest('hex')},null,2));
} finally {await browser.close();}
