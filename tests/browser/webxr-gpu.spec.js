// webgpu_xr_cubes / webgpu_xr_rollercoaster / webgpu_xr_shadows: the gallery's
// ports against the pinned r186 pages ( reference/three-js/webxr-gpu.html ) on
// the same fake WebXR device with the WebXR/WebGPU binding
// ( tests/browser/fake-xr.js ). Both pages run the same window frames, enter
// the session through their XRButton / VRButton, run the same session frames
// and controller input, and leave it. Compared:
// - the canvas before and after the session;
// - both views of the projection layer after the session's frames ( and,
//   for the cubes, while the trigger selects );
// - the button's label and style through the session;
// - the adapter request ( xrCompatible ) and the session request.
// The port must create no GPU objects in steady session frames.
import { test, expect } from '@playwright/test';
import { PNG } from 'pngjs';
import { writeFileSync } from 'node:fs';

const RANDOM = () => {

	let seed = 1;
	window.galleryRandom = () => ( ( seed = ( Math.imul( seed, 1664525 ) + 1013904223 ) >>> 0 ) / 4294967296 );

};

const GPU_CREATES = () => {

	window.gpuCreates = 0;
	for ( const name of [ 'createBuffer', 'createTexture', 'createBindGroup', 'createRenderPipeline', 'createComputePipeline', 'createShaderModule', 'createSampler', 'createBindGroupLayout', 'createPipelineLayout', 'createQuerySet' ] ) {

		const original = GPUDevice.prototype[ name ];
		GPUDevice.prototype[ name ] = function ( ...args ) {

			window.gpuCreates ++;
			return original.apply( this, args );

		};

	}

};

// Per view: at most this fraction of pixels may differ by more than 2 in a
// channel, and the mean channel difference stays below MEAN ( docs/webxr-gpu.md ).
const FRACTION = 0.001;
const MEAN = 0.5;

const cases = {
	webgpu_xr_cubes: { button: 'XRButton', start: 'START XR', stop: 'STOP XR', model: true },
	webgpu_xr_rollercoaster: { button: 'VRButton', start: 'ENTER VR', stop: 'EXIT VR', model: false },
	webgpu_xr_shadows: { button: 'VRButton', start: 'ENTER VR', stop: 'EXIT VR', model: false },
	// An AR-capable device: XRButton starts immersive-ar on an alpha-blend display.
	'webgpu_xr_cubes immersive-ar': { id: 'webgpu_xr_cubes', ar: true, button: 'XRButton', start: 'START XR', stop: 'STOP XR', model: true }
};

function compare( a, b ) {

	expect( [ a.width, a.height ] ).toEqual( [ b.width, b.height ] );
	let bad = 0, sum = 0;
	for ( let p = 0; p < a.data.length; p += 4 ) {

		let worst = 0;
		for ( let c = 0; c < 4; c ++ ) worst = Math.max( worst, Math.abs( a.data[ p + c ] - b.data[ p + c ] ) );
		if ( worst > 2 ) bad ++;
		sum += worst;

	}

	const pixels = a.width * a.height;
	return { fraction: bad / pixels, mean: sum / pixels };

}

async function eyes( page ) {

	const data = Buffer.from( await page.evaluate( () => fakeXR.readEyes() ), 'base64' );
	const size = await page.evaluate( () => fakeXR.EYE );
	return [ 0, 1 ].map( e => {

		const png = new PNG( { width: size, height: size } );
		data.copy( png.data, 0, e * size * size * 4, ( e + 1 ) * size * size * 4 );
		return png;

	} );

}

