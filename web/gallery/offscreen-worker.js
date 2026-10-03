// webgl_worker_offscreencanvas's worker: this package rendering scene.js's scene into the
// transferred OffscreenCanvas ( src/browser/offscreen.rs ), as jsm/offscreen/offscreen.js does.
// Messages that arrive while the package loads are queued.
const queued = [];
let receive = message => queued.push(message);
self.onmessage = message => receive(message);
const runtime = new URL(location.href).searchParams.get('v') ?? '';
const {default: init, offscreen_worker, offscreen_time, offscreen_samples} = await import(`../pkg/three_rs_wasm.js?v=${runtime}`);
await init({module_or_path: new URL(`../pkg/three_rs_wasm_bg.wasm?v=${runtime}`, import.meta.url)});
receive = message => {
 const data = message.data;
 if ('time' in data) { offscreen_time(data.time); return; }
 if ('samples' in data) { offscreen_samples(data.samples); return; }
 offscreen_worker(data.drawingSurface, data.width, data.height, data.pixelRatio, data.matcap).catch(error => self.postMessage({error: String(error)}));
};
for (const message of queued) receive(message);
