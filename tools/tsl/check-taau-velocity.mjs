// Diagnose r187's TAAU projection override without modifying its shaders:
// TAAUNode.setViewOffset() hands VelocityNode the unjittered projection
// ( velocity.setProjectionMatrix ) before the pipeline renders, and VelocityNode
// copies it each frame into its currentProjectionMatrix uniform, so the current
// clip is unjittered like the previous one ( the previous frame's current ): a
// stationary view has zero velocity ( docs/taau-r187-velocity.json,
// threeRevision r187's commit ). r186 compiled the current clip with the
// jittered camera projection instead.
import assert from 'node:assert/strict';
import{chromium}from'@playwright/test';import{writeFileSync}from'node:fs';
const browser=await chromium.launch({executablePath:process.env.CHROME_PATH||(process.platform==='darwin'?'/Applications/Google Chrome.app/Contents/MacOS/Google Chrome':undefined),args:['--enable-unsafe-webgpu','--use-angle=default']});const page=await browser.newPage({viewport:{width:512,height:512}});
await page.route('**/tsl-procedural.html?*',async route=>{const res=await route.fetch();let html=await res.text();html=html.replace('await import(URL.createObjectURL',`module=module.replace('const scenePassColor =','window.debugPass=scenePass;window.debugCamera=camera;const scenePassColor =');await import(URL.createObjectURL`);await route.fulfill({response:res,body:html});});
await page.goto((process.env.BASE_URL||'http://127.0.0.1:8173')+'/reference/three-js/tsl-procedural.html?id=webgpu_upscaling_taau');await page.waitForFunction(()=>document.querySelector('canvas')?.dataset.ready==='true');const results=[];
for(let i=0;i<5;i++){await page.evaluate(()=>renderFixture(0));results.push(await page.evaluate(async()=>{const r=reference.renderer,d=r.backend.device,t=r.backend.get(debugPass.renderTarget.textures[1]).texture;const b=d.createBuffer({size:256,usage:GPUBufferUsage.COPY_DST|GPUBufferUsage.MAP_READ});const e=d.createCommandEncoder();e.copyTextureToBuffer({texture:t,origin:{x:128,y:128,z:0}},{buffer:b,bytesPerRow:256},{width:1,height:1,depthOrArrayLayers:1});d.queue.submit([e.finish()]);await b.mapAsync(GPUMapMode.READ);const u=new Uint16Array(b.getMappedRange());const values=Array.from(u.slice(0,4)).map(x=>{const sign=x>>15?-1:1,exp=(x>>10)&31,m=x&1023;return sign*(exp?2**(exp-15)*(1+m/1024):2**(-14)*m/1024);});b.unmap();b.destroy();return {values};}));}
for(const [i,{values}] of results.entries()){const expected=[0,0];for(let c=0;c<2;c++)assert.ok(Math.abs(values[c]-expected[c])<3e-6,JSON.stringify({i,values,expected}));}
const report={threeRevision:'7e89746a687bb88271253698fa108b1dd59825d0',viewport:[512,512],inputResolution:[256,256],sampleTexel:[128,128],time:0,description:'Five consecutive stationary frames after initial render. Current and previous NDC both use the unjittered projection ( VelocityNode\'s currentProjectionMatrix uniform holds TAAU\'s override ), so the velocity is zero. Each value is read directly from the original RGBA16F velocity MRT.',samples:results};
writeFileSync(process.env.OUTPUT||'docs/taau-r187-velocity.json',JSON.stringify(report,null,2)+'\n');console.log('TAAU stationary GPU velocity is zero ( unjittered current and previous clips ) for all five frames.');await browser.close();
