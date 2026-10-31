// Every example's own page, web/examples/<id>.html, as three.js has a
// page per example. A ported example's page loads the gallery's Wasm ( the
// library, then the parts that carry the example's program, each with its
// glue ) and runs its program; the other examples' pages show their diagnostic
// ( web/examples/diagnostic.js ). scripts/build-gallery writes them, as the
// parts are its output ( web/build/parts.json ).
//
// A ported example's script is web/examples/example.js specialized for it:
// PAGE_ID, PAGE_PARTS ( its module's parts ) and PAGE_PROGRAM written in, the
// conditions on PAGE_EXAMPLE, the gallery's number for the example, evaluated,
// the code for other examples dropped and the imports it no longer uses
// removed. The markup is tools/gallery/page.html.
//
//   node tools/gallery/pages.mjs
import * as acorn from 'acorn';
import fs from 'node:fs';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const GALLERY = path.join(ROOT, 'web/examples');
const UNKNOWN = Symbol('unknown');

// The value of an expression whose only variable is PAGE_EXAMPLE, or UNKNOWN.
function evaluate(node, n) {
	switch (node.type) {
		case 'Literal': return node.value;
		case 'Identifier': return node.name === 'PAGE_EXAMPLE' ? n : UNKNOWN;
		case 'MemberExpression':
			if (node.computed && node.object.type === 'ObjectExpression') {
				const key = evaluate(node.property, n);
				if (key === UNKNOWN) return UNKNOWN;
				const prop = node.object.properties.find(p => p.type === 'Property' && String(p.key.value ?? p.key.name) === String(key));
				return prop ? {source: prop.value} : undefined;
			}
			return UNKNOWN;
		case 'ArrayExpression': {
			const values = node.elements.map(e => e ? evaluate(e, n) : UNKNOWN);
			return values.includes(UNKNOWN) ? UNKNOWN : values;
		}
		case 'UnaryExpression': {
			if (node.operator !== '!') return UNKNOWN;
			const v = evaluate(node.argument, n);
			return v === UNKNOWN ? UNKNOWN : !v;
		}
		case 'BinaryExpression': {
			const a = evaluate(node.left, n), b = evaluate(node.right, n);
			if (a === UNKNOWN || b === UNKNOWN) return UNKNOWN;
			switch (node.operator) {
				case '===': return a === b;
				case '!==': return a !== b;
				case '<': return a < b;
				case '<=': return a <= b;
				case '>': return a > b;
				case '>=': return a >= b;
				default: return UNKNOWN;
			}
		}
		case 'LogicalExpression': {
			const a = evaluate(node.left, n);
			if (a === UNKNOWN) return UNKNOWN;
			if (node.operator === '&&') return a ? evaluate(node.right, n) : a;
			if (node.operator === '||') return a ? a : evaluate(node.right, n);
			return UNKNOWN;
		}
		case 'CallExpression': {
			// [ … ].includes( PAGE_EXAMPLE )
			const c = node.callee;
			if (c.type === 'MemberExpression' && !c.computed && c.property.name === 'includes' && node.arguments.length === 1) {
				const list = evaluate(c.object, n), v = evaluate(node.arguments[0], n);
				if (list === UNKNOWN || v === UNKNOWN || !Array.isArray(list)) return UNKNOWN;
				return list.includes(v);
			}
			return UNKNOWN;
		}
		default: return UNKNOWN;
	}
}

const lexical = stmt => stmt.type === 'ClassDeclaration' || stmt.type === 'FunctionDeclaration' || (stmt.type === 'VariableDeclaration' && stmt.kind !== 'var');

