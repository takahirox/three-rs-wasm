// Records the pinned release's node system's output for the TSL corpus ( tests/tsl/corpus.json ):
// runs reference/three-js/tsl-corpus.html in Chrome, chunk by chunk ( each chunk one editor
// session, as the port replays it ), and writes tests/tsl/golden.json.gz with deduplicated text.
// Usage: node tools/tsl/golden.mjs [--probe] ( --probe regenerates the corpus' probe programs ).
import { chromium } from 'playwright';
import { spawn } from 'node:child_process';
import fs from 'node:fs';
import zlib from 'node:zlib';

const root = new URL( '../../', import.meta.url ).pathname;
const port = 8173;
const server = spawn( 'python3', [ 'tools/serve.py' ], { cwd: root, stdio: 'ignore' } );
await new Promise( r => setTimeout( r, 1500 ) );
const launch = () => chromium.launch( {
	executablePath: process.platform === 'darwin' ? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' : undefined,
	args: process.platform === 'linux'
		? [ '--enable-unsafe-webgpu', '--enable-features=Vulkan', '--use-angle=vulkan', '--use-vulkan=swiftshader', '--disable-vulkan-surface' ]
		: [ '--enable-unsafe-webgpu', '--enable-features=Vulkan', '--use-angle=default' ]
} );
let browser = await launch();
// A fresh browser when a page fails to come up ( a GPU process lost to an earlier program ).
const open = async () => {
	for ( let attempt = 0; ; attempt ++ ) {
		const log = [];
		try {
			const page = await browser.newPage();
			page.on( 'console', m => log.push( m.text() ) );
			page.on( 'pageerror', e => log.push( String( e ) ) );
			await page.goto( `http://127.0.0.1:${ port }/reference/three-js/tsl-corpus.html` );
			await page.waitForFunction( () => window.ready === true, null, { timeout: 60000 } );
			return page;
		} catch ( e ) {
			if ( attempt === 2 ) {
				console.error( log.slice( - 20 ).join( '\n' ) );
				throw e;
			}
			await browser.close().catch( () => {} );
			browser = await launch();
		}
	}
};
// A program that does not finish in 30 s is dropped from the corpus ( a rerun regenerates it ).
const run = ( page, code ) => Promise.race( [ page.evaluate( code => window.runProgram( code ), code ), new Promise( r => setTimeout( () => r( null ), 30000 ) ) ] );
const corpusPath = root + 'tests/tsl/corpus.json';
let corpus = fs.existsSync( corpusPath ) ? JSON.parse( fs.readFileSync( corpusPath, 'utf8' ) ) : { chunks: [] };
if ( process.argv.includes( '--probe' ) ) {
	const page = await open();
	const exports = await page.evaluate( () => window.tslExports );
	await page.close();
	const editor = fs.readFileSync( root + 'tests/tsl/programs/editor.js', 'utf8' );
	const handwritten = fs.readdirSync( root + 'tests/tsl/programs' ).filter( f => f !== 'editor.js' ).sort().map( f => fs.readFileSync( root + 'tests/tsl/programs/' + f, 'utf8' ) );
	// Each export called with the argument patterns below, in the module's export order.
	const patterns = [ null, '', 'a', 'v', 'a, a', 'v, v', 'a, a, a', 'v, v, v', 'v, a' ];
	const probes = [];
	for ( const [ name, type ] of exports ) {
		if ( type !== 'function' && type !== 'object' ) continue;
		for ( const args of patterns ) {
			if ( args === null && type !== 'object' ) continue;
			if ( args !== null && type !== 'function' ) continue;
			const imports = [ ...new Set( [ name, 'uv', 'vec3', 'vec4' ] ) ].join( ', ' );
			const call = args === null ? name : `${ name }( ${ args } )`.replace( '(  )', '()' );
			probes.push( `const { ${ imports } } = await import( 'three/tsl' );\nconst a = uv().x;\nconst v = vec3( uv(), 0.5 );\noutput = ${ call };\n` );
		}
	}
	const programs = [ editor, ...handwritten, ...probes ];
	const size = 40;
	corpus = { chunks: [] };
	for ( let i = 0; i < programs.length; i += size ) corpus.chunks.push( programs.slice( i, i + size ) );
	fs.writeFileSync( corpusPath, JSON.stringify( corpus, null, 1 ) );
}
// --add: append the hand-written programs not yet in the corpus as new chunks.
if ( process.argv.includes( '--add' ) ) {
	const known = new Set( corpus.chunks.flat() );
	const fresh = fs.readdirSync( root + 'tests/tsl/programs' ).filter( f => f !== 'editor.js' ).sort().map( f => fs.readFileSync( root + 'tests/tsl/programs/' + f, 'utf8' ) ).filter( code => ! known.has( code ) );
	for ( let i = 0; i < fresh.length; i += 40 ) corpus.chunks.push( fresh.slice( i, i + 40 ) );
	fs.writeFileSync( corpusPath, JSON.stringify( corpus, null, 1 ) );
	console.log( `${ fresh.length } programs added` );
}
// Only these chunks are recorded again ( --chunks a,b ); the rest keep their goldens.
const onlyChunks = process.argv.includes( '--chunks' ) ? process.argv[ process.argv.indexOf( '--chunks' ) + 1 ].split( ',' ).map( Number ) : null;
const previous = fs.existsSync( root + 'tests/tsl/golden.json.gz' ) ? JSON.parse( zlib.gunzipSync( fs.readFileSync( root + 'tests/tsl/golden.json.gz' ) ) ) : null;
// Text table: every distinct string once, results refer to it by index.
const table = [];
const index = new Map();
const intern = s => {
	if ( index.has( s ) ) return index.get( s );
	index.set( s, table.length );
	table.push( s );
	return table.length - 1;
};
const results = [];
const hung = [];
let done = 0;
for ( const [ chunkIndex, chunk ] of corpus.chunks.entries() ) {
	if ( onlyChunks !== null && ! onlyChunks.includes( chunkIndex ) && previous && previous.results[ chunkIndex ] ) {
		// Keep the recorded results, re-interned into the new table.
		const re = ( v ) => v.error !== undefined
			? { error: intern( previous.table[ v.error ] ), console: v.console.map( ( [ l, i ] ) => [ l, intern( previous.table[ i ] ) ] ) }
			: { vertex: intern( previous.table[ v.vertex ] ), fragment: intern( previous.table[ v.fragment ] ), console: v.console.map( ( [ l, i ] ) => [ l, intern( previous.table[ i ] ) ] ) };
		results.push( previous.results[ chunkIndex ].map( r => r && { wgsl: re( r.wgsl ), glsl: re( r.glsl ), wgslLinear: re( r.wgslLinear ) } ) );
		done += chunk.length;
		continue;
	}
	let page = await open();
	const chunkResults = [];
	for ( const code of chunk ) {
		const r = await run( page, code ).catch( () => null );
		if ( r === null ) {
			hung.push( code );
			await page.close().catch( () => {} );
			await browser.close().catch( () => {} );
			browser = await launch();
			page = await open();
			chunkResults.push( null );
			continue;
		}
		const packed = {};
		for ( const key of Object.keys( r ) ) {
			const v = r[ key ];
			packed[ key ] = v.error !== undefined
				? { error: intern( v.error ), console: v.console.map( ( [ l, m ] ) => [ l, intern( m ) ] ) }
				: { vertex: intern( v.vertex ), fragment: intern( v.fragment ), console: v.console.map( ( [ l, m ] ) => [ l, intern( m ) ] ) };
		}
		chunkResults.push( packed );
		done ++;
	}
	results.push( chunkResults );
	await page.close();
	process.stdout.write( `\r${ done } programs` );
}
if ( hung.length > 0 ) {
	corpus.chunks = corpus.chunks.map( chunk => chunk.filter( code => ! hung.includes( code ) ) ).filter( chunk => chunk.length > 0 );
	fs.writeFileSync( corpusPath, JSON.stringify( corpus, null, 1 ) );
	console.log( `\n${ hung.length } programs dropped; run again for their goldens` );
}
fs.writeFileSync( root + 'tests/tsl/golden.json.gz', zlib.gzipSync( JSON.stringify( { table, results } ) ) );
console.log( `\n${ table.length } distinct strings` );
await browser.close();
server.kill();
