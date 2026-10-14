// Replays tests/tsl/corpus.json against the port ( the three-tsl crate built
// for Node.js into .cache/tsl-node ) and compares every program with its
// golden ( tests/tsl/golden.json.gz ). Each chunk runs in a fresh worker,
// one editor session as in reference/three-js/tsl-corpus.html.
// Usage: node tools/tsl/check.mjs [--chunk N] [--program K] [--diff] [--console]
import { Worker, isMainThread, workerData, parentPort } from 'node:worker_threads';
import { createRequire, register } from 'node:module';
import fs from 'node:fs';
import os from 'node:os';
import zlib from 'node:zlib';

const ROOT = new URL( '../../', import.meta.url ).pathname;

// Console lines the node system writes; the GPU's own validation messages and
// the stack locations of three's JavaScript sources have no counterpart in the port.
export function normalizeConsole( lines ) {
	return lines
		.filter( ( [ level, text ] ) => level !== 'render' && ! /GPUValidationError|Render pipeline creation failed|renderPipeline_|Failed to execute|Failed to create texture/.test( text ) )
		.map( ( [ level, text ] ) => [ level, text.split( '\n' )[ 0 ].replace( / (?:"[^"]*\(\)" at )?"[^"]*:\d+"$/, '' ).replace( / \[Unknown location\]$/, '' ) ] );
}

if ( isMainThread ) {
	const args = process.argv.slice( 2 );
	const opt = ( name ) => {
		const i = args.indexOf( name );
		return i === - 1 ? null : args[ i + 1 ];
	};
	const corpus = JSON.parse( fs.readFileSync( ROOT + 'tests/tsl/corpus.json', 'utf8' ) );
	const golden = JSON.parse( zlib.gunzipSync( fs.readFileSync( ROOT + 'tests/tsl/golden.json.gz' ) ) );
	const T = ( i ) => golden.table[ i ];
	const only = opt( '--chunk' ) !== null ? [ Number( opt( '--chunk' ) ) ] : corpus.chunks.map( ( _, i ) => i );
	const exports = JSON.parse( fs.readFileSync( ROOT + 'tests/tsl/tsl-exports.json', 'utf8' ) );
	const results = new Map();
	let next = 0;
	const parallel = Math.max( 1, Math.min( os.cpus().length - 1, 8 ) );
	await new Promise( ( resolve ) => {
		let running = 0;
		const launch = () => {
			while ( running < parallel && next < only.length ) {
				const c = only[ next ++ ];
				running ++;
				const w = new Worker( new URL( import.meta.url ), { workerData: { chunk: corpus.chunks[ c ], exports } } );
				w.on( 'message', ( r ) => results.set( c, r ) );
				w.on( 'error', ( e ) => results.set( c, { crash: String( e && e.stack || e ) } ) );
				w.on( 'exit', () => {
					running --;
					if ( ! results.has( c ) ) results.set( c, { crash: 'worker exited' } );
					if ( next >= only.length && running === 0 ) resolve();
					else launch();
				} );
			}
		};
		launch();
	} );
	let total = 0, pass = 0, crashes = 0;
	const failures = [];
	const consoleFailures = [];
	for ( const c of only ) {
		const r = results.get( c );
		if ( r.crash ) {
			crashes ++;
			failures.push( { chunk: c, crash: r.crash } );
			continue;
		}
		r.results.forEach( ( got, k ) => {
			if ( opt( '--program' ) !== null && Number( opt( '--program' ) ) !== k ) return;
			const want = golden.results[ c ][ k ];
			if ( ! want ) return;
			total ++;
			let ok = true;
			for ( const key of [ 'wgsl', 'glsl', 'wgslLinear' ] ) {
				const w = want[ key ], g = got[ key ];
				const fields = w.error !== undefined ? [ 'error' ] : [ 'vertex', 'fragment' ];
				for ( const f of fields ) {
					const ws = T( w[ f ] ), gs = g[ f ];
					if ( ws !== gs ) {
						ok = false;
						failures.push( { chunk: c, program: k, key, field: f, want: ws, got: gs ?? ( g.error ? `<${ g.error }>` : '<missing>' ), code: corpus.chunks[ c ][ k ] } );
					}
				}
				const wc = JSON.stringify( normalizeConsole( w.console.map( ( [ l, i ] ) => [ l, T( i ) ] ) ) );
				const gc = JSON.stringify( normalizeConsole( g.console ) );
				if ( wc !== gc ) consoleFailures.push( { chunk: c, program: k, key, want: wc, got: gc } );
			}
			if ( ok ) pass ++;
		} );
	}
	fs.mkdirSync( ROOT + '.cache/tsl-check', { recursive: true } );
	fs.writeFileSync( ROOT + '.cache/tsl-check/failures.json', JSON.stringify( failures, null, 1 ) );
	fs.writeFileSync( ROOT + '.cache/tsl-check/console.json', JSON.stringify( consoleFailures, null, 1 ) );
	console.log( `${ pass } / ${ total } programs match; ${ crashes } chunks crashed; ${ consoleFailures.length } console mismatches` );
	if ( args.includes( '--diff' ) ) {
		for ( const f of failures.slice( 0, Number( opt( '--diff-count' ) ?? 3 ) ) ) {
			if ( f.crash ) {
				console.log( `chunk ${ f.chunk } crashed: ${ f.crash.slice( 0, 2000 ) }` );
				continue;
			}
			console.log( `--- chunk ${ f.chunk } program ${ f.program } ${ f.key }.${ f.field }\n${ f.code }` );
			const a = String( f.want ).split( '\n' ), b = String( f.got ).split( '\n' );
			let shown = 0;
			for ( let i = 0; i < Math.max( a.length, b.length ) && shown < 12; i ++ ) {
				if ( a[ i ] !== b[ i ] ) {
					console.log( `${ i + 1 }\n  want: ${ a[ i ] }\n  got:  ${ b[ i ] }` );
					shown ++;
				}
			}
		}
	}
	if ( args.includes( '--console' ) ) {
		for ( const f of consoleFailures.slice( 0, 5 ) ) console.log( `console chunk ${ f.chunk } program ${ f.program } ${ f.key }\n  want: ${ f.want.slice( 0, 600 ) }\n  got:  ${ f.got.slice( 0, 600 ) }` );
	}
} else {
	try {
		await runWorker();
	} catch ( e ) {
		parentPort.postMessage( { crash: ( e && ( e.stack || e.message ) ) || String( e ) } );
	}
}

async function runWorker() {
	const { chunk, exports } = workerData;
	// A document for the renderer's canvas and the TextureLoader's image ( nothing loads ).
	const element = ( name ) => ( {
		nodeName: name.toUpperCase(), style: {}, width: 300, height: 150,
		getContext() { return null; }, addEventListener() {}, removeEventListener() {}, setAttribute() {}, getAttribute() { return null; },
		getBoundingClientRect() { return { left: 0, top: 0, width: this.width, height: this.height }; }
	} );
	globalThis.document = { createElementNS: ( ns, name ) => element( name ), createElement: ( name ) => element( name ) };
	const require = createRequire( import.meta.url );
	const log = [];
	for ( const level of [ 'log', 'warn', 'error' ] ) {
		console[ level ] = ( ...args ) => {
			log.push( [ level, args.map( a => a instanceof Error ? a.message : typeof a === 'object' ? ( a && a.constructor ? a.constructor.name : String( a ) ) : String( a ) ).join( ' ' ) ] );
		};
	}
	const tsl = require( ROOT + '.cache/tsl-node/three_tsl.js' );
	tsl.tsl_init();
	const THREE = tsl.three_namespace();
	globalThis.__tslNamespace = tsl.tsl_namespace();
	const hook = `
		export async function resolve( specifier, context, next ) {
			if ( specifier === 'three/tsl' ) return { url: 'tsl:three-tsl', shortCircuit: true };
			return next( specifier, context );
		}
		export async function load( url, context, next ) {
			if ( url === 'tsl:three-tsl' ) return { format: 'module', shortCircuit: true, source: ${ JSON.stringify( exports.map( n => `export const ${ n } = globalThis.__tslNamespace[ ${ JSON.stringify( n ) } ];` ).join( '\n' ) ) } };
			return next( url, context );
		}`;
	register( 'data:text/javascript,' + encodeURIComponent( hook ), import.meta.url );
	// The editor's scene and renderers ( reference/three-js/tsl-corpus.html ).
	const { vec4 } = globalThis.__tslNamespace;
	const camera = new THREE.PerspectiveCamera( 70, 1, 0.1, 10 );
	camera.position.z = .72;
	camera.lookAt( 0, 0, 0 );
	const scene = new THREE.Scene();
	scene.background = new THREE.Color( 0x222222 );
	const renderer = new THREE.WebGPURenderer( { antialias: true } );
	renderer.setPixelRatio( 1 );
	renderer.setSize( 200, 200 );
	renderer.debug.diagnostics.keywords = true;
	const material = new THREE.NodeMaterial();
	material.fragmentNode = vec4( 0, 0, 0, 1 );
	const mesh = new THREE.Mesh( new THREE.PlaneGeometry( 1, 1 ), material );
	scene.add( mesh );
	const webGLRenderer = new THREE.WebGPURenderer( { forceWebGL: true } );
	webGLRenderer.debug.diagnostics.keywords = true;
	await renderer.init();
	renderer.render( scene, camera );
	const build = async ( code, output ) => {
		try {
			const AsyncFunction = async function () {}.constructor;
			const tslCode = `let output = null;\n${ code }\nreturn { output };`;
			const nodes = await ( new AsyncFunction( 'THREE', tslCode )( THREE ) );
			mesh.material.fragmentNode = nodes.output;
			mesh.material.needsUpdate = true;
			mesh.visible = false;
			const raw = output === 'WGSL' ? await renderer.debug.getShaderAsync( scene, camera, mesh ) : await webGLRenderer.debug.getShaderAsync( scene, camera, mesh );
			mesh.visible = true;
			return { vertex: raw.vertexShader, fragment: raw.fragmentShader };
		} catch ( e ) {
			mesh.visible = true;
			return { error: 'Error: ' + e.message };
		}
	};
	const results = [];
	for ( const code of chunk ) {
		const result = {};
		for ( const [ key, output, colorSpace ] of [ [ 'wgsl', 'WGSL', THREE.SRGBColorSpace ], [ 'glsl', 'GLSL ES 3.0', THREE.SRGBColorSpace ], [ 'wgslLinear', 'WGSL', THREE.LinearSRGBColorSpace ] ] ) {
			renderer.outputColorSpace = colorSpace;
			log.length = 0;
			result[ key ] = await build( code, output );
			try {
				renderer.render( scene, camera );
			} catch ( e ) {
				log.push( [ 'render', e.message ] );
			}
			result[ key ].console = log.splice( 0 );
		}
		renderer.outputColorSpace = THREE.SRGBColorSpace;
		results.push( result );
	}
	parentPort.postMessage( { results } );
}