// The source of `node` for the page: its constants written in, the conditions
// on PAGE_EXAMPLE folded.
function specialize(src, node, page) {
	const n = page.example;
	const edits = [];
	const sub = node => specialize(src, node, page);
	// A kept branch: a block without declarations of its own loses its braces.
	// A chosen operand in its place: a name, a property path or a number needs
	// no parentheses.
	const operand = node => { const s = sub(node); return /^[\w$.]+$/.test(s) ? s : '(' + s + ')'; };
	const branch = stmt => stmt.type === 'BlockStatement' && !stmt.body.some(lexical)
		? stmt.body.map(sub).join('\n')
		: sub(stmt);
	const visit = node => {
		if (!node || typeof node.type !== 'string') return;
		switch (node.type) {
			case 'Identifier':
				if (node.name === 'PAGE_EXAMPLE') edits.push([node.start, node.end, String(n)]);
				else if (node.name === 'PAGE_ID') edits.push([node.start, node.end, `'${page.id}'`]);
				else if (node.name === 'PAGE_PARTS') edits.push([node.start, node.end, JSON.stringify(page.parts)]);
				else if (node.name === 'PAGE_PROGRAM') edits.push([node.start, node.end, page.program]);
				return;
			case 'IfStatement': {
				const v = evaluate(node.test, n);
				if (v === UNKNOWN) break;
				const keep = v ? node.consequent : node.alternate;
				edits.push([node.start, node.end, keep ? branch(keep) : '']);
				return;
			}
			case 'ConditionalExpression': {
				const v = evaluate(node.test, n);
				if (v === UNKNOWN) break;
				edits.push([node.start, node.end, operand(v ? node.consequent : node.alternate)]);
				return;
			}
			case 'LogicalExpression': {
				const a = evaluate(node.left, n);
				if (a !== UNKNOWN) {
					const left = node.operator === '&&' ? !a : !!a;
					edits.push([node.start, node.end, left ? JSON.stringify(a) : operand(node.right)]);
					return;
				}
				// x && true, x || false: x
				const b = evaluate(node.right, n);
				if (b === (node.operator === '&&')) {
					edits.push([node.start, node.end, sub(node.left)]);
					return;
				}
				break;
			}
			case 'MemberExpression': {
				if (!node.computed || node.object.type !== 'ObjectExpression') break;
				const v = evaluate(node, n);
				if (v === UNKNOWN) break;
				edits.push([node.start, node.end, v === undefined ? 'undefined' : sub(v.source)]);
				return;
			}
			case 'CallExpression': case 'BinaryExpression': case 'UnaryExpression': {
				const v = evaluate(node, n);
				if (typeof v !== 'boolean') break;
				edits.push([node.start, node.end, String(v)]);
				return;
			}
		}
		for (const key of Object.keys(node)) {
			const v = node[key];
			if (Array.isArray(v)) v.forEach(visit);
			else if (v && typeof v.type === 'string') visit(v);
		}
	};
	visit(node);
	edits.sort((a, b) => a[0] - b[0]);
	let out = '';
	let at = node.start;
	for (const [s, e, text] of edits) {
		out += src.slice(at, s) + text;
		at = e;
	}
	return out + src.slice(at, node.end);
}

// The script without the imports it does not use; an import of a module for
// its effects stays.
function withoutUnusedImports(script) {
	const ast = acorn.parse(script, {ecmaVersion: 'latest', sourceType: 'module'});
	const used = new Set();
	const visit = (n, parent) => {
		if (!n || typeof n.type !== 'string' || n.type === 'ImportDeclaration') return;
		if (n.type === 'Identifier' && !(parent?.type === 'MemberExpression' && parent.property === n && !parent.computed)
			&& !(parent?.type === 'Property' && parent.key === n && !parent.computed && !parent.shorthand)) used.add(n.name);
		for (const k of Object.keys(n)) {
			const v = n[k];
			if (Array.isArray(v)) v.forEach(c => visit(c, n));
			else if (v && typeof v.type === 'string') visit(v, n);
		}
	};
	visit(ast, null);
	const edits = [];
	for (const node of ast.body) {
		if (node.type !== 'ImportDeclaration' || !node.specifiers.length) continue;
		const kept = node.specifiers.filter(s => used.has(s.local.name));
		if (kept.length === node.specifiers.length) continue;
		const names = kept.map(s => s.imported && s.imported.name !== s.local.name ? `${s.imported.name} as ${s.local.name}` : s.local.name);
		edits.push([node.start, node.end, kept.length ? `import {${names.join(', ')}} from ${node.source.raw};` : '']);
	}
	let out = '', at = 0;
	for (const [a, b, t] of edits) { out += script.slice(at, a) + t; at = b; }
	return out + script.slice(at);
}

// The records of the original's UI an example's page carries ( web/examples/ui/ui.js ):
// its spec ( specs.json ), its page's chrome ( pages.json ), the gallery's
// names for its controllers ( aliases.json ) and its live readout ( extras/ ).
function uiRecords() {
	const ui = path.join(GALLERY, 'ui');
	const read = file => JSON.parse(fs.readFileSync(path.join(ui, file), 'utf8'));
	const specs = read('specs.json'), chrome = read('pages.json'), aliases = read('aliases.json');
	const extras = new Map();
	for (const file of fs.readdirSync(path.join(ui, 'extras')).filter(f => f.endsWith('.js'))) {
		const ast = acorn.parse(fs.readFileSync(path.join(ui, 'extras', file), 'utf8'), {ecmaVersion: 'latest', sourceType: 'module'});
		const list = ast.body.find(n => n.type === 'ExportNamedDeclaration' && n.declaration?.declarations?.[0]?.id.name === 'examples');
		for (const e of list.declaration.declarations[0].init.elements) extras.set(e.value, file.slice(0, -3));
	}
	return id => {
		const records = {spec: specs[id], page: chrome[id], aliases: aliases[id], extras: extras.get(id)};
		if (!Object.values(records).some(v => v !== undefined)) return '';
		// A script element's text: no '<' ( '</script', '<!--' ) changes how it parses.
		return `<script type="application/json" id="gallery-ui">${JSON.stringify(records).replaceAll('<', '\\u003c')}</script>\n`;
	};
}

