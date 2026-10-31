// three.js's own assets each example's page loads
// ( web/examples/upstream-assets.json lists them, tools/gallery/upstream_assets.py ),
// recorded by opening every ported example's page in Chromium:
// tools/gallery/upstream-pages.json. tools/gallery/pages.mjs writes each page's
// part of the listing into it, so a published page redirects its assets to the
// CDN without the whole listing; a page that asks for an asset its part lacks
// ( after input the recording did not give ) reads the whole listing then
// ( web/examples/upstream.js ).
//
//   node tools/gallery/upstream_pages.mjs   ( after scripts/build-gallery )
import {chromium} from '@playwright/test';
import config from '../../playwright.config.js';
import {spawn} from 'node:child_process';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const OUT = path.join(ROOT, 'tools/gallery/upstream-pages.json');
const PORT = 8176;
const WORKERS = 6;
const GESTURES = {
	webgpu_compute_audio: 'button:text-is("Play")',
	webgl_postprocessing_glitch: '#startButton',
};

const listing = JSON.parse(fs.readFileSync(path.join(ROOT, 'web/examples/upstream-assets.json'), 'utf8')).files;
const catalog = JSON.parse(fs.readFileSync(path.join(ROOT, 'web/examples/catalog.json'), 'utf8'));
const ids = catalog.examples.filter(e => e.port && e.status !== 'excluded').map(e => e.id);

const server = spawn('python3', ['-m', 'http.server', String(PORT), '--bind', '127.0.0.1'], {cwd: ROOT, stdio: 'ignore'});
await new Promise(resolve => setTimeout(resolve, 1000));
const browser = await chromium.launch(config.use.launchOptions);
const found = {};

async function record(id) {
	const page = await browser.newPage({viewport: {width: 1280, height: 720}});
	const assets = new Set();
	page.on('request', request => {
		// By its path from the site's root ( web/ ), as the listing names it.
		const path = decodeURIComponent(new URL(request.url()).pathname);
		const file = path.startsWith('/web/') ? path.slice('/web/'.length) : null;
		if (file && listing[file]) assets.add(file);
	});
	try {
		await page.goto(`http://127.0.0.1:${PORT}/web/examples/${id}.html`);
		// The gestures some pages wait for before they load their assets, as
		// the published-site check gives them ( tests/pages/gallery.spec.js ).
		const gesture = GESTURES[id];
		if (gesture) await page.locator(gesture).click({timeout: 60000});
		const start = Date.now();
		while (Date.now() - start < 45000) {
			const [frames, status, error] = await page.evaluate(() => {
				const canvas = document.querySelector('canvas');
				return [Number(canvas?.dataset.frames ?? 0), document.body.dataset.status, canvas?.dataset.error];
			}).catch(() => [0, null, null]);
			if (error || status === 'error' || frames > 2) break;
			if (status === 'running' && frames > 0 && Date.now() - start > 15000) break;
			await page.waitForTimeout(500);
		}
		// Assets the first frames start loading.
		await page.waitForTimeout(1000);
	} catch (error) {
		console.warn(id, String(error).split('\n')[0]);
	}
	await page.close();
	if (assets.size) found[id] = [...assets].sort();
}

const queue = [...ids];
await Promise.all(Array.from({length: WORKERS}, async () => {
	while (queue.length) await record(queue.shift());
}));
await browser.close();
server.kill();
const sorted = Object.fromEntries(Object.keys(found).sort().map(id => [id, found[id]]));
fs.writeFileSync(OUT, JSON.stringify(sorted, null, 1) + '\n');
console.log(`${Object.keys(sorted).length} examples load upstream assets ( ${new Set(Object.values(sorted).flat()).size} files )`);
