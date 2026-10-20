// The gallery's Wasm ( scripts/build-gallery ): the primary module, then the
// parts the example's module needs ( tools/gallery/split/ ). The primary
// module's exports ( memory, function tables, functions ) are every part's
// imports; a part writes its functions into the shared table and its data into
// the shared memory as it is instantiated, before the example starts.
const base = new URL('../pkg/gallery/', import.meta.url);

// The module each example runs ( tools/gallery/modules.py ).
export async function moduleOf(id) {
	const {examples} = await (await fetch(new URL('modules.json', import.meta.url))).json();
	return examples[id] ?? 'core';
}

export async function loadRuntime(module, revision = '') {
	const glue = await import(new URL(`three_rs_wasm.js?v=${revision}`, base));
	const exports = await glue.default({module_or_path: new URL(`three_rs_wasm_bg.wasm?v=${revision}`, base)});
	const listing = await (await fetch(new URL(`parts.json?v=${revision}`, base))).json();
	await Promise.all((listing[module] ?? []).map(async (name) => {
		const response = fetch(new URL(`parts/${name}.wasm?v=${revision}`, base));
		await WebAssembly.instantiateStreaming(response, {primary: exports});
	}));
	return glue;
}
