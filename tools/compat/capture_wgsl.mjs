// The WGSL each original example compiles, for moving the baseline to a new
// release: opens <tree>/examples/<id>.html ( a three.js checkout served from
// <root> ) in Chrome, records every createShaderModule() source for a few
// seconds and writes { id: [ wgsl, … ] } to <out>, the header's release line
// dropped. Comparing two releases' captures lists the examples whose generated
// shaders changed.
//   node tools/compat/capture_wgsl.mjs <root> <tree> <out.json> <id> …
import { chromium } from 'playwright';
import { spawn } from 'node:child_process';
import fs from 'node:fs';

const [ root, tree, out, ...ids ] = process.argv.slice( 2 );
if ( ! ids.length ) throw new Error( 'usage: capture_wgsl.mjs <root> <tree> <out.json> <id> …' );
const port = 28190 + Math.floor( Math.random() * 100 );
const server = spawn( 'python3', [ '-m', 'http.server', String( port ), '--bind', '127.0.0.1', '--directory', root ], { stdio: 'ignore' } );
await new Promise( r => setTimeout( r, 1000 ) );
const browser = await chromium.launch( {
	executablePath: process.platform === 'darwin' ? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome' : undefined,
	args: [ '--enable-unsafe-webgpu' ]
} );
const result = fs.existsSync( out ) ? JSON.parse( fs.readFileSync( out, 'utf8' ) ) : {};
for ( const id of ids ) {
	if ( result[ id ] ) continue;
	const page = await browser.newPage( { viewport: { width: 800, height: 600 } } );
	await page.addInitScript( () => {
		window.__wgsl = [];
		if ( ! window.GPUDevice ) return;
		const create = GPUDevice.prototype.createShaderModule;
		GPUDevice.prototype.createShaderModule = function ( descriptor ) {
			window.__wgsl.push( descriptor.code );
			return create.call( this, descriptor );
		};
	} );
	try {
		await page.goto( `http://127.0.0.1:${ port }/${ tree }/examples/${ id }.html`, { timeout: 60000 } );
		// Shaders compiled after load ( async pipelines, the first frames, lazily built passes ).
		// Until some exist and none have been added for three seconds ( at most 45 s ).
		let count = - 1, still = 0;
		for ( let i = 0; i < 45 && still < 3; i ++ ) {
			await page.waitForTimeout( 1000 );
			const now = await page.evaluate( () => window.__wgsl.length );
			still = now > 0 && now === count ? still + 1 : 0;
			count = now;
		}
		const shaders = await page.evaluate( () => window.__wgsl );
		result[ id ] = [ ...new Set( shaders.map( s => s.replace( /^\/\/ Three\.js r\d+ - Node System\n/, '' ) ) ) ].sort();
	} catch ( e ) {
		result[ id ] = { error: String( e ).slice( 0, 300 ) };
	}
	await page.close();
	fs.writeFileSync( out, JSON.stringify( result ) );
	console.log( id, Array.isArray( result[ id ] ) ? result[ id ].length : 'error' );
}
await browser.close();
server.kill();
