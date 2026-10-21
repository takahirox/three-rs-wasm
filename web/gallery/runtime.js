// The gallery's Wasm ( scripts/build-gallery ): the primary module and its
// glue, then the parts an example needs and their glue ( tools/gallery/split/ ).
// The primary module's exports ( memory, function tables, functions ) are
// every part's imports; a part writes its functions into the shared table and
// its data into the shared memory as it is instantiated, before the example
// starts. A part's glue holds its exports' wrappers ( the example's program )
// and the JavaScript functions only the part calls.
const base = new URL('../pkg/gallery/', import.meta.url);

// parts: [ [ name, whether it has glue ] ], as an example's page lists them
// ( tools/gallery/pages.mjs ). The primary module's glue and the parts' glue
// exports, together.
export async function loadParts(parts, revision = '') {
	const core = await import(new URL(`three_rs_wasm.js?v=${revision}`, base));
	const primary = await core.default({module_or_path: new URL(`three_rs_wasm_bg.wasm?v=${revision}`, base)});
	const exports = {...core};
	await Promise.all(parts.map(async ([name, glued]) => {
		const response = fetch(new URL(`parts/${name}.wasm?v=${revision}`, base));
		const glue = glued ? (await import(new URL(`parts/${name}.js?v=${revision}`, base))).default(core) : null;
		const imports = {...core.__wbg_get_imports(), primary};
		for (const [module, functions] of Object.entries(glue?.imports ?? {}))
			imports[module] = {...imports[module], ...functions};
		const {instance} = await WebAssembly.instantiateStreaming(response, imports);
		glue?.connect(instance);
		Object.assign(exports, glue?.exports);
	}));
	return exports;
}

// A module's parts ( parts.json ), for the browser oracles and tools that call
// a module's exports outside an example's page.
export async function loadRuntime(module, revision = '') {
	const listing = await (await fetch(new URL(`parts.json?v=${revision}`, base))).json();
	const glue = new Set(listing.glue ?? []);
	return loadParts(((listing.modules ?? listing)[module] ?? []).map(name => [name, glue.has(name)]), revision);
}
