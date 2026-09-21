// Offline asset conversion with the pinned official UltraHDR decoder. Keeps every
// half-float bit; runtime lighting and PMREM generation are Rust/WebGPU.
import {chromium} from '@playwright/test';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync} from 'node:fs';
import {deflateSync} from 'node:zlib';
import config from '../../playwright.config.js';
function crc32(data) {let crc=0xffffffff;for(const byte of data){crc^=byte;for(let b=0;b<8;b++)crc=(crc>>>1)^((crc&1)?0xedb88320:0);}return (crc^0xffffffff)>>>0;}
function chunk(type,data) {const name=Buffer.from(type),size=Buffer.alloc(4),crc=Buffer.alloc(4);size.writeUInt32BE(data.length);crc.writeUInt32BE(crc32(Buffer.concat([name,data])));return Buffer.concat([size,name,data,crc]);}
function halfPng(image) {const source=Buffer.from(image.data,'base64');source.swap16();const stride=image.width*8;const rows=[];for(let y=0;y<image.height;y++)rows.push(Buffer.from([0]),source.subarray(y*stride,(y+1)*stride));const header=Buffer.alloc(13);header.writeUInt32BE(image.width);header.writeUInt32BE(image.height,4);header[8]=16;header[9]=6;return Buffer.concat([Buffer.from([137,80,78,71,13,10,26,10]),chunk('IHDR',header),chunk('IDAT',deflateSync(Buffer.concat(rows))),chunk('IEND',Buffer.alloc(0))]);}
const browser=await chromium.launch(config.use.launchOptions);
const records=[];
const hash=data=>createHash('sha256').update(data).digest('hex');
try {
 const page=await browser.newPage();
 await page.goto('http://127.0.0.1:8173/reference/three-js/tsl-environment.html?id=webgpu_cubemap_mix');
 for(const name of ['spruit_sunrise_2k.hdr.jpg','ice_planet_close.jpg']) {
  const image=await page.evaluate(async name=>{
   const {UltraHDRLoader}=await import('/.cache/three-r186/examples/jsm/loaders/UltraHDRLoader.js');
   const texture=await new UltraHDRLoader().loadAsync('/web/gallery/assets/tsl-lighting/textures/equirectangular/'+name);
   const {data,width,height}=texture.image;const bytes=new Uint8Array(data.buffer,data.byteOffset,data.byteLength);
   let binary='';for(let i=0;i<bytes.length;i+=32768)binary+=String.fromCharCode(...bytes.subarray(i,i+32768));
   return {width,height,data:btoa(binary)};
  },name);
  const output=halfPng(image);
  writeFileSync('web/gallery/assets/tsl-lighting/'+name+'.rgba16f.png',output);
  records.push({source:'textures/equirectangular/'+name,source_sha256:hash(readFileSync('web/gallery/assets/tsl-lighting/textures/equirectangular/'+name)),file:name+'.rgba16f.png',sha256:hash(output),width:image.width,height:image.height});
  console.log(name,image.width,image.height);
 }
 writeFileSync('web/gallery/assets/tsl-lighting/hdr.json',JSON.stringify(records,null,2)+'\n');
} finally {await browser.close();}
