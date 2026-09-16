import{test,expect}from'@playwright/test';
test('glTF controls, resize, recoverable failures and 20 overlapping replacement cycles',async({page})=>{
 test.setTimeout(300000);const errors=[];page.on('pageerror',e=>errors.push(String(e)));
 await page.goto('/web/?example=4&fixture=1');const canvas=page.locator('canvas');
 await expect.poll(async()=>Number(await canvas.getAttribute('data-frames')),{timeout:120000}).toBeGreaterThan(2);
 const nextFrame=async()=>{const frame=Number(await canvas.getAttribute('data-frames'));await expect.poll(async()=>Number(await canvas.getAttribute('data-frames')),{timeout:30000}).toBeGreaterThan(frame+1);};
 const original=await canvas.screenshot();
 await page.locator('#gltf-exposure').fill('2');await nextFrame();expect((await canvas.screenshot()).equals(original)).toBe(false);
 await page.locator('#gltf-background').uncheck();await nextFrame();expect((await canvas.screenshot()).equals(original)).toBe(false);
 await page.locator('#gltf-environment').selectOption('none');await page.locator('#gltf-environment').selectOption('royal');
 await page.evaluate(()=>{document.querySelector('canvas').width=320;document.querySelector('canvas').height=200;});await nextFrame();
 expect(await page.evaluate(()=>[document.querySelector('canvas').width,document.querySelector('canvas').height])).toEqual([320,200]);
 await page.route('**/BoomBox.glb',route=>route.fulfill({status:503,body:'temporary fixture failure'}));
 await page.locator('#gltf-model').selectOption('5');await expect(page.locator('#status')).toContainText('503');await nextFrame();
 await page.unroute('**/BoomBox.glb');await page.locator('#gltf-model').selectOption('4');await expect(page.locator('#status')).toContainText('表示中');
 await page.locator('#gltf-model').selectOption('5');await expect(page.locator('#status')).toContainText('表示中',{timeout:60000});await nextFrame();
 // Delay an older environment load and allow the newer model load to win.
 let release;const delayed=new Promise(resolve=>release=resolve);
 await page.route('**/royal_esplanade_2k.hdr?stale',async route=>{await delayed;await route.continue();});
 await page.evaluate(()=>{window.pendingEnvironment=window.app.load_environment('/web/environments/royal_esplanade_2k.hdr?stale');});
 expect(await page.evaluate(()=>window.app.load_model(4))).toBe(true);release();expect(await page.evaluate(()=>window.pendingEnvironment)).toBe(false);
 const before=await page.evaluate(()=>Array.from(window.app.resource_counts()));
 for(let i=0;i<20;i++){
  const loaded=await page.evaluate(async i=>{const old=window.app.load_model(i%2===0?4:5),latest=window.app.load_model(i%2===0?5:4);return Promise.all([old,latest]);},i);
  expect(loaded).toEqual([false,true]);await nextFrame();const counts=await page.evaluate(()=>Array.from(window.app.resource_counts()));
  expect(counts[0]).toBeLessThanOrEqual(6);expect(counts[2]).toBe(before[2]);
 }
 const stable=await page.evaluate(()=>Array.from(window.app.resource_counts()));await nextFrame();expect(await page.evaluate(()=>Array.from(window.app.resource_counts()))).toEqual(stable);
 const failure=await page.evaluate(async()=>{try{await window.app.load_environment('/web/environments/missing.hdr');return '';}catch(error){return String(error);}});expect(failure).toContain('404');await nextFrame();
 expect(await page.evaluate(()=>window.app.load_environment('/web/environments/royal_esplanade_2k.hdr'))).toBe(true);await nextFrame();
 expect((await page.evaluate(()=>Array.from(window.app.resource_counts())))[2]).toBe(stable[2]+1);
 expect(await canvas.getAttribute('data-error')).toBeNull();expect(errors).toEqual([]);
});
