// webgl_worker_offscreencanvas's worker: the gallery's Wasm rendering scene.js's scene into the
// transferred OffscreenCanvas ( gallery/src/browser/offscreen.rs ), as jsm/offscreen/offscreen.js does.
// Messages that arrive while the Wasm loads are queued.
const queued = [];
let receive = message => queued.push(message);
self.onmessage = message => receive(message);
// The worker loads the matcap, an r186 asset, as the page does.
await import('./upstream.js');
const params = new URL(location.href).searchParams;
const runtime = params.get('v') ?? '';
// The page's parts ( tools/gallery/pages.mjs ).
const {loadParts} = await import(`./runtime.js?v=${runtime}`);
const {offscreen_worker, offscreen_time, offscreen_samples} = await loadParts(JSON.parse(params.get('parts')), runtime);
receive = message => {
 const data = message.data;
 if ('time' in data) { offscreen_time(data.time); return; }
 if ('samples' in data) { offscreen_samples(data.samples); return; }
 offscreen_worker(data.drawingSurface, data.width, data.height, data.pixelRatio, data.matcap).catch(error => self.postMessage({error: String(error)}));
};
for (const message of queued) receive(message);
