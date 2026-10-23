// webgpu_tsl_editor: the gallery's port ( the three-tsl crate ) against the
// original page on the pinned r186 sources ( reference/three-js/tsl-editor.html ).
// Both pages run the same TSL with the same option changes; every shader text
// must match, the previews must agree, and the port's steady frames must
// create no GPU objects. The clock is frozen in both ( time stays 0 ).
import { test, expect } from '@playwright/test';
import fs from 'node:fs';
import { PNG } from 'pngjs';

const programs = [ 'fn-layout.js', 'if-else.js', 'struct.js', 'noise.js', 'texture-uv.js', 'color-adjust.js', 'errors-throw.js' ].map( f => fs.readFileSync( new URL( `../tsl/programs/${ f }`, import.meta.url ), 'utf8' ) );
const STATIC = `const { vec4, uv, vec3 } = await import( 'three/tsl' );\noutput = vec4( vec3( uv(), 0.25 ), 1 );\n`;

const pages = {
	reference: '/reference/three-js/tsl-editor.html',
	port: '/web/examples/webgpu_tsl_editor.html'
};

async function open( page, url ) {

	await page.addInitScript( () => {

		performance.now = () => 1000;
		window.__gpuCreates = 0;
		for ( const name of [ 'createBuffer', 'createTexture', 'createBindGroup', 'createRenderPipeline', 'createShaderModule', 'createSampler', 'createBindGroupLayout', 'createPipelineLayout' ] ) {

			const original = GPUDevice.prototype[ name ];
			GPUDevice.prototype[ name ] = function ( ...args ) {

				window.__gpuCreates ++;
				return original.apply( this, args );

			};

		}

	} );
	await page.setViewportSize( { width: 1280, height: 720 } );
	await page.goto( url );
	await page.waitForFunction( () => window.monaco && monaco.editor.getEditors().length === 2 && monaco.editor.getEditors()[ 1 ].getValue().length > 0, null, { timeout: 120000 } );

}

const result = ( page ) => page.evaluate( () => monaco.editor.getEditors()[ 1 ].getValue() );

// The editor rebuilds 1 s after an edit; wait for the result to settle.
async function setCode( page, code ) {

	await page.evaluate( ( code ) => monaco.editor.getEditors()[ 0 ].setValue( code ), code );
	await page.waitForTimeout( 1500 );
	let previous = await result( page );
	for ( let i = 0; i < 20; i ++ ) {

		await page.waitForTimeout( 250 );
		const now = await result( page );
		if ( now === previous ) return now;
		previous = now;

	}

	return previous;

}

// A lil-gui option, as the page's controls set it.
async function setOption( page, name, value ) {

	await page.evaluate( ( [ name, value ] ) => {

		for ( const controller of document.querySelectorAll( '.lil-gui .controller' ) ) {

			if ( controller.querySelector( '.name' ).textContent !== name ) continue;
			const select = controller.querySelector( 'select' );
			const index = [ ...select.options ].findIndex( o => o.textContent === value );
			select.selectedIndex = index;
			select.dispatchEvent( new Event( 'change' ) );

		}

	}, [ name, value ] );
	await page.waitForTimeout( 1200 );

}

async function run( page, url ) {

	await open( page, url );
	const texts = [ await result( page ) ];
	for ( const code of programs ) texts.push( await setCode( page, code ) );
	await setOption( page, 'shader', 'vertex' );
	texts.push( await result( page ) );
	await setOption( page, 'shader', 'fragment' );
	await setOption( page, 'output', 'GLSL ES 3.0' );
	texts.push( await result( page ) );
	await setOption( page, 'output', 'WGSL' );
	await setOption( page, 'outputColorSpace', 'srgb-linear' );
	texts.push( await result( page ) );
	await setOption( page, 'outputColorSpace', 'srgb' );
	texts.push( await setCode( page, STATIC ) );
	await page.waitForTimeout( 500 );
	const preview = PNG.sync.read( await page.locator( '#renderer canvas' ).screenshot() );
	return { texts, preview };

}

test( 'webgpu_tsl_editor matches the original', async ( { browser } ) => {

	test.setTimeout( 600000 );
	const out = {};
	for ( const [ name, url ] of Object.entries( pages ) ) {

		const page = await browser.newPage();
		out[ name ] = await run( page, url );
		if ( name === 'port' ) {

			// Steady frames create no GPU objects.
			await page.waitForTimeout( 1000 );
			const before = await page.evaluate( () => window.__gpuCreates );
			await page.waitForTimeout( 1000 );
			expect( await page.evaluate( () => window.__gpuCreates ) ).toBe( before );
			expect( Number( await page.locator( '#renderer canvas' ).getAttribute( 'data-frames' ) ) ).toBeGreaterThan( 10 );

		}

		await page.close();

	}

	const { reference, port } = out;
	expect( port.texts.length ).toBe( reference.texts.length );
	port.texts.forEach( ( text, i ) => expect( text, `result ${ i }` ).toBe( reference.texts[ i ] ) );
	const a = port.preview, b = reference.preview;
	expect( [ a.width, a.height ] ).toEqual( [ b.width, b.height ] );
	let bad = 0;
	for ( let p = 0; p < a.data.length; p += 4 ) {

		let worst = 0;
		for ( let c = 0; c < 3; c ++ ) worst = Math.max( worst, Math.abs( a.data[ p + c ] - b.data[ p + c ] ) );
		if ( worst > 2 ) bad ++;

	}

	expect( bad / ( a.width * a.height ) ).toBeLessThanOrEqual( 0.001 );

} );
