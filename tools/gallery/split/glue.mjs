// wasm-bindgen's glue, split as the Wasm is ( tools/gallery/split/build.py ).
//
//   node glue.mjs OUT
//
// OUT holds the glue ( three_rs_wasm.js ), the primary module and the parts
// ( parts/<name>.wasm ). A part that exports functions ( its programs, its
// classes' methods ) or imports JavaScript functions the primary module does
// not gets its own glue, parts/<name>.js: the wrappers of its exports and the
// functions it imports. The primary module's glue keeps the rest and exports
// what the parts' glue uses. A page loads the primary module's glue and its
// parts' ( web/gallery/runtime.js ).
//
// A part's glue is a module whose default export binds it to the primary
// module's glue, given as its namespace ( live: wasm, WASM_VECTOR_LEN ):
//   ( core ) => { imports, connect( instance ), exports }
// OUT/parts.json then lists, beside the parts each module loads, the parts with
// glue.
import * as acorn from 'acorn';
import fs from 'node:fs';
import path from 'node:path';

const out = process.argv[2];
const GLUE = path.join(out, 'three_rs_wasm.js');
const BG = './three_rs_wasm_bg.js';

const interfaceOf = file => {
	const module = new WebAssembly.Module(fs.readFileSync(file));
	return {imports: WebAssembly.Module.imports(module), exports: WebAssembly.Module.exports(module)};
};
const primary = interfaceOf(path.join(out, 'three_rs_wasm_bg.wasm'));
const primaryExports = new Set(primary.exports.map(e => e.name));
const primaryImports = new Set(primary.imports.filter(i => i.module === BG).map(i => i.name));
const parts = {};
for (const file of fs.readdirSync(path.join(out, 'parts')).filter(f => f.endsWith('.wasm')).sort())
	parts[file.slice(0, -5)] = interfaceOf(path.join(out, 'parts', file));
// An export a part holds instead of the primary module.
const owner = new Map();
for (const [name, {exports}] of Object.entries(parts))
	for (const e of exports) if (e.kind === 'function' && !primaryExports.has(e.name)) owner.set(e.name, name);

const src = fs.readFileSync(GLUE, 'utf8');
const ast = acorn.parse(src, {ecmaVersion: 'latest', sourceType: 'module'});
const text = node => src.slice(node.start, node.end);

// The top-level declarations: their names, the primary module's or parts'
// exports they call ( wasm.X ) and the top-level names they use.
const decls = [];
const topNames = new Map();
let shims = null;
let importsFunction = null;
for (const node of ast.body) {
	if (node.type === 'ImportDeclaration' || (node.type === 'ExportNamedDeclaration' && !node.declaration)) continue;
	const decl = node.type === 'ExportNamedDeclaration' ? node.declaration : node;
	if (decl.type === 'FunctionDeclaration' && decl.id.name === '__wbg_get_imports') {
		importsFunction = decl;
		const object = decl.body.body[0].declarations[0].init;
		shims = object.properties.filter(p => p.type === 'Property');
		continue;
	}
	const names = decl.type === 'VariableDeclaration'
		? decl.declarations.flatMap(d => d.id.type === 'Identifier' ? [d.id.name] : [])
		: decl.id ? [decl.id.name] : [];
	const entry = {node, decl, names, exported: node.type === 'ExportNamedDeclaration'};
	decls.push(entry);
	for (const n of names) topNames.set(n, entry);
}
for (const node of ast.body) if (node.type === 'ImportDeclaration')
	for (const s of node.specifiers) topNames.set(s.local.name, {imported: node, names: [s.local.name]});

// References in a node: wasm.X, and the top-level names it reads.
function references(node) {
	const calls = new Set(), names = new Set(), idents = [];
	const visit = (n, parent, key) => {
		if (!n || typeof n.type !== 'string') return;
		if (n.type === 'MemberExpression' && !n.computed && n.object.type === 'Identifier' && n.object.name === 'wasm') {
			calls.add(n.property.name);
			idents.push({node: n, call: n.property.name});
			return;
		}
		if (n.type === 'Identifier') {
			const binding = (parent?.type === 'MemberExpression' && key === 'property' && !parent.computed)
				|| (parent?.type === 'Property' && key === 'key' && !parent.computed)
				|| (parent?.type === 'MethodDefinition' && key === 'key')
				|| (parent?.type === 'PropertyDefinition' && key === 'key');
			if (!binding && topNames.has(n.name)) {
				names.add(n.name);
				idents.push({node: n, name: n.name});
			}
			return;
		}
		for (const k of Object.keys(n)) {
			const v = n[k];
			if (Array.isArray(v)) v.forEach(c => visit(c, n, k));
			else if (v && typeof v.type === 'string') visit(v, n, k);
		}
	};
	visit(node, null, null);
	return {calls, names, idents};
}

