// Records the UI the pinned r186 examples build: their lil-gui panels, the
// WebGPU Inspector's parameter groups and profiler state, and stats.js. Each
// example runs as published ( .cache/three-r186/examples/<id>.html ) with
// three's build entries served from src/, while its UI libraries are wrapped
// to record what the page creates. After the page settles, the UI's final
// structure is read through the libraries' own objects and written to
// web/examples/ui/specs.json, which the gallery replays with the same
// libraries ( web/examples/ui/ ).
//
// Usage: node tools/gallery/ui/extract.mjs [ids…] [--settle ms] [--workers n] [--merge]
import { chromium } from 'playwright';
import { spawn } from 'node:child_process';
import fs from 'node:fs';

const ROOT = new URL( '../../../', import.meta.url ).pathname;
const OUT = ROOT + 'web/examples/ui/specs.json';
const PORT = 8191;
const args = process.argv.slice( 2 );
const option = ( name, fallback ) => {

	const i = args.indexOf( name );
	return i === - 1 ? fallback : args[ i + 1 ];

};

const settle = Number( option( '--settle', 6000 ) );
const merge = args.includes( '--merge' );
const catalog = JSON.parse( fs.readFileSync( ROOT + 'web/examples/catalog.json', 'utf8' ) );
let ids = args.filter( ( a, i ) => ! a.startsWith( '--' ) && ! [ '--settle', '--workers' ].includes( args[ i - 1 ] ) );
if ( ids.length === 0 ) {

	ids = catalog.examples.filter( e => e.port ).map( e => e.id ).filter( id => {

		const path = `${ ROOT }.cache/three-r186/examples/${ id }.html`;
		if ( ! fs.existsSync( path ) ) return false;
		const src = fs.readFileSync( path, 'utf8' );
		return /lil-gui|inspector\/Inspector|libs\/stats\.module/.test( src );

	} );

}

const BUILD = {
	'three.webgpu.js': '../src/Three.WebGPU.js',
	'three.webgpu.nodes.js': '../src/Three.WebGPU.Nodes.js',
	'three.module.js': '../src/Three.js',
	'three.core.js': '../src/Three.Core.js',
	'three.tsl.js': '../src/Three.TSL.js'
};

const LIL_WRAPPER = `
import * as m from './lil-gui.module.min.js?orig';
export * from './lil-gui.module.min.js?orig';
window.__lil = m;
class GUI extends m.GUI {
	constructor( options = {} ) {
		super( options );
		if ( ! options.parent ) ( window.__guis ||= [] ).push( this );
	}
}
export { GUI };
export default GUI;
`;

const INSPECTOR_WRAPPER = `
import { Inspector as Original } from './Inspector.js?orig';
export * from './Inspector.js?orig';
class Inspector extends Original {
	constructor( options ) {
		super( options );
		( window.__inspectors ||= [] ).push( { inspector: this, options: options ?? null, calls: [] } );
		const record = window.__inspectors.at( - 1 );
		for ( const name of [ 'setActiveTab', 'setHorizontalAlign', 'setVerticalAlign', 'setVisible', 'hide', 'show', 'setActiveExtension' ] ) {
			const f = this[ name ];
			this[ name ] = function ( ...a ) {
				record.calls.push( [ name, ...a.map( v => v && v.id !== undefined ? v.id : v ) ] );
				return f.apply( this, a );
			};
		}
	}
}
export { Inspector };
`;