async function run( page, id, spec, runtime ) {

	const errors = [];
	page.on( 'pageerror', e => errors.push( String( e ) ) );
	const warnings = [];
	page.on( 'console', m => {

		if ( m.type() === 'warning' && m.text().includes( 'XRManager' ) ) warnings.push( m.text() );

	} );
	// fake-xr.js steps requestAnimationFrame: waits poll on a timer.
	if ( spec.ar ) await page.addInitScript( () => window.fakeXROptions = { ar: true } );
	await page.addInitScript( { path: 'tests/browser/fake-xr.js' } );
	if ( runtime === 'port' ) {

		await page.addInitScript( RANDOM );
		await page.addInitScript( GPU_CREATES );

	}

	await page.setViewportSize( { width: 800, height: 600 } );
	await page.goto( runtime === 'reference' ? `/reference/three-js/webxr-gpu.html?id=${ id }` : `/web/examples/${ id }.html` );
	const button = page.locator( `#${ spec.button }` );
	await expect( button ).toHaveText( spec.start, { timeout: 90000 } );
	await page.waitForFunction( runtime => runtime === 'reference' ? window.fixtureScene !== undefined && window.fixtureRenderer !== undefined : window.app !== undefined, runtime, { polling: 100 } );
	const style = () => button.evaluate( b => {

		const s = getComputedStyle( b );
		return [ 'position', 'bottom', 'left', 'width', 'padding', 'border', 'borderRadius', 'background', 'color', 'font', 'textAlign', 'opacity', 'zIndex', 'cursor' ].map( k => s[ k ] );

	} );
	const out = { errors, warnings, style: await style() };
	await page.addStyleTag( { content: '#info,#notice,#XRButton,#VRButton{visibility:hidden!important}' } );
	await page.evaluate( () => fakeXR.windowFrames( 3 ) );
	const canvas = page.locator( 'canvas' ).first();
	out.before = PNG.sync.read( await canvas.screenshot() );
	await button.evaluate( b => b.click() );
	for ( let i = 0; i < 300 && ! ( await page.evaluate( () => fakeXR.state.session !== null && fakeXR.state.frames >= 0 && fakeXR.state.layer !== null ) ); i ++ ) await page.waitForTimeout( 50 );
	await expect( button ).toHaveText( spec.stop );
	if ( spec.model ) {

		// The controller model loads without session frames.
		await page.waitForFunction( runtime => runtime === 'reference'
			? window.fixtureScene.children.some( c => c.children.some( m => m.motionController && m.children.length > 0 ) )
			: window.app.controller_model_ready( 0 ), runtime, { timeout: 60000, polling: 100 } );

	}

	await page.evaluate( () => fakeXR.step( 30 ) );
	out.views = await eyes( page );
	if ( spec.model ) {

		// The trigger: selectstart, frames with the trigger pressed, selectend.
		await page.evaluate( () => {

			fakeXR.setTrigger( 1 );
			fakeXR.select( 'selectstart' );

		} );
		await page.evaluate( () => fakeXR.step( 6 ) );
		out.selecting = await eyes( page );
		await page.evaluate( () => {

			fakeXR.setTrigger( 0 );
			fakeXR.select( 'selectend' );
			fakeXR.select( 'select' );

		} );
		await page.evaluate( () => fakeXR.step( 12 ) );
		out.released = await eyes( page );

	}

	if ( runtime === 'port' ) {

		await page.evaluate( () => fakeXR.step( 5 ) );
		const before = await page.evaluate( () => window.gpuCreates );
		await page.evaluate( () => fakeXR.step( 10 ) );
		out.creates = await page.evaluate( () => window.gpuCreates ) - before;
		out.xrFrames = Number( await canvas.getAttribute( 'data-xr-frames' ) );

	} else {

		await page.evaluate( () => fakeXR.step( 15 ) );

	}

	out.final = await eyes( page );
	out.events = await page.evaluate( () => fakeXR.state.events );
	await page.evaluate( () => fakeXR.end() );
	await expect( button ).toHaveText( spec.start );
	await page.evaluate( () => fakeXR.windowFrames( 3 ) );
	out.after = PNG.sync.read( await canvas.screenshot() );
	return out;

}

for ( const [ name, spec ] of Object.entries( cases ) ) {

	const id = spec.id ?? name;
	test( `${ name } matches the original on the WebGPU XR path`, async ( { browser }, info ) => {

		test.setTimeout( 600000 );
		const out = {};
		for ( const runtime of [ 'reference', 'port' ] ) {

			const page = await browser.newPage();
			out[ runtime ] = await run( page, id, spec, runtime );
			await page.close();

		}

		const { reference, port } = out;
		expect( reference.errors ).toEqual( [] );
		expect( port.errors ).toEqual( [] );
		expect( port.warnings ).toEqual( reference.warnings );
		expect( port.style ).toEqual( reference.style );
		expect( port.events ).toEqual( reference.events );
		expect( port.events[ 0 ] ).toEqual( { type: 'requestAdapter', xrCompatible: true } );
		const results = {};
		const images = [ [ 'before', reference.before, port.before ], [ 'after', reference.after, port.after ] ];
		for ( const key of [ 'views', 'selecting', 'released', 'final' ] ) {

			if ( ! reference[ key ] ) continue;
			for ( const e of [ 0, 1 ] ) images.push( [ `${ key }-${ e }`, reference[ key ][ e ], port[ key ][ e ] ] );

		}

		for ( const [ name, a, b ] of images ) {

			writeFileSync( info.outputPath( `${ name }-reference.png` ), PNG.sync.write( a ) );
			writeFileSync( info.outputPath( `${ name }-port.png` ), PNG.sync.write( b ) );
			results[ name ] = compare( b, a );

		}

		writeFileSync( info.outputPath( 'results.json' ), JSON.stringify( { events: port.events, results }, null, 1 ) );
		for ( const [ name, r ] of Object.entries( results ) ) {

			expect( r.fraction, `${ name } fraction` ).toBeLessThanOrEqual( FRACTION );
			expect( r.mean, `${ name } mean` ).toBeLessThanOrEqual( MEAN );

		}

		expect( port.xrFrames ).toBeGreaterThan( 30 );
		expect( port.creates ).toBe( 0 );

	} );

}
