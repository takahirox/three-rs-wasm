// The gallery's ports show their original's UI: each lil-gui panel's DOM,
// the WebGPU Inspector's parameter panel, stats.js and the page's #info equal
// what the pinned r186 page builds ( web/gallery/ui/specs.json, recorded by
// tools/gallery/ui/extract.mjs at the same 1280×720 viewport, and pages.json ).
// lil-gui's generated ids and the values of controllers that listen to a
// changing property are normalized. Controllers without a port parameter are
// disabled and reported in each test's unbound.json.
import { test, expect } from '@playwright/test';
import { readFileSync, writeFileSync } from 'node:fs';

const specs = JSON.parse( readFileSync( 'web/gallery/ui/specs.json', 'utf8' ) );
const pages = JSON.parse( readFileSync( 'web/gallery/ui/pages.json', 'utf8' ) );
const catalog = JSON.parse( readFileSync( 'web/gallery/catalog.json', 'utf8' ) );
const ported = new Set( catalog.examples.filter( e => e.port ).map( e => e.id ) );

// Normalized markup: generated ids, listened values and the gallery's marks.
function normalize( html, listen ) {

	const template = document.createElement( 'template' );
	template.innerHTML = html;
	const root = template.content.firstElementChild;
	if ( ! root ) return '';
	let n = 0;
	const ids = new Map();
	for ( const el of root.querySelectorAll( '[id],[aria-labelledby],[for]' ) ) {

		for ( const attr of [ 'id', 'aria-labelledby', 'for' ] ) {

			const v = el.getAttribute( attr );
			if ( v === null ) continue;
			if ( ! ids.has( v ) ) ids.set( v, `id${ n ++ }` );
			el.setAttribute( attr, ids.get( v ) );

		}

	}

	// Controllers the port has no parameter for are disabled ( and titled ).
	for ( const el of root.querySelectorAll( '[data-unbound]' ) ) {

		el.removeAttribute( 'data-unbound' );
		el.removeAttribute( 'title' );
		el.classList.remove( 'disabled' );
		for ( const input of el.querySelectorAll( '[disabled]' ) ) input.removeAttribute( 'disabled' );

	}
	// Readouts of a running count ( the path tracer's samples ).
	for ( const el of root.querySelectorAll( '.gui-render' ) ) el.textContent = '';
	const controllers = [ ...root.querySelectorAll( '.controller' ) ];
	listen.forEach( ( on, i ) => {

		if ( ! on || ! controllers[ i ] ) return;
		for ( const el of controllers[ i ].querySelectorAll( 'input,select,.fill,.display' ) ) {

			el.removeAttribute( 'value' );
			el.removeAttribute( 'style' );
			if ( el.tagName === 'INPUT' ) el.value = '';

		}

	} );
	return root.outerHTML.replace( /\s+/g, ' ' );

}

// The first difference of two strings, with context.
function around( a, b ) {

	let i = 0;
	while ( i < a.length && a[ i ] === b[ i ] ) i ++;
	return a === b ? a : `at ${ i }: …${ a.slice( Math.max( 0, i - 120 ), i + 160 ) }`;

}

const listened = nodes => nodes.flatMap( n => n.kind === 'folder' ? listened( n.children ) : [ !! n.listen ] );

const ids = Object.keys( specs ).filter( id => ported.has( id ) && ! specs[ id ].error && ( specs[ id ].guis.length || specs[ id ].inspectors.length || specs[ id ].stats.length ) );

for ( const id of ids ) test( `original UI: ${ id }`, async ( { page }, info ) => {

	test.setTimeout( 120000 );
	const spec = specs[ id ];
	const errors = [];
	page.on( 'pageerror', e => errors.push( String( e ) ) );
	await page.setViewportSize( { width: 1280, height: 720 } );
	await page.goto( `/web/gallery/examples/${ id }.html?ui=1` );
	await page.waitForFunction( () => document.body.dataset.ui === 'ready' || document.body.dataset.status === 'error', null, { timeout: 90000, polling: 200 } );
	expect( await page.evaluate( () => document.body.dataset.status ) ).not.toBe( 'error' );
	expect( await page.evaluate( () => document.body.dataset.uiError ?? null ), 'UI installation' ).toBe( null );
	const found = await page.evaluate( ( { normalizeSource, listens } ) => {

		const normalize = new Function( `return ${ normalizeSource }` )();
		const roots = [ ...document.querySelectorAll( '.lil-gui.root' ) ];
		return {
			guis: roots.map( ( r, i ) => normalize( r.outerHTML, listens[ i ] ?? [] ) ),
			inspectors: ( window.galleryInspectors ?? [] ).map( i => i.parameters.content.outerHTML ),
			stats: [ ...document.body.children ].filter( e => e.style.zIndex === '10000' && e.querySelector( 'canvas' ) ).map( e => ( {
				style: e.getAttribute( 'style' ),
				shown: [ ...e.children ].findIndex( c => c.style.display !== 'none' )
			} ) ),
			info: document.querySelector( '#info' )?.textContent.replace( /\s+/g, ' ' ).trim() ?? null,
			unbound: document.body.dataset.uiUnbound
		};

	}, { normalizeSource: normalize.toString(), listens: spec.guis.map( g => listened( g.children ) ) } );

	expect( found.guis.length, 'lil-gui panels' ).toBe( spec.guis.length );
	const expected = await page.evaluate( ( { normalizeSource, guis } ) => {

		const normalize = new Function( `return ${ normalizeSource }` )();
		return guis.map( g => normalize( g.dom, g.listen ) );

	}, { normalizeSource: normalize.toString(), guis: spec.guis.map( g => ( { dom: g.dom, listen: listened( g.children ) } ) ) } );
	found.guis.forEach( ( html, i ) => expect( around( html, expected[ i ] ), `lil-gui ${ i }` ).toBe( around( expected[ i ], html ) ) );
	expect( found.inspectors.length, 'inspectors' ).toBe( spec.inspectors.length );
	found.inspectors.forEach( ( html, i ) => {

		const clean = h => h.replace( /data-list-id="[^"]*"/g, '' ).replace( /\s+/g, ' ' );
		const a = clean( html ), b = clean( spec.inspectors[ i ].dom );
		expect( around( a, b ), `inspector ${ i }` ).toBe( around( b, a ) );

	} );
	expect( found.stats.map( s => s.style ), 'stats' ).toEqual( spec.stats.map( s => s.place.style ) );
	found.stats.forEach( ( s, i ) => expect( s.shown, `stats panel ${ i }` ).toBe( spec.stats[ i ].shows.at( - 1 ) ?? 0 ) );
	if ( pages[ id ]?.info && found.info !== null ) {

		const text = await page.evaluate( html => {

			// Without the elements the port made itself ( see installPage ).
			const t = document.createElement( 'template' );
			t.innerHTML = html;
			const info = t.content.firstElementChild;
			for ( const el of info.querySelectorAll( '[id]' ) ) if ( document.getElementById( el.id ) && ! document.querySelector( '#info' ).contains( document.getElementById( el.id ) ) ) el.remove();
			return info.textContent;

		}, pages[ id ].info );
		expect( found.info.replace( /\s+/g, '' ), '#info' ).toBe( text.replace( /\s+/g, '' ) );

	}

	// Per test: a failure restarts the worker.
	if ( found.unbound ) writeFileSync( info.outputPath( 'unbound.json' ), JSON.stringify( { [ id ]: found.unbound.split( '|' ) } ) );
	expect( errors ).toEqual( [] );

} );