// ParametersGroup: record each child in order, and each editor's listen().
const PARAMETERS_PATCH = `
for ( const name of [ 'addFolder', 'addSlider', 'addNumber', 'addBoolean', 'addSelect', 'addColor', 'addString', 'addButton' ] ) {
	const f = ParametersGroup.prototype[ name ];
	ParametersGroup.prototype[ name ] = function ( ...a ) {
		const result = f.apply( this, a );
		( this.__children ||= [] ).push( { op: name, args: a, result } );
		return result;
	};
}
for ( const name of [ 'close', 'hide', 'show' ] ) {
	const f = ParametersGroup.prototype[ name ];
	ParametersGroup.prototype[ name ] = function ( ...a ) {
		( this.__calls ||= [] ).push( name );
		return f.apply( this, a );
	};
}
{
	const f = ParametersGroup.prototype._addParameter;
	ParametersGroup.prototype._addParameter = function ( object, property, editor, subItem ) {
		f.call( this, object, property, editor, subItem );
		const listen = editor.listen;
		editor.listen = function () { editor.__listen = true; return listen.call( this ); };
		editor.__subItem = subItem;
	};
}
{
	const f = Parameters.prototype.createGroup;
	Parameters.prototype.createGroup = function ( name ) {
		const group = f.call( this, name );
		( this.__groups ||= [] ).push( group );
		group.__title = name;
		return group;
	};
}
`;

const STATS_PATCH = `
const __Stats = function () {
	const s = Stats();
	const record = { stats: s, shows: [], begin: 0, end: 0, update: 0 };
	( window.__stats ||= [] ).push( record );
	const show = s.showPanel;
	s.showPanel = function ( i ) { record.shows.push( i ); return show.call( this, i ); };
	for ( const k of [ 'begin', 'end', 'update' ] ) {
		const f = s[ k ];
		s[ k ] = function ( ...a ) { record[ k ] ++; return f.apply( this, a ); };
	}
	const add = s.addPanel;
	s.addPanel = function ( panel ) { ( record.panels ||= [] ).push( 'custom' ); return add.call( this, panel ); };
	return s;
};
__Stats.Panel = Stats.Panel;
export default __Stats;
`;

