import { test, expect } from '@playwright/test';
test('Rust app renders, animates, selects objects and releases its event loop', async ({ page }) => {
  const errors=[];
  page.on('pageerror', error => errors.push(String(error)));
  await page.goto('/web/');
  const canvas=page.locator('canvas');
  await expect.poll(async()=>({error:await canvas.getAttribute('data-error'),ready:Number(await canvas.getAttribute('data-frames'))>=5})).toEqual({error:null,ready:true});
  const before=await canvas.screenshot();
  await canvas.click({position:{x:256,y:256}});
  await expect(canvas).toHaveAttribute('data-selected','true');
  await canvas.click({position:{x:10,y:10}});
  await expect(canvas).toHaveAttribute('data-selected','false');
  const frames=Number(await canvas.getAttribute('data-frames'));
  await expect.poll(async()=>Number(await canvas.getAttribute('data-frames'))).toBeGreaterThan(frames+5);
  const after=await canvas.screenshot();
  expect(after.equals(before)).toBe(false);
  await page.evaluate(()=>{window.app.free();window.app=null;});
  const stopped=Number(await canvas.getAttribute('data-frames'));
  await page.waitForTimeout(100);
  expect(Number(await canvas.getAttribute('data-frames'))).toBe(stopped);
  expect(await canvas.getAttribute('data-error')).toBeNull();
  expect(errors).toEqual([]);
});

test('Rust scene rebuild updates attributes, groups and draw range and releases old resources',async({page})=>{
  await page.goto('/web/?static=1');const canvas=page.locator('canvas');
  await expect.poll(async()=>Number(await canvas.getAttribute('data-frames'))).toBeGreaterThan(2);
  const before=await canvas.screenshot();
  for(let i=0;i<20;i++)await page.evaluate(()=>window.app.rebuild());
  await expect(canvas).toHaveAttribute('data-rebuilt','true');
  const frame=Number(await canvas.getAttribute('data-frames'));
  await expect.poll(async()=>Number(await canvas.getAttribute('data-frames'))).toBeGreaterThan(frame+2);
  expect((await canvas.screenshot()).equals(before)).toBe(false);
  expect(await canvas.getAttribute('data-error')).toBeNull();
});