const shim = new Map(shims.map(p => [p.key.name ?? p.key.value, p]));
for (const d of decls) { d.refs = references(d.decl); d.part = null; }
// What the primary module's glue keeps: what the shims use ( BrowserApp, which
// a shim wraps ), what uses several parts' exports, and what all that uses.
// It calls a part's exports through __wbg_parts, which the part's glue fills.
const pinned = new Set();
const pin = n => {
	const d = topNames.get(n);
	if (!d || d.imported || pinned.has(d)) return;
	pinned.add(d);
	d.part = null;
	d.refs.names.forEach(pin);
};
// Every shim's: a class a shim wraps ( BrowserApp.__wrap ) serves every part
// that imports the shim.
const importedByParts = new Set(Object.values(parts).flatMap(({imports}) => imports.filter(i => i.module === BG).map(i => i.name)));
for (const [f, p] of shim) if (primaryImports.has(f) || importedByParts.has(f)) references(p.value).names.forEach(pin);
// Each other declaration's part: the part whose exports it calls, or whose
// declaration it uses ( Symbol.dispose on its class ).
for (let again = true; again;) {
	for (let changed = true; changed;) {
		changed = false;
		for (const d of decls) {
			if (pinned.has(d)) continue;
			const owners = new Set([...d.refs.calls].filter(c => owner.has(c)).map(c => owner.get(c)));
			for (const n of d.refs.names) if (topNames.get(n).part) owners.add(topNames.get(n).part);
			if (owners.size > 1) { d.names.forEach(pin); if (!d.names.length) { pinned.add(d); d.part = null; d.refs.names.forEach(pin); } changed = true; continue; }
			const part = owners.size ? [...owners][0] : null;
			if (part !== d.part) { d.part = part; changed = true; }
		}
	}
	// The primary module's glue keeps what it uses.
	again = false;
	for (const d of decls) if (d.part === null)
		for (const n of d.refs.names) if (topNames.get(n).part) { pin(n); again = true; }
}

// Shims: each part imports the ones it calls; the primary module's glue keeps
// those the primary module imports.
const partImports = {};
for (const [name, {imports}] of Object.entries(parts)) {
	const fields = imports.filter(i => i.module === BG && i.kind === 'function').map(i => i.name);
	for (const f of fields) if (!shim.has(f)) throw new Error(`glue: ${name} imports ${f}, which the glue lacks`);
	const other = imports.filter(i => i.module !== BG && i.module !== 'primary').map(i => i.module);
	partImports[name] = {fields: fields.filter(f => !primaryImports.has(f)), modules: [...new Set(other)]};
}

// The snippet modules the glue imports, by their import bindings.
const snippetOf = new Map();
for (const node of ast.body) if (node.type === 'ImportDeclaration')
	for (const s of node.specifiers) snippetOf.set(s.local.name, {source: node.source.value, specifier: s});
const returned = importsFunction.body.body.find(s => s.type === 'ReturnStatement').argument.properties;
const namespaceOf = new Map(returned.map(p => [p.key.value ?? p.key.name, p.value.name]));

// A part's copy of a node: its calls of the part's exports through `part`, the
// primary module's glue's names through `core`.
function rewrite(node, part, local) {
	const {idents} = references(node);
	const edits = [];
	for (const r of idents) {
		if (r.call) {
			edits.push([r.node.start, r.node.end, owner.get(r.call) === part ? `part.${r.call}` : `core.wasm.${r.call}`]);
		} else if (!local.has(r.name)) {
			const ns = snippetOf.get(r.name);
			edits.push([r.node.start, r.node.end, ns ? `${ns.specifier.local.name}` : `core.${r.name}`]);
		}
	}
	edits.sort((a, b) => a[0] - b[0]);
	let s = '', at = node.start;
	for (const [a, b, t] of edits) { s += src.slice(at, a) + t; at = b; }
	return s + src.slice(at, node.end);
}