// Read in the page: the final UI.
function snapshot() {

	const ids = new WeakMap();
	let next = 0;
	const objectId = o => {

		if ( o === null || typeof o !== 'object' && typeof o !== 'function' ) return null;
		if ( ! ids.has( o ) ) ids.set( o, next ++ );
		return ids.get( o );

	};

	const plain = v => {

		if ( v === undefined ) return { undefined: true };
		if ( typeof v === 'function' ) return { function: v.name || true };
		if ( typeof v === 'number' && ! Number.isFinite( v ) ) return { number: String( v ) };
		if ( v && typeof v === 'object' ) {

			if ( v.isColor ) return { color: v.getHex() };
			if ( Array.isArray( v ) ) return v.map( plain );
			const out = {};
			for ( const k of Object.keys( v ) ) out[ k ] = plain( v[ k ] );
			return out;

		}

		return v;

	};

	const where = el => {

		if ( ! el ) return null;
		const parent = el.parentElement;
		return {
			parent: parent ? ( parent === document.body ? 'body' : parent.id ? '#' + parent.id : parent.tagName.toLowerCase() + ( parent.className ? '.' + String( parent.className ).split( ' ' ).join( '.' ) : '' ) ) : null,
			style: el.getAttribute( 'style' ),
			display: getComputedStyle( el ).display,
			rect: ( r => [ r.x, r.y, r.width, r.height ] )( el.getBoundingClientRect() )
		};

	};

	const m = window.__lil;
	const controllerType = c => m && (
		c instanceof m.NumberController ? 'number' :
			c instanceof m.BooleanController ? 'boolean' :
				c instanceof m.OptionController ? 'option' :
					c instanceof m.ColorController ? 'color' :
						c instanceof m.FunctionController ? 'function' :
							c instanceof m.StringController ? 'string' : 'controller' );

	const lilNode = node => {

		if ( m && node instanceof m.GUI ) {

			// Elements the page added to the folder itself ( not controllers or folders ).
			const own = new Set( node.children.map( c => c.domElement ) );
			const extras = [ ...node.$children.children ].map( ( el, index ) => ( { el, index } ) ).filter( ( { el } ) => ! own.has( el ) ).map( ( { el, index } ) => ( { index, html: el.outerHTML } ) );
			return {
				kind: 'folder',
				title: node._title,
				closed: node._closed,
				hidden: node._hidden,
				classes: node.domElement.className,
				style: node.domElement.getAttribute( 'style' ),
				extras,
				children: node.children.map( lilNode )
			};

		}

		const c = node;
		const out = {
			kind: 'controller',
			type: controllerType( c ),
			object: objectId( c.object ),
			property: c.property,
			name: c._name,
			value: plain( c.getValue() ),
			initial: plain( c.initialValue ),
			hidden: c._hidden,
			disabled: c._disabled,
			listen: c._listening,
			classes: c.domElement.className,
			style: c.domElement.getAttribute( 'style' )
		};
		if ( out.type === 'number' ) Object.assign( out, { min: plain( c._min ), max: plain( c._max ), step: plain( c._step ), decimals: plain( c._decimals ), slider: c._hasSlider } );
		if ( out.type === 'option' ) Object.assign( out, { values: plain( c._values ), names: plain( c._names ) } );
		if ( out.type === 'color' ) out.rgbScale = c._rgbScale;
		return out;

	};

	const guis = ( window.__guis || [] ).map( g => ( {
		title: g._title,
		closed: g._closed,
		hidden: g._hidden,
		width: g.domElement.style.width || null,
		place: where( g.domElement ),
		classes: g.domElement.className,
		extras: ( () => {

			const own = new Set( g.children.map( c => c.domElement ) );
			return [ ...g.$children.children ].map( ( el, index ) => ( { el, index } ) ).filter( ( { el } ) => ! own.has( el ) ).map( ( { el, index } ) => ( { index, html: el.outerHTML } ) );

		} )(),
		dom: g.domElement.outerHTML,
		children: g.children.map( lilNode )
	} ) );

	const inspectorNode = ( entry ) => {

		const { op, args, result } = entry;
		if ( op === 'addFolder' ) return groupNode( result, args[ 0 ] );
		const [ object, property, ...params ] = args;
		const sub = result && result.__subItem;
		const label = sub && sub.data && sub.data[ 0 ] ? sub.data[ 0 ].textContent : property;
		return {
			kind: 'controller',
			type: op.replace( /^add/, '' ).toLowerCase(),
			object: objectId( object ),
			property,
			name: label,
			value: plain( object[ property ] ),
			params: plain( params ),
			listen: !! ( result && result.__listen ),
			hidden: sub ? sub.domElement.style.display === 'none' : false
		};

	};

	const groupNode = ( group, title ) => ( {
		kind: 'folder',
		title: group.__title ?? title,
		calls: group.__calls || [],
		children: ( group.__children || [] ).map( inspectorNode )
	} );

	const inspectors = ( window.__inspectors || [] ).map( ( { inspector, options, calls } ) => ( {
		options: plain( options ),
		calls,
		activeTab: inspector.profiler.activeTabId,
		parametersVisible: inspector.parameters.isVisible,
		groups: ( inspector.parameters.__groups || [] ).map( g => groupNode( g ) ),
		dom: inspector.parameters.content ? inspector.parameters.content.outerHTML : null,
		toolbar: inspector.profiler.toggleButton ? inspector.profiler.toggleButton.querySelector( '.builtin-tabs-container' ).outerHTML : null,
		place: where( inspector.domElement ),
		visible: inspector.domElement.style.display !== 'none'
	} ) );

	const stats = ( window.__stats || [] ).map( r => ( {
		shows: r.shows,
		panels: r.panels || [],
		begin: r.begin > 0,
		end: r.end > 0,
		update: r.update > 0,
		place: where( r.stats.dom )
	} ) );

	return { guis, inspectors, stats };

}

const server = spawn( 'python3', [ '-m', 'http.server', String( PORT ), '--bind', '127.0.0.1' ], { cwd: ROOT, stdio: 'ignore' } );
await new Promise( r => setTimeout( r, 800 ) );
const browser = await chromium.launch( {
	...( process.platform === 'darwin' ? { executablePath: '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' } : {} ),
	args: process.platform === 'linux'
		? [ '--enable-unsafe-webgpu', '--enable-features=Vulkan', '--use-angle=vulkan', '--use-vulkan=swiftshader', '--disable-vulkan-surface' ]
		: [ '--enable-unsafe-webgpu', '--enable-features=Vulkan', '--use-angle=default' ]
} );
const specs = merge && fs.existsSync( OUT ) ? JSON.parse( fs.readFileSync( OUT, 'utf8' ) ) : {};
const base = `http://127.0.0.1:${ PORT }/.cache/three-r186/`;

