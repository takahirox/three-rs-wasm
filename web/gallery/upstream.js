// The demo's files that are the pinned three.js r186's own example assets
// ( upstream-assets.json, tools/gallery/upstream_assets.py ) are left out of the
// published site, which loads them from the three.js repository at that commit:
// fetch(), image, media and Audio URLs are redirected there. Locally, the files are
// in the repository and nothing changes ( the listing's base is null ).
const listing = await (await fetch(new URL('upstream-assets.json', import.meta.url))).json();

export function upstream(url) {
	if (!listing.base) return url;
	try {
		const resolved = new URL(url, globalThis.document?.baseURI ?? globalThis.location.href);
		const path = decodeURIComponent(resolved.pathname);
		const at = path.indexOf('/web/');
		const file = at < 0 ? undefined : listing.files[path.slice(at + 1)];
		return file ? listing.base + encodeURI(file) : url;
	} catch {
		return url;
	}
}

if (listing.base) {
	const fetch = globalThis.fetch;
	globalThis.fetch = function (input, init) {
		if (typeof input === 'string' || input instanceof URL) return fetch.call(this, upstream(String(input)), init);
		if (input instanceof Request && upstream(input.url) !== input.url) return fetch.call(this, new Request(upstream(input.url), input), init);
		return fetch.call(this, input, init);
	};
	if (globalThis.document) {
		// A redirected image or medium is cross-origin: it loads with CORS ( the CDN
		// allows any origin ), as the same-origin file did, so canvases and GPU
		// uploads may read it.
		const redirect = (element, value) => {
			const url = upstream(value);
			const media = element instanceof HTMLSourceElement ? element.parentElement : element;
			if (url !== value && media && 'crossOrigin' in media && media.crossOrigin === null) media.crossOrigin = 'anonymous';
			return url;
		};
		for (const element of [HTMLImageElement, HTMLMediaElement, HTMLSourceElement]) {
			const src = Object.getOwnPropertyDescriptor(element.prototype, 'src');
			Object.defineProperty(element.prototype, 'src', {
				...src,
				set(value) {
					src.set.call(this, redirect(this, value));
				}
			});
		}
		const setAttribute = Element.prototype.setAttribute;
		Element.prototype.setAttribute = function (name, value) {
			const media = this instanceof HTMLImageElement || this instanceof HTMLMediaElement || this instanceof HTMLSourceElement;
			return setAttribute.call(this, name, media && name.toLowerCase() === 'src' ? redirect(this, value) : value);
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
			return src === undefined ? new Audio() : new Audio(upstream(src));
		};
		globalThis.Audio.prototype = Audio.prototype;
	}
}