// The module without the imports nothing in it uses: the snippets ( the
// inline_js of the Rust modules ) that only parts call load with those parts.
function withoutUnusedImports(code) {
	const tree = acorn.parse(code, {ecmaVersion: 'latest', sourceType: 'module'});
	const used = new Set();
	const visit = (n, parent, key) => {
		if (!n || typeof n.type !== 'string' || n.type === 'ImportDeclaration') return;
		if (n.type === 'Identifier' && !(parent?.type === 'MemberExpression' && key === 'property' && !parent.computed)
			&& !(parent?.type === 'Property' && key === 'key' && !parent.computed && !parent.shorthand)) used.add(n.name);
		for (const k of Object.keys(n)) {
			const v = n[k];
			if (Array.isArray(v)) v.forEach(c => visit(c, n, k));
			else if (v && typeof v.type === 'string') visit(v, n, k);
		}
	};
	visit(tree, null, null);
	const cut = [];
	for (const node of tree.body) {
		if (node.type !== 'ImportDeclaration' || !node.specifiers.length) continue;
		const kept = node.specifiers.filter(s => used.has(s.local.name));
		if (kept.length === node.specifiers.length) continue;
		const text = kept.length === 0 ? ''
			: kept[0].type === 'ImportNamespaceSpecifier' ? `import * as ${kept[0].local.name} from ${node.source.raw};`
			: `import { ${kept.map(s => s.imported.name === s.local.name ? s.local.name : `${s.imported.name} as ${s.local.name}`).join(', ')} } from ${node.source.raw};`;
		cut.push([node.start, node.end, text]);
	}
	let result = '', from = 0;
	for (const [a, b, t] of cut) { result += code.slice(from, a) + t; from = b; }
	return result + code.slice(from);
}

// JavaScript functions from snippets that the primary module imports for
// several modules' parts ( build.py: shared-imports.json ): the primary
// module's glue forwards them to __wbg_handed, which those parts' glue fills,
// so the snippet loads with the parts.
const shared = process.argv[3] ? JSON.parse(fs.readFileSync(process.argv[3], 'utf8')) : {};
const usesSnippet = f => [...references(shim.get(f).value).names].some(n => snippetOf.has(n));
const handed = new Map(Object.entries(shared).filter(([f, ps]) => primaryImports.has(f) && shim.has(f) && usesSnippet(f) && ps.length));