const WORKERS = Number( option( '--workers', 3 ) );
let cursor = 0;
await Promise.all( Array.from( { length: WORKERS }, async () => {

	while ( cursor < ids.length ) await extract( ids[ cursor ++ ] );

} ) );

async function extract( id ) {

	const context = await browser.newContext( { viewport: { width: 1280, height: 720 } } );
	const page = await context.newPage();
	const errors = [];
	page.on( 'pageerror', e => errors.push( String( e ).slice( 0, 300 ) ) );
	await page.route( `${ base }build/**`, route => {

		const name = new URL( route.request().url() ).pathname.split( '/' ).pop();
		if ( BUILD[ name ] ) return route.fulfill( { contentType: 'text/javascript', body: `export * from '${ BUILD[ name ] }';` } );
		return route.continue();

	} );
	await page.route( `${ base }examples/jsm/libs/lil-gui.module.min.js`, route => route.fulfill( { contentType: 'text/javascript', body: LIL_WRAPPER } ) );
	await page.route( `${ base }examples/jsm/inspector/Inspector.js`, route => route.fulfill( { contentType: 'text/javascript', body: INSPECTOR_WRAPPER } ) );
	await page.route( `${ base }examples/jsm/inspector/tabs/Parameters.js`, async route => {

		const body = fs.readFileSync( `${ ROOT }.cache/three-r186/examples/jsm/inspector/tabs/Parameters.js`, 'utf8' ).replace( 'export { Parameters };', PARAMETERS_PATCH + '\nexport { Parameters };' );
		return route.fulfill( { contentType: 'text/javascript', body } );

	} );
	await page.route( `${ base }examples/jsm/libs/stats.module.js`, async route => {

		const body = fs.readFileSync( `${ ROOT }.cache/three-r186/examples/jsm/libs/stats.module.js`, 'utf8' ).replace( 'export default Stats;', STATS_PATCH );
		return route.fulfill( { contentType: 'text/javascript', body } );

	} );
	await page.route( /lil-gui\.module\.min\.js\?orig$/, route => route.fulfill( { contentType: 'text/javascript', body: fs.readFileSync( `${ ROOT }.cache/three-r186/examples/jsm/libs/lil-gui.module.min.js` ) } ) );
	await page.route( /inspector\/Inspector\.js\?orig$/, route => route.fulfill( { contentType: 'text/javascript', body: fs.readFileSync( `${ ROOT }.cache/three-r186/examples/jsm/inspector/Inspector.js` ) } ) );
	let spec;
	try {

		await page.goto( `${ base }examples/${ id }.html`, { timeout: 60000 } );
		// Settle: the UI stops changing for 1.5 s, or the time runs out.
		let previous = '', stable = 0;
		const start = Date.now();
		while ( Date.now() - start < settle + 20000 ) {

			await page.waitForTimeout( 500 );
			const now = JSON.stringify( await page.evaluate( snapshot ) );
			if ( now === previous && Date.now() - start > settle ) stable += 500; else stable = 0;
			previous = now;
			if ( stable >= 1500 ) break;

		}

		spec = JSON.parse( previous );

	} catch ( e ) {

		spec = { error: String( e ).slice( 0, 300 ) };

	}

	spec.errors = errors;
	specs[ id ] = spec;
	console.log( id, spec.error ? 'ERROR ' + spec.error : `gui ${ spec.guis.length } inspector ${ spec.inspectors.length } stats ${ spec.stats.length }`, errors.length ? `pageerrors ${ errors.length }` : '' );
	await context.close();

}

await browser.close();
server.kill();
fs.mkdirSync( ROOT + 'web/examples/ui', { recursive: true } );
const sorted = Object.fromEntries( Object.keys( specs ).sort().map( k => [ k, specs[ k ] ] ) );
fs.writeFileSync( OUT, JSON.stringify( sorted, null, '\t' ) + '\n' );
