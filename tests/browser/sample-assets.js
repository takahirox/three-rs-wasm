// glTF-Sample-Assets files the Sponza pages fetch at run time: served from a local
// copy in .cache/glTF-Sample-Assets when one exists ( it is never committed, as
// Sponza's license does not grant redistribution ), else fetched as the pages do.
import {existsSync,readFileSync} from 'node:fs';
const ORIGIN='https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/';
const TYPES={json:'application/json',gltf:'model/gltf+json',bin:'application/octet-stream',jpg:'image/jpeg',png:'image/png'};
export async function routeSampleAssets(page){
 await page.route(`${ORIGIN}**`,route=>{
  const path=`.cache/glTF-Sample-Assets/${decodeURIComponent(new URL(route.request().url()).pathname.split('/main/')[1])}`;
  if(!existsSync(path))return route.continue();
  return route.fulfill({body:readFileSync(path),contentType:TYPES[path.split('.').pop()]??'application/octet-stream',headers:{'access-control-allow-origin':'*'}});
 });
}
