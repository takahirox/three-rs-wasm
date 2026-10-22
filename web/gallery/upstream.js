// The demo's files that are the pinned three.js r186's own example assets
// ( upstream-assets.json, tools/gallery/upstream_assets.py ) are left out of the
// published site, which loads them from the three.js repository at that commit:
// fetch(), image, media and Audio URLs are redirected there. Locally, the files are
// in the repository and nothing changes ( the listing's base is null ).
//
// An example's page carries its part of the listing ( #upstream-assets, written
// by tools/gallery/pages.mjs from tools/gallery/upstream-pages.json ); a request
// for an asset its part lacks that fails is retried with the whole listing.
// Other pages and workers read the whole listing.
const fetchFile = globalThis.fetch;
const WHOLE = new URL('upstream-assets.json', import.meta.url);
const part = globalThis.document?.getElementById('upstream-assets');
const listing = part ? JSON.parse(part.textContent) : await (await fetchFile(WHOLE)).json();
let whole = part ? null : Promise.resolve(listing);

function lookup(files, url) {
	try {
		const resolved = new URL(url, globalThis.document?.baseURI ?? globalThis.location.href);
		const path = decodeURIComponent(resolved.pathname);
		const at = path.indexOf('/web/');
		const file = at < 0 ? undefined : files[path.slice(at + 1)];
		return file ? listing.base + encodeURI(file) : null;
	} catch {
		return null;
	}
}

export function upstream(url) {
	if (!listing.base) return url;
	return lookup(listing.files, url) ?? url;
}

// The upstream URL of an asset the page's part of the listing lacks, or null.
async function retry(url) {
	if (!listing.base || !part || lookup(listing.files, url)) return null;
	whole ??= fetchFile(WHOLE).then(r => r.json()).catch(() => ({files: {}}));
	return lookup((await whole).files, url);
}

if (listing.base) {
	globalThis.fetch = async function (input, init) {
		const url = typeof input === 'string' || input instanceof URL ? String(input) : input instanceof Request ? input.url : null;
		if (url === null) return fetchFile.call(this, input, init);
		const redirected = upstream(url);
		const request = input instanceof Request ? (redirected !== url ? new Request(redirected, input) : input) : redirected;
		const response = await fetchFile.call(this, request, init);
		if (response.status !== 404) return response;
		const second = await retry(url);
		return second ? fetchFile.call(this, input instanceof Request ? new Request(second, input) : second, init) : response;
	};
	if (globalThis.document) {
		// A redirected image or medium is cross-origin: it loads with CORS ( the CDN
		// allows any origin ), as the same-origin file did, so canvases and GPU
		// uploads may read it.
		const anonymous = element => {
			const media = element instanceof HTMLSourceElement ? element.parentElement : element;
			if (media && 'crossOrigin' in media && media.crossOrigin === null) media.crossOrigin = 'anonymous';
		};
		const redirect = (element, value, set) => {
			const url = upstream(value);
			if (url !== value) anonymous(element);
			else element.addEventListener('error', () => retry(value).then(second => {
				if (!second) return;
				anonymous(element);
				set(second);
			}), {once: true});
			return url;
		};
		for (const element of [HTMLImageElement, HTMLMediaElement, HTMLSourceElement]) {
			const src = Object.getOwnPropertyDescriptor(element.prototype, 'src');
			Object.defineProperty(element.prototype, 'src', {
				...src,
				set(value) {
					src.set.call(this, redirect(this, value, second => src.set.call(this, second)));
				}
			});
		}
		const setAttribute = Element.prototype.setAttribute;
		Element.prototype.setAttribute = function (name, value) {
			const media = this instanceof HTMLImageElement || this instanceof HTMLMediaElement || this instanceof HTMLSourceElement;
			return setAttribute.call(this, name, media && name.toLowerCase() === 'src' ? redirect(this, value, second => setAttribute.call(this, name, second)) : value);
		};
		// Images in markup ( a port's HTML texture content ).
		const markup = html => typeof html !== 'string' ? html : html.replace(/<img\b([^>]*?)\ssrc=(["'])([^"']*)\2/gi, (whole, before, quote, url) => {
			const redirected = upstream(url);
			if (redirected === url) return whole;
			return `<img${before}${/crossorigin/i.test(before) ? '' : ' crossorigin="anonymous"'} src=${quote}${redirected}${quote}`;
		});
		for (const name of ['innerHTML', 'outerHTML']) {
			const property = Object.getOwnPropertyDescriptor(Element.prototype, name);
			Object.defineProperty(Element.prototype, name, {
				...property,
				set(value) {
					property.set.call(this, markup(value));
				}
			});
		}
		const insertAdjacentHTML = Element.prototype.insertAdjacentHTML;
		Element.prototype.insertAdjacentHTML = function (position, html) {
			return insertAdjacentHTML.call(this, position, markup(html));
		};
		const Audio = globalThis.Audio;
		globalThis.Audio = function (src) {
			if (src === undefined) return new Audio();
			const audio = new Audio();
			audio.src = src;
			return audio;
		};
		globalThis.Audio.prototype = Audio.prototype;
	}
}