// The page's part of the listing of three.js's own assets that load from r186's
// commit ( web/examples/upstream.js; jsDelivr refuses r187's ):
// the files its example loads ( tools/gallery/upstream-pages.json ), with the
// listing's base, which the published site sets ( tools/build_site.py ).
function upstreamAssets() {
	const listing = JSON.parse(fs.readFileSync(path.join(GALLERY, 'upstream-assets.json'), 'utf8'));
	const recorded = path.join(ROOT, 'tools/gallery/upstream-pages.json');
	const pages = fs.existsSync(recorded) ? JSON.parse(fs.readFileSync(recorded, 'utf8')) : {};
	return id => {
		const files = Object.fromEntries((pages[id] ?? []).filter(f => listing.files[f]).map(f => [f, listing.files[f]]));
		return `<script type="application/json" id="upstream-assets">${JSON.stringify({base: listing.base, files}).replaceAll('<', '\\u003c')}</script>\n`;
	};
}

// A ported example's page: whether it shows the original's UI ( automated
// runs leave it out unless ?ui ), and with it the start of the GPU memory
// counts the Inspector reads ( web/examples/ui/gpu-memory.js ), fetched while
// the page parses.
const PRELUDE = '<script>window.galleryUi=!navigator.webdriver||new URLSearchParams(location.search).has("ui");'
	+ 'if(galleryUi)window.galleryGpuMemoryReady=import("./ui/gpu-memory.js").catch(e=>console.warn("gpu memory",e));</script>\n';

function importmap(id) {
	// The TSL editor's three/tsl is the three-tsl crate; elsewhere 'three' names resolve to the
	// Inspector's stand-ins ( ui/three/shim.js ): no three.js renderer loads in the gallery.
	const shim = './ui/three/shim.js';
	return JSON.stringify({imports: {
		'three/tsl': id === 'webgpu_tsl_editor' ? './tsl-module.js' : shim,
		'three': shim,
		'three/webgpu': shim,
		'three/addons/': 'https://cdn.jsdelivr.net/npm/three@0.187.0/examples/jsm/',
	}});
}

export function pages() {
	const catalog = JSON.parse(fs.readFileSync(path.join(GALLERY, 'catalog.json'), 'utf8'));
	const modules = JSON.parse(fs.readFileSync(path.join(GALLERY, 'modules.json'), 'utf8')).examples;
	// The parts each module loads and those with glue ( tools/gallery/split/ );
	// an unsplit build has none.
	const listing = JSON.parse(fs.readFileSync(path.join(ROOT, 'web/build/parts.json'), 'utf8'));
	const glue = new Set(listing.glue ?? []);
	const partsOf = module => ((listing.modules ?? listing)[module] ?? []).map(name => [name, glue.has(name)]);
	const template = fs.readFileSync(path.join(ROOT, 'tools/gallery/page.html'), 'utf8');
	const src = fs.readFileSync(path.join(GALLERY, 'example.js'), 'utf8');
	const ast = acorn.parse(src, {ecmaVersion: 'latest', sourceType: 'module'});
	const ui = uiRecords();
	const upstream = upstreamAssets();
	const out = new Map();
	for (const entry of catalog.examples) {
		const ported = entry.port && entry.status !== 'excluded';
		const script = ported
			? withoutUnusedImports(specialize(src, ast, {id: entry.id, example: entry.port.example, parts: partsOf(modules[entry.id] ?? 'core'), program: entry.id.replaceAll('-', '_')}))
				// Without the template's own comment and its emptied lines.
				.replace(/^(\/\/.*\n)+/, '').split('\n').filter(line => line.trim()).join('\n')
			: `import {explain} from './diagnostic.js';\nexplain('${entry.id}');`;
		if (script.includes('</script')) throw new Error(`${entry.id}: the script closes its element`);
		const title = `three-rs-wasm - ${entry.id.replaceAll('_', ' - ')}`;
		const html = template.replace('%TITLE%', title).replace('%PRELUDE%', () => ported ? PRELUDE : '').replace('%IMPORTMAP%', importmap(entry.id)).replace('%UI%', () => ported ? ui(entry.id) : '').replace('%UPSTREAM%', () => ported ? upstream(entry.id) : '').replace('%SCRIPT%', () => script);
		out.set(`${entry.id}.html`, html);
	}
	return out;
}

if (process.argv[1] === fileURLToPath(import.meta.url)) {
	for (const [f, html] of pages()) fs.writeFileSync(path.join(GALLERY, f), html);
}