const glued = [];
const coreUsed = new Set();
for (const name of Object.keys(parts)) {
	const own = decls.filter(d => d.part === name);
	const {fields, modules} = partImports[name];
	const hands = [...handed].filter(([, ps]) => ps.includes(name)).map(([f]) => f);
	if (!own.length && !fields.length && !modules.length && !hands.length) continue;
	const local = new Set(own.flatMap(d => d.names));
	const snippets = new Set();
	const note = node => {
		for (const r of references(node).idents) {
			if (r.call && owner.get(r.call) !== name) coreUsed.add('wasm');
			if (r.name && !local.has(r.name)) {
				if (snippetOf.has(r.name)) snippets.add(r.name); else coreUsed.add(r.name);
			}
		}
	};
	own.forEach(d => note(d.decl));
	fields.forEach(f => note(shim.get(f).value));
	hands.forEach(f => note(shim.get(f).value));
	const lines = [];
	const imported = new Map();
	for (const n of snippets) {
		const {source, specifier} = snippetOf.get(n);
		if (!imported.has(source)) imported.set(source, []);
		imported.get(source).push(specifier.type === 'ImportNamespaceSpecifier' ? `* as ${n}` : specifier.imported.name === n ? n : `${specifier.imported.name} as ${n}`);
	}
	// The snippet modules the part imports from, as the glue hands them to the primary module.
	for (const m of modules) {
		const ns = namespaceOf.get(m);
		if (!ns) throw new Error(`glue: ${name} imports from ${m}, which the glue does not provide`);
		if (!snippets.has(ns)) { snippets.add(ns); const {source} = snippetOf.get(ns); if (!imported.has(source)) imported.set(source, []); imported.get(source).push(`* as ${ns}`); }
	}
	for (const [source, specs] of imported) {
		const named = specs.filter(s => !s.startsWith('* as')), star = specs.filter(s => s.startsWith('* as'));
		const rel = '../' + source.replace(/^\.\//, '');
		for (const s of star) lines.push(`import ${s} from '${rel}';`);
		if (named.length) lines.push(`import { ${named.join(', ')} } from '${rel}';`);
	}
	lines.push(`// ${name}'s glue ( tools/gallery/split/glue.mjs ).`);
	lines.push('export default function (core) {');
	lines.push('let part;');
	for (const d of own) lines.push(rewrite(d.decl, name, local));
	const objects = [];
	if (fields.length) objects.push(`'${BG}': {\n__proto__: null,\n${fields.map(f => `${JSON.stringify(f)}: ${rewrite(shim.get(f).value, name, local)},`).join('\n')}\n}`);
	for (const m of modules) objects.push(`${JSON.stringify(m)}: ${namespaceOf.get(m)}`);
	const exported = own.filter(d => d.exported).flatMap(d => d.names);
	if (hands.length) lines.push(`Object.assign(core.__wbg_handed, {\n${hands.map(f => `${JSON.stringify(f)}: ${rewrite(shim.get(f).value, name, local)},`).join('\n')}\n});`);
	lines.push(`return {\nimports: {\n${objects.join(',\n')}\n},\nconnect(instance) { part = instance.exports; Object.assign(core.__wbg_parts, part); },\nexports: { ${exported.join(', ')} },\n};`);
	lines.push('}');
	fs.writeFileSync(path.join(out, 'parts', `${name}.js`), lines.join('\n') + '\n');
	glued.push(name);
}

// The primary module's glue: without the parts' declarations and shims, and
// exporting what their glue uses.
const edits = [];
const throughParts = node => {
	for (const r of references(node).idents)
		if (r.call && owner.has(r.call)) edits.push([r.node.start, r.node.end, `__wbg_parts.${r.call}`]);
};
for (const d of decls) if (d.part !== null) edits.push([d.node.start, d.node.end, '']); else throughParts(d.decl);
for (const [f, p] of shim) if (primaryImports.has(f) && !handed.has(f)) throughParts(p.value);
for (const f of handed.keys()) {
	const p = shim.get(f);
	edits.push([p.value.start, p.value.end, `function() { return __wbg_handed[${JSON.stringify(f)}].apply(this, arguments); }`]);
}
for (const [f, p] of shim) if (!primaryImports.has(f)) {
	// The property and its trailing comma.
	let end = p.end;
	while (/[\s,]/.test(src[end]) && src[end] !== '\n') end++;
	edits.push([p.start, end, '']);
}
// The snippet modules only parts import from: their glue hands them over.
const primaryModules = new Set(primary.imports.map(i => i.module));
for (const p of returned) if (!primaryModules.has(p.key.value ?? p.key.name)) {
	let end = p.end;
	while (/[\s,]/.test(src[end]) && src[end] !== '\n') end++;
	edits.push([p.start, end, '']);
}
edits.sort((a, b) => a[0] - b[0]);
let core = '', at = 0;
for (const [a, b, t] of edits) { core += src.slice(at, a) + t; at = b; }
core += src.slice(at);
core = withoutUnusedImports(core);
const already = new Set(decls.filter(d => d.exported).flatMap(d => d.names));
const extra = [...coreUsed].filter(n => !already.has(n)).sort();
core += `\n// The parts' exports the primary module's glue calls, as their glue connects them.\nconst __wbg_parts = {};\n// The snippets' functions the parts' glue hands over.\nconst __wbg_handed = {};\n// What the parts' glue uses ( parts/<name>.js ).\nexport { ${[...extra, '__wbg_get_imports', '__wbg_parts', '__wbg_handed'].join(', ')} };\n`;
fs.writeFileSync(GLUE, core);

const listingFile = path.join(out, 'parts.json');
const read = JSON.parse(fs.readFileSync(listingFile, 'utf8'));
const listing = read.modules ?? read;
fs.writeFileSync(listingFile, JSON.stringify({modules: listing, glue: glued}, null, 1) + '\n');
console.log(`glue: ${glued.length} parts with glue; the primary module's ${fs.statSync(GLUE).size} bytes`);
